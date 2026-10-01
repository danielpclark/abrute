// Copyright 2017-2023 Daniel P. Clark & other abrute Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::result::Error;
use aescry::detect;
use clap;
use digits::Digits;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn validate_adjacent_input(v: &String) -> Result<(), Error> {
    if v.parse::<u8>().is_ok() {
        return Ok(());
    }
    Err(Error::InvalidAdjacentNumber)
}

pub fn validate_chunk_input(v: &str) -> Result<(), Error> {
    if v.parse::<usize>().is_ok() {
        return Ok(());
    }
    Err(Error::InvalidChunkNumber)
}

pub fn validate_start_string(matches: &clap::ArgMatches, max: usize) -> Result<(), Error> {
    if let Some(s) = matches.get_one::<String>("start") {
        if s.len() > max {
            return Err(Error::InvalidStringLength);
        }

        let chrctrs: Vec<char> = matches
            .get_one::<String>("CHARACTERS")
            .unwrap()
            .chars()
            .collect();
        let mut itr = s.chars();
        loop {
            match itr.next() {
                Some(ref c) => {
                    if !chrctrs.contains(c) {
                        return Err(Error::InvalidCharacterSet);
                    }
                }
                _ => break,
            }
        }
    }

    Ok(())
}

pub fn validate_and_prep_sequencer_adjacent<'a>(
    sequencer: &mut Digits,
    adjacent: Option<&String>,
) -> Result<(), Error> {
    let seq_base = sequencer.base();

    if let &Some(num) = &adjacent {
        validate_adjacent_input(num)?;
        if seq_base > 3 {
            sequencer.prep_non_adjacent(num.parse::<usize>().unwrap());
        }
    }

    Ok(())
}

pub fn validate_file_exists(target: &str) -> Result<(), Error> {
    if !Path::new(target).exists() {
        return Err(Error::FileMissing);
    }

    Ok(())
}

pub fn validate_aescrypt_file(target: &str) -> Result<(), Error> {
    // Decryption is handled in-process by the `aescry` crate, so there is no
    // external `aescrypt` executable to look for.  Instead confirm up front
    // that the target really is an AES Crypt stream, giving a clear error for
    // anything else before we spend time guessing passwords.
    if detect::get_file(target).is_none() {
        return Err(Error::NotAescryptFile);
    }

    Ok(())
}

pub fn validate_unzip_executable() -> Result<(), Error> {
    if Command::new("unzip")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_err()
    {
        return Err(Error::UnzipMissing);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_aescrypt_file;
    use crate::result::Error;
    use aescry::aescrypt::{Encryptor, Iterations};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn accepts_a_real_aescrypt_file() {
        let dir = TempDir::new().unwrap();
        let aes_path = dir.path().join("secret.txt.aes");
        let stream = Encryptor::new("pw")
            .unwrap()
            .iterations(Iterations::new(5).unwrap())
            .encrypt(b"data")
            .unwrap();
        fs::write(&aes_path, stream).unwrap();

        assert!(validate_aescrypt_file(aes_path.to_str().unwrap()).is_ok());
    }

    #[test]
    fn rejects_a_file_that_is_not_aescrypt() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("plain.aes");
        fs::write(&path, b"this is not an AES Crypt stream").unwrap();

        match validate_aescrypt_file(path.to_str().unwrap()) {
            Err(Error::NotAescryptFile) => {}
            other => panic!("expected NotAescryptFile, got {:?}", other),
        }
    }

    #[test]
    fn rejects_a_missing_file() {
        match validate_aescrypt_file("definitely/does/not/exist.aes") {
            Err(Error::NotAescryptFile) => {}
            other => panic!("expected NotAescryptFile, got {:?}", other),
        }
    }
}
