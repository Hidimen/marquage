#![doc = include_str!("../../docs/main.md")]
mod api;
pub mod data;
pub mod error;
mod generable;
pub mod generate;
pub mod parse;
mod parseable;
#[macro_use]
pub mod macros;

pub use api::from_str;
pub use generable::Generable;
pub use parseable::Parseable;
