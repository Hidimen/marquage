use crate::value::Value;

#[derive(Debug)]
pub enum SerializerError {
  UnsupportedValueType(Value),
}

impl std::fmt::Display for SerializerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::UnsupportedValueType(val) => write!(f, "Unsupported value type: {:?}", val),
    }
  }
}

impl std::error::Error for SerializerError {}
