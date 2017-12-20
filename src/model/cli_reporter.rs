use ::Digits;

pub enum CliReporter {
  TickerTape,
  Spinner,
}

impl CliReporter {
  #[inline]
  pub fn report(&self, data: &Digits) {
    match *self {
      ref _thingy @ CliReporter::TickerTape => ::reporter::ticker_tape::report(data),
      ref _thingy @ CliReporter::Spinner => ::reporter::spinner::report(data),
    }
  }
}
