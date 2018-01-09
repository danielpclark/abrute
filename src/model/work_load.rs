// Copyright 2017-2018 Daniel P. Clark & other abrute Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use ::Digits;
use ::model::report_data::ReportData;
use ::model::cli_reporter::CliReporter;

pub struct WorkLoad(
  pub String,         // characters: String,
  pub usize,          // max: usize,
  pub Digits,         // mut sequencer: Digits,
  pub String,         // target: String,
  pub Option<String>, // adj: Option<String>
  pub Option<String>, // chunk: Option<String>
  pub Option<usize>,  // cluster_step: Option<(usize,usize)>
  pub ReportData,     // cloned ReportData for web JSON results and other reporters
  pub CliReporter,    // cli Reporter chosen
);
