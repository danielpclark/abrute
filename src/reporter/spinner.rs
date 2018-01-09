// Copyright 2017-2018 Daniel P. Clark & other abrute Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use ::{Digits,ITERATIONS};
use std::sync::atomic::Ordering;
use std::io::{self, Write};

const SPINNER: [char; 10] = ['⠋','⠙','⠹','⠸','⠼','⠴','⠦','⠧','⠇','⠏'];

pub fn report(data: &Digits) {
  let global_iterations = ITERATIONS.load(Ordering::SeqCst);
  print!(
    "\x1b[1000D {}  Iterations: {}  String: {}",
    SPINNER[global_iterations % 10],
    global_iterations,
    data.to_s()
  );
  io::stdout().flush().unwrap();
}
