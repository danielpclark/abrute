#[allow(unused_imports)]
extern crate digits;
extern crate array_tool;
extern crate rayon;
// extern crate array_fire;
extern crate num_cpus;
use digits::{BaseCustom, Digits};
use array_tool::vec::{Times,Join};
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

fn fill_with_zero<'a>(d: Digits<'a>, c: i32) -> Digits<'a> {
  d.propagate(vec![d.zero().to_s()].times(c).join(""))
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
  sequencer = fill_with_zero(sequencer, min);
  let target = args.next_back().unwrap();

  println!("min: {:?}\nmax: {:?}\ncharacters: {:?}\ntarget: {:?}", min, max, sequencer.to_s(), target);

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
