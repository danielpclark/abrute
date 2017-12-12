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
