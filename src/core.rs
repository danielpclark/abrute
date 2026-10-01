// Copyright 2017-2023 Daniel P. Clark & other abrute Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::result::Error;
use crate::model::cli_reporter::CliReporter;
use crate::model::work_load::WorkLoad;
use crate::resume::{ResumeFile, ResumeKey};
use crate::{ITERATIONS, SUCCESS};
use aescry::aescrypt::Decryptor;
use aescry::security::{self, DecryptKey};
use digits::Digits;
use rayon::prelude::*;
use std::io::Read;
use std::process::{Command, Output};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{env, fs, path};
use tempfile::{Builder, TempDir};

fn has_five_minutes_passed(t: Instant) -> bool {
    Instant::now().duration_since(t) > Duration::new(300, 0)
}

fn update_report_data(five_min_iters: usize, last: &Digits, fmp: &Arc<Mutex<(usize, String)>>) {
    let mut lock = fmp.try_lock();
    if let Ok(ref mut mutex) = lock {
        **mutex = (five_min_iters, last.to_s());
    }
}

fn chunk_sequence(
    d: &mut Digits,
    adj: Option<String>,
    chunk: usize,
    step: Option<usize>,
) -> Vec<String> {
    let qty: usize = num_cpus::get() * chunk;
    let mut counter = 0;
    let mut result = vec![];
    loop {
        if counter >= qty {
            break;
        }

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

/// Return `true` when `password` correctly opens the AES Crypt stream in
/// `data`.
///
/// This uses `aescry`'s `security::verify`, which runs the same key
/// derivation and HMAC checks a real decryption would, but throws the
/// plaintext away instead of allocating it.  That keeps every wrong guess as
/// cheap as possible while still only reporting success when the password can
/// actually produce the original file.  A wrong password is reported as a
/// mismatch (not an error); genuine errors (a corrupt or non-AES-Crypt
/// stream) simply count as "no match" so the search can continue.
fn password_matches(password: &str, data: &[u8]) -> bool {
    match DecryptKey::password(password) {
        Ok(key) => security::verify(&key, data)
            .map(|verification| verification.is_authentic())
            .unwrap_or(false),
        Err(_) => false,
    }
}

/// Where the decrypted plaintext should be written for a given target.
///
/// Mirrors the behaviour of the `aescrypt` CLI: a `foo.txt.aes` target
/// decrypts to `foo.txt` alongside it.  A target without the `.aes`
/// extension gets a `.decrypted` suffix so we never overwrite the ciphertext.
fn output_path_for(target: &str) -> path::PathBuf {
    let path = path::Path::new(target);
    if path.extension().and_then(|ext| ext.to_str()) == Some("aes") {
        path.with_extension("")
    } else {
        let mut name = target.to_string();
        name.push_str(".decrypted");
        path::PathBuf::from(name)
    }
}

/// Decrypt `target` with the discovered `password`, writing the plaintext to
/// disk and returning the path it was written to.
fn decrypt_to_file(password: &str, target: &str) -> Result<path::PathBuf, Error> {
    let output = output_path_for(target);
    Decryptor::new(password)?.decrypt_file(target, &output)?;
    Ok(output)
}

fn unzip_command(value: &str, target: &str) -> Output {
    let mut dir = path::PathBuf::from(&target);
    dir.pop();
    Command::new("unzip")
        .current_dir(dir)
        .arg("-u")
        .arg("-P")
        .arg(value)
        .arg(target)
        .output()
        .unwrap()
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
        cli_reporter,
    ) = work_load;

    // The encrypted file never changes while we search, so read it into memory
    // once and verify every guess against those bytes in-process with `aescry`.
    let data = fs::read(&target).map_err(|_| Error::FileMissing)?;

    let mut time_keeper = Instant::now();
    let mut five_minute_iterations: usize = 0;
    loop {
        has_reached_end(&sequencer, max)?;
        progress_report(&cli_reporter, &sequencer);

        let chunk = chunk_sequence(
            &mut sequencer,
            adj.clone(),
            chunk_size
                .clone()
                .map_or(32, |s| s.parse::<usize>().ok().unwrap()),
            cluster_step,
        );
        let code: Mutex<Vec<String>> = Mutex::new(vec![]);

        chunk.par_iter().for_each(|ref value| {
            let matched = password_matches(value, &data);

            ITERATIONS.fetch_add(1, Ordering::SeqCst);

            if matched {
                let mut code_mutex = code.lock().unwrap();
                code_mutex.push(value.to_string());
                SUCCESS.store(true, Ordering::SeqCst);
                println!("Success!\nPassword is: {}", value);
            }
        });

        let code = code.lock().unwrap();
        if !code.is_empty() {
            // The guessing loop only verifies the password (no plaintext is
            // produced), so now that we have the answer, decrypt the source
            // file once to write the recovered plaintext to disk.
            decrypt_to_file(code.first().unwrap(), &target)?;
            ResumeFile::purge();
            break;
        }

        if has_five_minutes_passed(time_keeper) {
            let global_iterations = ITERATIONS.load(Ordering::SeqCst);
            five_minute_iterations = global_iterations - five_minute_iterations;
            ResumeFile::save(ResumeKey::new(
                characters.clone(),
                adj.clone(),
                sequencer.clone(),
                target.to_string(),
            ));

            update_report_data(
                five_minute_iterations,
                &sequencer,
                &reporter_handler.five_min_progress,
            );
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
            if fs::File::open(&entry.path())
                .expect("Could not open file for validity check in tempdir.")
                .bytes()
                .count()
                > 1
            {
                true
            } else {
                false
            }
        } else {
            false
        }
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
        cli_reporter,
    ) = work_load;
    let mut time_keeper = Instant::now();
    let mut five_minute_iterations: usize = 0;
    if let Ok(dir) = Builder::new().prefix("abrute").tempdir() {
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
                chunk_size
                    .clone()
                    .map_or(32, |s| s.parse::<usize>().ok().unwrap()),
                cluster_step,
            );
            let code: Mutex<Vec<Result<(), Error>>> = Mutex::new(vec![]);

            chunk.par_iter().for_each(|ref value| {
                let output = unzip_command(&value, &target);

                ITERATIONS.fetch_add(1, Ordering::SeqCst);

                if output.status.success() {
                    if any_file_contents(&dir, &target) {
                        fs::read_dir(&dir)
                            .expect("Failure reading tempdir's contents.")
                            .into_iter()
                            .for_each(|entry| {
                                let entry =
                                    entry.expect("Failure reading specific file in tempdir.");
                                let file_name = entry.file_name();
                                let dest_file = path::Path::new(&cwd).join(file_name);

                                fs::copy(entry.path(), dest_file)
                                    .expect("Failure copying file from tempdir.");
                            });
                        let mut code_mutex = code.lock().unwrap();
                        code_mutex.push(Ok(()));
                        SUCCESS.store(true, Ordering::SeqCst);
                        println!("Success!\nPassword is: {}", value);
                    }
                }
            });

            let mut code = code.lock().unwrap();
            if !code.is_empty() {
                ResumeFile::purge();
                return code.pop().unwrap();
            }

            if has_five_minutes_passed(time_keeper) {
                let global_iterations = ITERATIONS.load(Ordering::SeqCst);
                five_minute_iterations = global_iterations - five_minute_iterations;
                ResumeFile::save(ResumeKey::new(
                    characters.clone(),
                    adj.clone(),
                    sequencer.clone(),
                    target.to_string(),
                ));

                update_report_data(
                    five_minute_iterations,
                    &sequencer,
                    &reporter_handler.five_min_progress,
                );
                five_minute_iterations = global_iterations;
                time_keeper = Instant::now();
            }
        }
    } else {
        return Err(Error::FailedTempDir);
    }
}

