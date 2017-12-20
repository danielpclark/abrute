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
