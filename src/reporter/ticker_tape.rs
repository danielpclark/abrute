// Copyright 2017-2018 Daniel P. Clark & other abrute Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::{self, Write};
use ::Digits;

pub fn report(data: &Digits) {
  print!("{}..", data.to_s());
  io::stdout().flush().unwrap();
}