#[cfg(test)]
mod tests {
    use super::{decrypt_to_file, output_path_for, password_matches};
    use aescry::aescrypt::{Encryptor, Iterations};
    use std::fs;
    use std::path;
    use tempfile::TempDir;

    // A small iteration count keeps these tests fast; the value the file was
    // written with is read back from its header at verification time.
    fn encrypt(password: &str, plaintext: &[u8]) -> Vec<u8> {
        Encryptor::new(password)
            .unwrap()
            .iterations(Iterations::new(5).unwrap())
            .encrypt(plaintext)
            .unwrap()
    }

    #[test]
    fn password_matches_accepts_the_correct_password() {
        let data = encrypt("swordfish", b"top secret");
        assert!(password_matches("swordfish", &data));
    }

    #[test]
    fn password_matches_rejects_a_wrong_password() {
        let data = encrypt("swordfish", b"top secret");
        assert!(!password_matches("clownfish", &data));
    }

    #[test]
    fn password_matches_rejects_non_aescrypt_bytes() {
        assert!(!password_matches(
            "swordfish",
            b"plainly not an AES Crypt stream"
        ));
    }

    #[test]
    fn output_path_for_strips_the_aes_extension() {
        assert_eq!(
            output_path_for("secret.txt.aes"),
            path::PathBuf::from("secret.txt")
        );
        assert_eq!(
            output_path_for("some/dir/secret.txt.aes"),
            path::PathBuf::from("some/dir/secret.txt")
        );
    }

    #[test]
    fn output_path_for_appends_when_not_aes() {
        assert_eq!(
            output_path_for("payload.bin"),
            path::PathBuf::from("payload.bin.decrypted")
        );
    }

    #[test]
    fn decrypt_to_file_writes_the_recovered_plaintext() {
        let dir = TempDir::new().unwrap();
        let aes_path = dir.path().join("message.txt.aes");
        let plaintext = b"Hello World!\n";
        fs::write(&aes_path, encrypt("4321", plaintext)).unwrap();

        let output = decrypt_to_file("4321", aes_path.to_str().unwrap()).unwrap();

        assert_eq!(output, dir.path().join("message.txt"));
        assert_eq!(fs::read(&output).unwrap(), plaintext);
    }

    #[test]
    fn decrypt_to_file_fails_with_a_wrong_password() {
        let dir = TempDir::new().unwrap();
        let aes_path = dir.path().join("message.txt.aes");
        fs::write(&aes_path, encrypt("4321", b"secret")).unwrap();

        assert!(decrypt_to_file("0000", aes_path.to_str().unwrap()).is_err());
    }
}
