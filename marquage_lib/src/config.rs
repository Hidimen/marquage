use std::{fs, path::Path};

use crate::{
  Deserializable, DeserializableError,
  deserializer::{Deserializer, lexer::Lexer},
};

/// Config
pub struct Config;

impl Config {
  /// Parse `Marquage` data from [`str`]
  ///
  /// # Example
  /// ```rust
  /// # use config::Config;
  /// # use config::value::Value;
  ///
  /// let instance: Value = Config::parse_from_string("name \"Jack\"; age 20;").unwrap();
  /// ```
  pub fn parse_from_string<T>(data: &str) -> Result<T, DeserializableError>
  where
    T: Deserializable,
  {
    let lexer = Lexer::new(data.to_string());
    let deserializer = Deserializer::new(lexer);
    match deserializer.parse() {
      Ok(val) => T::deserialize(&val),
      Err(e) => Err(DeserializableError::ParsingError(e)),
    }
  }

  pub fn parse_from_file<T>(path: &Path) -> Result<T, DeserializableError>
  where
    T: Deserializable,
  {
    match fs::exists(path) {
      Ok(true) => todo!(),
      Ok(false) => todo!(),
      Err(e) => Err(DeserializableError::IOError(e)),
    }
  }
}
