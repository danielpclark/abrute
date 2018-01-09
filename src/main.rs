// Copyright 2017-2018 Daniel P. Clark & other abrute Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![feature(try_from)]
#![feature(libc)]
extern crate digits;
extern crate rayon;
use digits::Digits;
mod web;
mod model;
use model::report_data::ReportData;
use model::work_load::WorkLoad;
mod resume;
mod result;
mod reporter;
mod process_input;
use process_input::*;
mod validators;
use validators::*;
mod core;
use core::*;
use std::io::{self, Write}; 
use result::Error;
use std::error::Error as StdError;
#[macro_use]
extern crate clap;
use clap::{Arg, App};
extern crate serde_json;
use std::time::SystemTime;
use std::sync::{Arc, Mutex};
extern crate num_cpus;
extern crate tiny_http;
#[macro_use]
extern crate lazy_static;

use std::thread;
extern crate libc;
use libc::pthread_cancel;
use std::os::unix::thread::{
  RawPthread,
  JoinHandleExt
};

use std::sync::atomic::{
  AtomicUsize,
  AtomicBool,
  ATOMIC_USIZE_INIT,
  ATOMIC_BOOL_INIT
};

static ITERATIONS: AtomicUsize = ATOMIC_USIZE_INIT;
static SUCCESS: AtomicBool = ATOMIC_BOOL_INIT;

