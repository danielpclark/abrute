use super::result::Error;

pub fn validate_adjacent_input(v: String) -> Result<(), Error> {
  if v.parse::<u8>().is_ok() { return Ok(()); } 
  Err(Error::InvalidAdjacentNumber)
}

pub fn validate_string_length(v: &str, max: usize) -> Result<(), Error> {
  if v.len() <= max { return Ok(()); } 
  Err(Error::InvalidStringLength)
}
