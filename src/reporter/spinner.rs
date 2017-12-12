use ::reporter::Reporter;
use ::Digits;
use std::io::{self, Write};

const SPINNER: &[char] = &['⠋','⠙','⠹','⠸','⠼','⠴','⠦','⠧','⠇','⠏'];

struct Spinner;

impl Reporter for Spinner {
  fn report(_data: &Digits) {
    print!(":");
    io::stdout().flush().unwrap();
  }
}
