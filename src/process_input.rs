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

