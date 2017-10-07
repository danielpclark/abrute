use digits::BaseCustom;

pub fn derive_min_max(range: &str) -> (i32, i32) {
  let rvals = range.split(':').collect::<Vec<&str>>();
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

pub fn derive_character_base(characters: &str) -> BaseCustom<char> {
  BaseCustom::<char>::new(characters.chars().collect())
}

