pub mod ticker_tape;
pub mod spinner;
use ::Digits;

#[derive(Debug)]
pub enum CliReporter {
  TickerTape,
  Spinner,
}

impl CliReporter {
  #[inline]
  pub fn report(&self, data: &Digits) {
    match *self {
      ref _thingy @ CliReporter::TickerTape => ticker_tape::report(data),
      ref _thingy @ CliReporter::Spinner => spinner::report(data),
    }
  }
}
