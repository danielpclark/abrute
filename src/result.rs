// Copyright 2017-2018 Daniel P. Clark & other abrute Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use aescry;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    DecryptionFailed,
    FailedTempDir,
    FileMissing,
    InvalidAdjacentNumber,
    InvalidCharacterSet,
    InvalidChunkNumber,
    InvalidRange,
    InvalidStringLength,
    NotAescryptFile,
    PasswordNotFound,
    MalformedResumeKey,
    UnzipMissing,
}

impl From<aescry::Error> for Error {
    fn from(_: aescry::Error) -> Self {
        Error::DecryptionFailed
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DecryptionFailed => f.write_str("DecryptionFailed"),
            Error::FailedTempDir => f.write_str("FailedTempDir"),
            Error::FileMissing => f.write_str("FileMissing"),
            Error::InvalidAdjacentNumber => f.write_str("InvalidAdjacentNumber"),
            Error::InvalidCharacterSet => f.write_str("InvalidCharacterSet"),
            Error::InvalidChunkNumber => f.write_str("InvalidChunkNumber"),
            Error::InvalidRange => f.write_str("InvalidRange"),
            Error::InvalidStringLength => f.write_str("InvalidStringLength"),
            Error::NotAescryptFile => f.write_str("NotAescryptFile"),
            Error::PasswordNotFound => f.write_str("PasswordNotFound"),
            Error::MalformedResumeKey => f.write_str("MalformedResumeKey"),
            Error::UnzipMissing => f.write_str("UnzipMissing"),
        }
    }
}

#[inline]
fn decryption_failed() -> &'static str {
    "The password was found but decrypting the target file failed."
}

#[inline]
fn not_aescrypt_file() -> &'static str {
    "The target file does not appear to be an AES Crypt (.aes) file."
}

#[inline]
fn failed_temp_dir() -> &'static str {
    "Failed in creating a temp directory to work in."
}

#[inline]
fn file_missing() -> &'static str {
    "The target file seems to be missing."
}

#[inline]
fn invalid_adjacent_number() -> &'static str {
    "Invalid number for adjacent input."
}

#[inline]
fn invalid_character_set() -> &'static str {
    "Invalid character set provided for start."
}

#[inline]
fn invalid_chunk_number() -> &'static str {
    "Invalid number for chunk input."
}

#[inline]
fn invalid_range() -> &'static str {
    "Invalid range input given."
}

#[inline]
fn invalid_string_length() -> &'static str {
    "Invalid string length for start given."
}

#[inline]
fn password_not_found() -> &'static str {
    "Password not found for given length and character set."
}

#[inline]
fn malformed_resume_key() -> &'static str {
    "The input data was not formatted properly for creating ResumeKey."
}

#[inline]
fn unzip_missing() -> &'static str {
    "unzip does not appear to be installed."
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DecryptionFailed => decryption_failed(),
            Error::FailedTempDir => failed_temp_dir(),
            Error::FileMissing => file_missing(),
            Error::InvalidAdjacentNumber => invalid_adjacent_number(),
            Error::InvalidCharacterSet => invalid_character_set(),
            Error::InvalidChunkNumber => invalid_chunk_number(),
            Error::InvalidRange => invalid_range(),
            Error::InvalidStringLength => invalid_string_length(),
            Error::NotAescryptFile => not_aescrypt_file(),
            Error::PasswordNotFound => password_not_found(),
            Error::MalformedResumeKey => malformed_resume_key(),
            Error::UnzipMissing => unzip_missing(),
        }
    }
}
