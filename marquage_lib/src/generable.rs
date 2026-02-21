use crate::data::Value;

pub trait Generable {
  fn generate(self) -> Value;
  fn generate_ref(&self) -> Value;
}
