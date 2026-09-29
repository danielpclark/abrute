// Copyright 2017-2023 Daniel P. Clark & other abrute Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Encrypt a file into the AES Crypt (`.aes`) format using the `aescry`
//! crate.  This is the counterpart to abrute's decryption and is used to
//! generate the encrypted fixtures the test-suite brute-forces, so the tests
//! no longer depend on the external `aescrypt` command line tool.
//!
//! Usage:
//!
//! ```text
//! cargo run --example aescrypt_encrypt -- <password> <input> <output.aes> [iterations]
//! ```
//!
//! `iterations` is the optional PBKDF2 iteration count (1..=5_000_000).  It
//! defaults to the `aescry` secure default (600,000).  The test-suite passes a
//! small value so that brute-forcing the fixture stays fast even in an
//! unoptimized debug build.

extern crate aescry;

use aescry::aescrypt::{Encryptor, Iterations};
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 || args.len() > 5 {
        eprintln!(
            "Usage: {} <password> <input> <output.aes> [iterations]",
            args.get(0).map(|s| s.as_str()).unwrap_or("aescrypt_encrypt")
        );
        process::exit(2);
    }

    let password = &args[1];
    let input = &args[2];
    let output = &args[3];

    let result = build_encryptor(password, args.get(4).map(|s| s.as_str()))
        .and_then(|encryptor| encryptor.encrypt_file(input, output));

    if let Err(err) = result {
        eprintln!("Encryption failed: {}", err);
        process::exit(1);
    }
}

fn build_encryptor(password: &str, iterations: Option<&str>) -> Result<Encryptor, aescry::Error> {
    let encryptor = Encryptor::new(password)?;
    match iterations {
        Some(text) => {
            let count: u32 = text.parse().unwrap_or_else(|_| {
                eprintln!("Invalid iteration count: {}", text);
                process::exit(2);
            });
            Ok(encryptor.iterations(Iterations::new(count)?))
        }
        None => Ok(encryptor),
    }
}
