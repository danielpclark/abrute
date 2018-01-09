use ::{Digits,ITERATIONS};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::Ordering;
use std::time::Instant;
use std::io;
use std::io::Write;

lazy_static! {
  // len => Iterations, Iteration Start, Start Instant (for Duration)
  static ref HASHMAP: Mutex<HashMap<u8, (u64, u64, Instant)>> = {
    let m = HashMap::new();
    Mutex::new(m)
  };
}

pub fn report(data: &Digits) {
  let len = data.length() as i32;
  let mut hm = HASHMAP.lock().unwrap();
  if hm.is_empty() {
    println!("Benchmark Results\nAttempt :: Category of Range:Iterations, Rating in IPS");
    hm.insert(len as u8, (0, 0, Instant::now()));
  }

  let reader = hm.clone();

  if hm.contains_key(&(len as u8)) {
    let entry = hm.get_mut(&(len as u8)).unwrap();
    let &mut (_, start, duration) = entry;

    let iters = ITERATIONS.load(Ordering::SeqCst);
    let iterated = iters - start.clone() as usize;

    let min = reader.keys().min().unwrap();
    let max = reader.keys().max().unwrap();
    let start_instant = reader.get(min).unwrap().2;
    let seconds = Instant::now().duration_since(start_instant).as_secs();
    let ips = if seconds == 0 { 0 } else { iters/seconds as usize };
    print!("\x1b[1000D{} :: Category: {}->{}:{}, Rating Total: {} IPS", data.to_s(), min, max, iterated, ips);

    *entry = (iterated as u64, start.clone(), duration.clone());
  } else {
   // New Entry
    let previous = len - 1;
    let has_previous = hm.contains_key(&(previous as u8));
    let start_iteration = if has_previous { hm.get(&(previous as u8)).unwrap().0 } else { 0 };

    let iters = ITERATIONS.load(Ordering::SeqCst);
    let instant = Instant::now();

    hm.insert(len as u8, (start_iteration, iters as u64, instant));

    if has_previous {
      let iterations =
        if hm.contains_key(&((previous) as u8)) {
          hm.get(&((previous) as u8)).unwrap().0 as usize
        } else { 0 };
      let start = hm.get(&(previous as u8)).unwrap().2;
      let seconds = Instant::now().duration_since(start).as_secs();
      let ips = if seconds == 0 { 0 } else { iterations/(seconds as usize) };
      println!("\nRange: {}, Iterations: {}, Interim Rating: {} IPS", previous, iterations, ips);
    }
  }
  io::stdout().flush().unwrap();
}
