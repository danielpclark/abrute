// Copyright 2017 Daniel P. Clark & other abrute Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use digits::Digits;
use std::io::{Read}; 
use std::process::{Command, Output};
use rayon::prelude::*;
use super::result::Error;
extern crate num_cpus;
extern crate tempdir;
use resume::{ResumeKey,ResumeFile};
use self::tempdir::TempDir;
use std::{fs,path,env};
use std::time::{Duration, Instant};
use ::{WorkLoad,ITERATIONS,SUCCESS};
use std::sync::{Arc, Mutex};
use reporter::CliReporter;
use std::sync::atomic::Ordering;

fn has_five_minutes_passed(t: Instant) -> bool {
  Instant::now().duration_since(t) > Duration::new(300,0)
}

fn update_report_data(five_min_iters: usize, last: &Digits, fmp: &Arc<Mutex<(usize, String)>>) {
  let mut lock = fmp.try_lock();
  if let Ok(ref mut mutex) = lock {
    **mutex = (five_min_iters, last.to_s());
  }
}

fn chunk_sequence(d: &mut Digits, adj: Option<String>, chunk: usize, step: Option<usize>) -> Vec<String> {
  let qty: usize = num_cpus::get() * chunk;
  let mut counter = 0;
  let mut result = vec![];
  loop {
    if counter >= qty { break; }

    if let Some(a) = adj.clone() { 
      if d.base() > 3 {
        for _ in 0..step.unwrap_or(1) {
          d.step_non_adjacent(a.parse::<u8>().unwrap() as usize);
        }
        result.push(d.to_s());
        counter += 1;
        continue;
      }
    }

    let step_size = d.gen(step.unwrap_or(1) as u64);
    result.push(d.mut_add(step_size).to_s());
    counter += 1;
  }
  result
}

fn aes_command(value: &str, target: &str) -> Output {
  Command::new("aescrypt").
    arg("-d").
    arg("-p").
    arg(value).
    arg(target).
    output().
    unwrap()
}

fn unzip_command(value: &str, target: &str) -> Output {
  let mut dir = path::PathBuf::from(&target);
  dir.pop();
  Command::new("unzip").
    current_dir(dir).
    arg("-u").
    arg("-P").
    arg(value).
    arg(target).
    output().
    unwrap()
}

fn progress_report<'a>(reporter: &CliReporter, sequencer: &Digits) {
  reporter.report(sequencer);
}

fn has_reached_end<'a>(sequencer: &Digits, max: usize) -> Result<(), Error> {
  if sequencer.length() > max {
    return Err(Error::PasswordNotFound);
  }

  Ok(())
}

