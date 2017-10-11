use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
  AescryptMissing,
  FileMissing,
  InvalidAdjacentNumber,
  InvalidCharacterSet,
  InvalidRange,
  InvalidStringLength,
  PasswordNotFound,
  UnzipMissing,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::AescryptMissing       => f.write_str("AescryptMissing"      ),
      Error::FileMissing           => f.write_str("FileMissing"          ),
      Error::InvalidAdjacentNumber => f.write_str("InvalidAdjacentNumber"),
      Error::InvalidCharacterSet   => f.write_str("InvalidCharacterSet"  ),
      Error::InvalidRange          => f.write_str("InvalidRange"         ),
      Error::InvalidStringLength   => f.write_str("InvalidStringLength"  ),
      Error::PasswordNotFound      => f.write_str("PasswordNotFound"     ),
      Error::UnzipMissing          => f.write_str("UnzipMissing"         ),
    }
  }
}

#[inline]
fn aescrypt_missing() -> &'static str {
  "aescrypt does not appear to be installed."
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
fn unzip_missing() -> &'static str {
  "unzip does not appear to be installed."
}

impl StdError for Error {
  fn description(&self) -> &str {
    match *self {
      Error::AescryptMissing       => aescrypt_missing(),
      Error::FileMissing           => file_missing(),
      Error::InvalidAdjacentNumber => invalid_adjacent_number(),
      Error::InvalidCharacterSet   => invalid_character_set(),
      Error::InvalidRange          => invalid_range(),
      Error::InvalidStringLength   => invalid_string_length(),
      Error::PasswordNotFound      => password_not_found(),
      Error::UnzipMissing          => unzip_missing(),
    }
  }
}
