// Copyright 2017 Daniel P. Clark & other abrute Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
extern crate digits;
extern crate rayon;
use digits::Digits;
use std::path::Path;
use std::io::{self, Write}; 
mod process_input;
use process_input::*;
mod core;
use core::*;
#[macro_use]
extern crate clap;
use clap::{Arg, App};

fn run_app() -> Result<(), String> {
  let matches = App::new("abrute - AES Brute Force File Decryptor").
    version(&format!("v{}", crate_version!())[..]).
    version_short("v").
    author(crate_authors!("\n")).
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
    arg(Arg::with_name("TARGET").
          required(true).
          last(true)
    ).
    template("\
-------------------------------------------------------------
       {bin} {version}
-------------------------------------------------------------
           By: {author}

USAGE:\tabrute <RANGE> <CHARACTERS> [OPTIONS] [--] <TARGET>

  <RANGE>         Single digit or a range 4:6 for password length.
  <CHARACTERS>    Characters to use in password attempt. Don't use quotes unless
                  they may be in password. Backslash may escape characters such
                  as space.
  -a, --adjacent  Set a limit for allowed adjacent characters. Zero will not
                  allow any characters of the same kind to neghibor in the
                  attempts.
  <TARGET>        Target file to decrypt.
  -h, --help      Prints help information.
  -v, --version   Prints version information.

-------------------------------------------------------------
USE OF THIS BINARY FALLS UNDER THE MIT LICENSE       (c) 2017").
    get_matches();
    
  let (min, max) = derive_min_max(matches.value_of("RANGE").unwrap());
  let mapping = derive_character_base(matches.value_of("CHARACTERS").unwrap());
  let mut sequencer = Digits::new(&mapping, "".to_string());
  sequencer.zero_fill(min as usize);
  let target = matches.value_of("TARGET").unwrap();

  if !Path::new(&target).exists() {
    writeln!(io::stderr(), "Error: File {:?} does not exist.", target).err();
    return Err("Please verify last argument is the proper filename.".to_string())
  }

  core_loop(max, sequencer, target)
}

fn main() {
  ::std::process::exit(
    match run_app() {
      Ok(_) => 0,
      Err(err) => {
        writeln!(io::stderr(), "error: {:?}", err).unwrap();
        1
      }
    }
  );
}
