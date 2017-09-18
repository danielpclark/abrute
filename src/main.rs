#[allow(unused_imports)]
extern crate digits;
extern crate rayon;
// extern crate array_fire;
extern crate num_cpus;
#[macro_use]
extern crate clap;

use clap::App;
use rayon::prelude::*;

fn main() {
  let cpus = num_cpus::get();
  let yaml = load_yaml!("../cli.yml");
  let matches = App::from_yaml(yaml).get_matches();
}
