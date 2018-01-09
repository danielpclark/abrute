// Copyright 2017-2018 Daniel P. Clark & other abrute Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use ::Digits;

pub enum CliReporter {
  TickerTape,
  Spinner,
  Benchmark,
}

impl CliReporter {
  #[inline]
  pub fn report(&self, data: &Digits) {
    match *self {
      ref _thingy @ CliReporter::TickerTape => ::reporter::ticker_tape::report(data),
      ref _thingy @ CliReporter::Spinner => ::reporter::spinner::report(data),
      ref _thingy @ CliReporter::Benchmark => ::reporter::benchmark::report(data),
    }
  }
}
