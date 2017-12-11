extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;
use ::reporter::{Reporter,ReportData};
use std::io::{self, Write};
pub struct JSON;

impl Reporter for JSON {
  fn report(data: ReportData) {
    let _a = serde_json::to_string(&ReportData)?;
  }
}
