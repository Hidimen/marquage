#![doc = include_str!("../../docs/introduction.md")]
mod api;
pub mod data;
pub mod error;
pub mod parse;
mod parseable;
#[macro_use]
pub mod macros;

pub use api::from_str;
pub use parseable::Parseable;