pub fn aescrypt_core_loop<'a>(work_load: WorkLoad) -> Result<(), Error> {
  let WorkLoad(
    characters,
    max,
    mut sequencer,
    target,
    adj,
    chunk_size,
    cluster_step,
    reporter_handler,
    cli_reporter
  ) = work_load;
  let mut time_keeper = Instant::now();
  let mut five_minute_iterations: usize = 0;
  loop {
    has_reached_end(&sequencer, max)?;
    progress_report(&cli_reporter, &sequencer);

    let chunk = chunk_sequence(
      &mut sequencer,
      adj.clone(),
      chunk_size.clone().map_or(32, |s| s.parse::<usize>().ok().unwrap()),
      cluster_step
    );
    let code: Mutex<Vec<String>> = Mutex::new(vec![]);

    chunk.par_iter().for_each(|ref value|
      {
        let output = aes_command(&value, &target);

        ITERATIONS.fetch_add(1, Ordering::SeqCst);

        if output.status.success() {
          let mut code_mutex = code.lock().unwrap();
          code_mutex.push(value.clone().to_string());
          SUCCESS.store(true, Ordering::SeqCst);
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
      
      aes_command(code.first().unwrap(), &target[..]);
      ResumeFile::purge();
      break;
    }

    if has_five_minutes_passed(time_keeper) {
      let global_iterations = ITERATIONS.load(Ordering::SeqCst);
      five_minute_iterations = global_iterations - five_minute_iterations;
      ResumeFile::save(
        ResumeKey::new(
          characters.clone(),
          adj.clone(),
          sequencer.clone(),
          target.to_string()
        )
      );

      update_report_data(five_minute_iterations, &sequencer, &reporter_handler.five_min_progress);
      five_minute_iterations = global_iterations;
      time_keeper = Instant::now();
    }
  }

  Ok(())
}

fn any_file_contents(dir: &TempDir, omit: &str) -> bool {
  let work_dir = fs::read_dir(&dir).expect("Failure reading tempdir's contents.");
  let mut work_iter = work_dir.into_iter();
  work_iter.any(|x| {
    let entry = x.expect("Failure reading specific file in tempdir.");

    if path::Path::new(&entry.path()) != path::Path::new(&omit) {
      if fs::File::open(&entry.path()).
        expect("Could not open file for validity check in tempdir.").
        bytes().count() > 1 {
        true
      } else { false }
    } else { false }
  })
}

pub fn unzip_core_loop<'a>(work_load: WorkLoad) -> Result<(), Error> {
  let WorkLoad(
    characters,
    max,
    mut sequencer,
    target,
    adj,
    chunk_size,
    cluster_step,
    reporter_handler,
    cli_reporter
  ) = work_load;
  let mut time_keeper = Instant::now();
  let mut five_minute_iterations: usize = 0;
  if let Ok(dir) = TempDir::new("abrute") {
    let cwd = env::current_dir().unwrap();
    let working = path::Path::new(&dir.path().as_os_str()).join(&target);
    fs::copy(&target, &working).unwrap();
    assert!(working.is_file());
    let target = working.to_str().unwrap();

    loop {
      has_reached_end(&sequencer, max)?;
      progress_report(&cli_reporter, &sequencer);

      let chunk = chunk_sequence(
        &mut sequencer,
        adj.clone(),
        chunk_size.clone().map_or(32, |s| s.parse::<usize>().ok().unwrap()),
        cluster_step
      );
      let code: Mutex<Vec<Result<(), Error>>> = Mutex::new(vec![]);

      chunk.par_iter().for_each(|ref value|
        {
          let output = unzip_command(&value, &target);

          ITERATIONS.fetch_add(1, Ordering::SeqCst);

          if output.status.success() {
            if any_file_contents(&dir, &target) {
              fs::read_dir(&dir).
                expect("Failure reading tempdir's contents.").
                into_iter().
                for_each( |entry| {
                let entry = entry.expect("Failure reading specific file in tempdir.");
                let file_name = entry.file_name();
                let dest_file = path::Path::new(&cwd).join(file_name);

                fs::copy(entry.path(), dest_file).
                  expect("Failure copying file from tempdir.");
              });
              let mut code_mutex = code.lock().unwrap();
              code_mutex.push(Ok(()));
              SUCCESS.store(true, Ordering::SeqCst);
              println!("Success!\nPassword is: {}", value);
            }
          }
        }
      );

      let mut code = code.lock().unwrap();
      if !code.is_empty() {
        ResumeFile::purge();
        return code.pop().unwrap();
      }


      if has_five_minutes_passed(time_keeper) {
        let global_iterations = ITERATIONS.load(Ordering::SeqCst);
        five_minute_iterations = global_iterations - five_minute_iterations;
        ResumeFile::save(
          ResumeKey::new(
            characters.clone(),
            adj.clone(),
            sequencer.clone(),
            target.to_string()
          )
        );

        update_report_data(five_minute_iterations, &sequencer, &reporter_handler.five_min_progress);
        five_minute_iterations = global_iterations;
        time_keeper = Instant::now();
      }
    }
  } else {
    return Err(Error::FailedTempDir);
  }
}
