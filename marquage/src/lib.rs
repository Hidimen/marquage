#![doc = include_str!("../../docs/main.md")]

#[cfg(feature = "core")]
pub use marquage_lib::*;

#[cfg(feature = "derive")]
pub use marquage_derive::*;