use std::io::{self, Write};
use ::reporter::Reporter;
use ::Digits;
pub struct TickerTape;

impl Reporter for TickerTape {
  fn report(data: &Digits) {
    print!("{}..", data.to_s());
    io::stdout().flush().unwrap();
  }
}
