use ::reporter::Reporter;
use ::{Digits,ITERATIONS};
use std::io::{self, Write};

const SPINNER: [char; 10] = ['⠋','⠙','⠹','⠸','⠼','⠴','⠦','⠧','⠇','⠏'];

struct Spinner;

impl Reporter for Spinner {
  fn report(_data: &Digits) {
    print!(":");
    io::stdout().flush().unwrap();
  }
}
