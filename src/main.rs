#[allow(unused_imports)]
extern crate digits;
extern crate rayon;
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

fn chunk_sequence(d: &mut Digits, qty: usize) -> Vec<String> {
  let mut counter = 0;
  let mut result = vec![];
  loop {
    if counter >= qty { break; }
    result.push(d.succ().to_s());
    counter += 1;
  }
  result
}

fn usage() -> String {
"abrute - AES Brute Force File Decryptor
----------------------------------------
Usage: abrute RANGE CHARACTERS TARGET

 RANGE\t\tCan be a single number or min max pair with a colon.
 \t\teg) abrute 4:6 asdf1234 file.tar.aes

 CHARACTERS\tShould not have quotes unless the password may have them.".to_string()
}

fn run_app() -> Result<(), String> {
  let mut args = env::args_os();
  args.next();

  if args.len() == 0 {
    println!("{}", usage());
    return Ok(());
  }

  if args.len() < 3 {
    writeln!(io::stderr(), "{}", usage()).err();
    return Err("Invalid arguments provided.".to_string());
  }

  let (min, max) = derive_min_max(args.next().unwrap());
  let mapping = derive_character_base(args.next().unwrap());
  let mut sequencer = Digits::new(&mapping, "".to_string());
  sequencer.zero_fill(min as usize);
  let target = args.next_back().unwrap();

  if !Path::new(&target).exists() {
    writeln!(io::stderr(), "Error: File {:?} does not exist.", target).err();
    return Err("Please verify last argument is the proper filename.".to_string())
  }

  loop {
    if sequencer.length() > max as usize {
      writeln!(io::stderr(), "Password not found for given length and character set.").err();
      return Err("EOL".to_string());
    }

    print!("{}..", sequencer.to_s()); // Verbose
    std::io::stdout().flush().unwrap();

    let chunk = chunk_sequence(&mut sequencer, 128);

    let result: i8 = chunk.par_iter().map(|ref value|
      {
        let output = Command::new("aescrypt").
          arg("-d").
          arg("-p").
          arg(&value).
          arg(&target).
          output().
          expect("Failed to execute decryption command!");

        if output.status.success() {
          println!("Success!\nPassword is: {}", value);
          1
        } else { 0 }
      }
    ).sum();

    if result == 1 {
      break;
    }
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
