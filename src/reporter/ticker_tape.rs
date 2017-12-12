use std::io::{self, Write};
use ::Digits;

pub fn report(data: &Digits) {
  print!("{}..", data.to_s());
  io::stdout().flush().unwrap();
}
