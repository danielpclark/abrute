use std::sync::Mutex;
use digits::Digits;
use std::io::{self, Write}; 
use std::process::Command;
use rayon::prelude::*;
extern crate num_cpus;

fn chunk_sequence(d: &mut Digits, qty: usize, adj: Option<&str>) -> Vec<String> {
  let mut counter = 0;
  let mut result = vec![];
  loop {
    if counter >= qty { break; }
    if let Some(a) = adj { 
      if d.base() > 3 {
        result.push(d.next_non_adjacent(a.parse::<u8>().unwrap() as usize).to_s());
      } else {
        result.push(d.succ().to_s());
      }
    } else {
      result.push(d.succ().to_s());
    }
    counter += 1;
  }
  result
}

pub fn core_loop<'a>(max: usize, mut sequencer: Digits<'a>, target: &str, adj: Option<&str>) -> Result<(),String> {
  loop {
    if sequencer.length() > max {
      writeln!(io::stderr(), "Password not found for given length and character set.").err();
      return Err("EOL".to_string());
    }

    print!("{}..", sequencer.to_s()); // Verbose
    io::stdout().flush().unwrap();

    let chunk = chunk_sequence(&mut sequencer, num_cpus::get() * 32, adj);
    let code: Mutex<Vec<String>> = Mutex::new(vec![]);

    chunk.par_iter().for_each(|ref value|
      {
        let output = Command::new("aescrypt").
          arg("-d").
          arg("-p").
          arg(&value).
          arg(&target).
          output().
          expect("Failed to execute decryption command!");

        if output.status.success() {
          let mut code_mutex = code.lock().unwrap();
          code_mutex.push(value.clone().to_string());
          println!("Success!\nPassword is: {}", value);
        }
      }
    );

    let code = code.lock().unwrap();
    if !code.is_empty() {
      // Other attempts will erase the output file as there is always an empty file
      // created in place when trying to decrypt. So we need to take the correct
      // answer and decrypt the source one last time.  Otherwise we'd need to isolate
      // every attempt in a temp dir or mem dir and copying that much data that many
      // times would be very slow and difficult to implement in a threaded way.
      Command::new("aescrypt").
        arg("-d").
        arg("-p").
        arg(code.first().unwrap()).
        arg(&target).
        output().
        expect("Failed to execute decryption command!");
      break;
    }
  }
  Ok(())
}
