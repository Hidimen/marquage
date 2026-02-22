use crate::data::Value;

/// A data structure that can be generate into a [Value].
pub trait Generable {
  /// Generate [Value].
  fn generate(self) -> Value;
  /// Generate [Value] with ref.
  fn generate_ref(&self) -> Value;
}
