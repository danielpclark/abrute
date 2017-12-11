use ::reporter::{Reporter,ReportData};
use std::io::{self, Write};

const SPINNER: &[char] = &['⠋','⠙','⠹','⠸','⠼','⠴','⠦','⠧','⠇','⠏'];

struct Spinner;

impl Reporter for Spinner {
  fn report(data: ReportData) {
    print!(":");
    io::stdout().flush().unwrap();
  }
}
