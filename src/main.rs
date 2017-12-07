// Copyright 2017 Daniel P. Clark & other abrute Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![feature(try_from)]
extern crate digits;
extern crate rayon;
use digits::Digits;
use std::io::{self, Write}; 
mod resume;
mod result;
use result::Error;
use std::error::Error as StdError;
mod process_input;
use process_input::*;
mod validators;
use validators::*;
mod core;
use core::*;
#[macro_use]
extern crate clap;
use clap::{Arg, App};

pub struct WorkLoad(
  pub String,        // characters: String,
  pub usize,         // max: usize,
  pub Digits,        // mut sequencer: Digits,
  pub String,        // target: &str,
  pub Option<String> // adj: Option<&str>
);

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
   <TARGET>        Target file to decrypt.  The target must be preceeded
                   by a double dash: -- target.aes
   -h, --help      Prints help information.
   -v, --version   Prints version information.

-------------------------------------------------------------
USE OF THIS BINARY FALLS UNDER THE MIT LICENSE       (c) 2017").
    get_matches();

  if matches.is_present("zip") {
    validate_unzip_executable()?;
  } else {
    validate_aescrpyt_executable()?;
  }
    
  let (min, max) = derive_min_max(matches.value_of("RANGE").unwrap())?;
  
  validate_start_string(&matches, max)?;

  let mapping = derive_character_base(matches.value_of("CHARACTERS").unwrap());
  let resume_key_chars = mapping_to_characters(&mapping);
  let mut sequencer = Digits::new(mapping, matches.value_of("start").unwrap_or("").to_string());
  sequencer.zero_fill(min as usize);

  let target = matches.value_of("TARGET").unwrap_or("");
  let adjacent = matches.value_of("adjacent");

  validate_and_prep_sequencer_adjacent(&mut sequencer, adjacent)?;
  validate_file_exists(&target)?;

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
  
  let work_load = WorkLoad(
    resume_key_chars,
    max,
    sequencer,
    target.to_string(),
    adjacent.map(str::to_string)
  );

  if matches.is_present("zip") {
    return unzip_core_loop(work_load);
  }

  aescrypt_core_loop(work_load)
}

fn main() {
  ::std::process::exit(
    match run_app() {
      Ok(_) => 0,
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
