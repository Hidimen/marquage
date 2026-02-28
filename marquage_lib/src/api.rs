use crate::{
  Parseable,
  error::Error,
  parse::{Lexer, Parser},
};

/// Parse from [str].
///
/// # Errors
/// [Error] will be returned when errors occur in parsing stage and type casting.
pub fn from_str<T: Parseable>(data: &str) -> Result<T, Error> {
  let lexer = Lexer::new(data);
  let parser = Parser::new(lexer);
  let data = parser.parse()?;
  T::parse(data).map_err(Error::Cast)
}

/// Parse from a [u8] slice.
///
/// # Errors
/// [Error] will be returned when errors occur in parsing stage, type casting and string building.
pub fn from_slice<T: Parseable>(data: &[u8]) -> Result<T, Error> {
  match std::str::from_utf8(data) {
    Ok(v) => from_str(v),
    Err(_) => Err(Error::InvalidSlice),
  }
}

/// Parse from a [u8] slice.
///
/// # Errors
/// [Error] will be returned when errors occur in parsing stage and type casting.
///
/// # Safety
/// Ensure that [u8] slice represents a valid UTF-8 string.
pub unsafe fn from_slice_unchecked<T: Parseable>(data: &[u8]) -> Result<T, Error> {
  from_str(unsafe { std::str::from_utf8_unchecked(data) })
}
