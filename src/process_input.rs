// Copyright 2017 Daniel P. Clark & other abrute Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use digits::BaseCustom;
use super::result::Error;

pub fn derive_min_max(range: &str) -> Result<(usize, usize), Error> {
  let rvals = range.split(':').collect::<Vec<&str>>();
  for item in &rvals { if item.parse::<u8>().is_err() { return Err(Error::InvalidRange); } }
  let mut rivals = rvals.iter();
  let min = rivals.next().unwrap();
  let max = rivals.next();
  let min = min.parse::<usize>().unwrap();
  let get_max = || -> usize {
    match max {
      Some(v) => { v.parse::<usize>().unwrap() },
      _ => { min }
    }
  };
  Ok((min, get_max()))
}

pub fn derive_character_base(characters: &str) -> BaseCustom<char> {
  BaseCustom::<char>::new(characters.chars().collect())
}

pub fn mapping_to_characters(m: &BaseCustom<char>) -> String {
  let mut crs = String::new();
  for x in 0..m.base as usize {
    crs.push_str(&m.gen(x as u64)[..]);
  }
  crs
}
