// Copyright 2017-2018 Daniel P. Clark & other abrute Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::result::Error;
use clap;
use digits::Digits;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn validate_adjacent_input(v: String) -> Result<(), Error> {
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
    if let Some(s) = matches.value_of("start") {
        if s.len() > max {
            return Err(Error::InvalidStringLength);
        }

        let chrctrs: Vec<char> = matches.value_of("CHARACTERS").unwrap().chars().collect();
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
    adjacent: Option<&str>,
) -> Result<(), Error> {
    let seq_base = sequencer.base();

    if let &Some(num) = &adjacent {
        validate_adjacent_input(num.to_string())?;
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

pub fn validate_aescrpyt_executable() -> Result<(), Error> {
    if Command::new("aescrypt")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_err()
    {
        return Err(Error::AescryptMissing);
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
