#[allow(unused_imports)]
extern crate digits;
extern crate rayon;
// extern crate array_fire;
extern crate num_cpus;
use std::process::Command;
use digits::{BaseCustom, Digits};
use std::path::Path;
use std::env;
use std::ffi::OsString;
use std::io::{self, Write}; 

use rayon::prelude::*;

fn derive_min_max(range: OsString) -> (i32, i32) {
  let rvals = range.to_str().unwrap().split(':').collect::<Vec<&str>>();
  let mut rivals = rvals.iter();
  let min = rivals.next().unwrap();
  let max = rivals.next();
  let min = min.parse::<i32>().unwrap();
  let get_max = || -> i32 {
    match max {
      Some(v) => { v.parse::<i32>().unwrap() },
      _ => { min }
    }
  };
  (min, get_max())
}

fn derive_character_base(characters: OsString) -> BaseCustom<char> {
  let chrs = characters.to_str().unwrap();
  BaseCustom::<char>::new(chrs.chars().collect())
}

fn run_app() -> Result<(), &'static str> {
  let cpus = num_cpus::get();
  let mut args = env::args_os();
  args.next();

  if args.len() < 3 {
    return Err(("Invalid arguments provided."));
  }

  let (min, max) = derive_min_max(args.next().unwrap());
  let mapping = derive_character_base(args.next().unwrap());
  let mut sequencer = Digits::new(&mapping, "".to_string());
  sequencer.zero_fill(min as usize);
  let target = args.next_back().unwrap();

  if !Path::new(&target).exists() {
    writeln!(io::stderr(), "Error: File {:?} does not exist.", target);
  }

  loop {
    if sequencer.length() > max as usize {
      writeln!(io::stderr(), "Password not found for given length and character set.");
      return Err("EOL");
    }

    let output = Command::new("aescrypt").
      arg("-d").
      arg("-p").
      arg(sequencer.to_s()).
      arg(&target).
      output().
      expect("Failed to execute decryption command!");

    if output.status.success() {
      println!("Success!\nPassword is: {}", sequencer.to_s());
      break;
    }
    sequencer.succ();
  }

  Ok(())
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