fn run_app() -> Result<(), Error> {
  let matches = App::new("abrute - AES Brute Force File Decryptor").
    version(&format!("v{}", crate_version!())[..]).
    version_short("v").
    author(crate_authors!("\n")).
    usage("abrute <RANGE> <CHARACTERS> [OPTIONS] -- <TARGET>").
    arg(Arg::with_name("RANGE").
          required(true).
          index(1)
    ).
    arg(Arg::with_name("CHARACTERS").
          required(true).
          index(2)
    ).
    arg(Arg::with_name("adjacent").
          short("a").
          long("adjacent").
          takes_value(true)
    ).
    arg(Arg::with_name("start").
          short("s").
          long("start").
          takes_value(true)
    ).
    arg(Arg::with_name("zip").
          short("z").
          long("zip").
          takes_value(false)
    ).
    arg(Arg::with_name("chunk").
          short("c").
          long("chunk").
          takes_value(true)
    ).
    arg(Arg::with_name("cluster").
          long("cluster").
          takes_value(true)
    ).
    arg(Arg::with_name("reporter").
          short("r").
          long("reporter").
          takes_value(true)
    ).
    arg(Arg::with_name("TARGET").
          required(true).
          last(true)
    ).
    template("\
-------------------------------------------------------------
       {bin} {version}
-------------------------------------------------------------
           By: {author}


  USAGE:\tabrute <RANGE> <CHARACTERS> [OPTIONS] -- <TARGET>

   <RANGE>         Single digit or a range 4:6 for password length.
   <CHARACTERS>    Characters to use in password attempt. Don't use quotes
                   unless they may be in the password. Backslash may escape
                   characters such as space.
   -a, --adjacent  Set a limit for allowed adjacent characters. Zero will
                   not allow any characters of the same kind to neighbor
                   in the attempts.
   -s, --start     Starting character sequence to begin with.
   -z, --zip       Use `unzip` decryption instead of `aescrypt`.
   -c, --chunk     Workload chunk size per core before status update.
                   Defaults to 32.
   --cluster       Takes an offset and cluster size such as 1:4 for the
                   first system in a cluster of 4.  Helps different systems
                   split the workload without trying the same passwords.
   -r, --reporter  Use `spinner`, or `benchmark` to use a different command
                   line reporter.
   <TARGET>        Target file to decrypt.  The target must be preceeded
                   by a double dash: -- target.aes
   -h, --help      Prints help information.
   -v, --version   Prints version information.

-------------------------------------------------------------
USE OF THIS BINARY FALLS UNDER THE MIT LICENSE  (c) 2017-2018").
    get_matches();

  if matches.is_present("zip") {
    validate_unzip_executable()?;
  } else {
    validate_aescrpyt_executable()?;
  }
    
  let (min, max) = derive_min_max(
    matches.value_of("RANGE").unwrap()
  )?;
  
  validate_start_string(&matches, max)?;

  let mapping = derive_character_base(
    matches.value_of("CHARACTERS").unwrap()
  );
  let resume_key_chars = mapping_to_characters(&mapping);
  let mut sequencer = Digits::new(
    mapping,
    matches.value_of("start").unwrap_or("").to_string()
  );
  sequencer.zero_fill(min as usize);

  let target = matches.value_of("TARGET").unwrap_or("");
  let adjacent = matches.value_of("adjacent");

  validate_and_prep_sequencer_adjacent(&mut sequencer, adjacent)?;
  validate_file_exists(&target)?;

  let chunk = matches.value_of("chunk");
  if matches.is_present("chunk") {
    validate_chunk_input(&chunk.unwrap()[..])?;
  }

  let mut cluster_step: Option<usize> = None;
  if matches.is_present("cluster") {
    let (offset, step) = derive_cluster(
      matches.value_of("cluster").unwrap()
    )?;
    cluster_step = Some(step);
    let additive = sequencer.gen(offset as u64).pred_till_zero();
    sequencer.mut_add(additive);
  }

  let reporter = 
    verify_reporter_name(
      matches.
      value_of("reporter").
      unwrap_or("ticker").
      to_string()
    );

  // JSON URI
  println!("JSON endpoint available on Port 3838");
  // END JSON URI

  // Begin Resume Feature
  let starting = sequencer.to_s();
  use ::resume::{ResumeKey,ResumeFile};
  let cli_key = ResumeKey::new(
    resume_key_chars.clone(),
    adjacent.map(str::to_string),
    sequencer,
    target.to_string(),
  );
  let latest = cli_key.latest(ResumeFile::load());
  let sequencer = latest.start;
  if starting != sequencer.to_s() {
    println!("Resuming from last save point: {}", sequencer.to_s());
  }
  // End Resume Feature
  
  // DATA for JSON web end point
  let reporter_handler = ReportData {
    cores: num_cpus::get() as u8,
    chunk: chunk.
      clone().
      unwrap_or("").
      parse::<usize>().
      unwrap_or(32),
    cluster: {
      if matches.is_present("cluster") {
        Some(
          derive_cluster(
            matches.value_of("cluster").unwrap()
          ).ok().unwrap()
        )
      } else { None }
    },
    character_set: resume_key_chars.clone(),
    start_time: SystemTime::now(),
    start_at: sequencer.to_s(),
    adjacent_limit: adjacent.map(|ref s|
      u8::from_str_radix(&s,10).ok().unwrap()
    ),
    five_min_progress: Arc::new(
      Mutex::new((0, "".to_string()))
    ),
  };

  let web_reporter = reporter_handler.clone();
  let web_runner = thread::spawn(move || {
    web::host_data(&web_reporter)
  });

  let work_load = WorkLoad(
    resume_key_chars,
    max,
    sequencer,
    target.to_string(),
    adjacent.map(str::to_string),
    chunk.map(str::to_string),
    cluster_step,
    reporter_handler,
    reporter
  );

  let mtchs = matches.clone();

  let crypt_runner = thread::spawn(move || {
    if mtchs.is_present("zip") {
      unzip_core_loop(work_load)
    } else {
      aescrypt_core_loop(work_load)
    }
  });

  let wr: RawPthread = web_runner.as_pthread_t();

  let cr = crypt_runner.join().unwrap();

  unsafe {
    pthread_cancel(wr);
  }

  cr
}

fn main() {
  ::std::process::exit(
    match run_app() {
      Ok(_) => {
        println!("Exiting…");
        0
      },
      Err(err) => {
        writeln!(
          io::stderr(),
          "Error: {}\n{}\n\nUse `abrute -h` for a help menu.",
          err,
          err.description()
        ).unwrap();
        1
      }
    }
  );
}
