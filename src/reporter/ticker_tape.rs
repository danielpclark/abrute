use std::io::{self, Write};
pub struct TickerTape;

impl Reporter for TickerTape {
  fn report(data: ReportData) {
    print!("{}..", ReportData.last.to_s());
    io::stdout().flush().unwrap();
  }
}
