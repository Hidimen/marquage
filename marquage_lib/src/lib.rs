//! # Marquage
//! A library aiming at fast `marquage` resolving.
//!
//! ## Introducing Marquage
//! ### What is Marquage
//! **Marquage** means mark.
//! It is a new mark language designed to address certain issues, including
//! poor readability in popular languages like `JSON`, slow parsing and so on.
//!
//! It is not to replace other languages, but to provide a new solution in
//! effective file parsing and easy-to-read format.
//! For it is under development, some features may have not been covered yet.
//! But we hope this file format will help improve efficiency and offer a
//! flexible expressing way.
//!
//! ### Field it covers
//! `Marquage` can be used in these situations:
//! - Server Config
//! - Data transferring
//! - Data storage
//!
//! ### Ability
//! `Marquage` contains two presenting method: plaintext and binary.
//!
//! In the near future, a syntax supporting customized parsing rules
//! will be rolled out.
//!
//! ### Syntax
//! `Marquage` has simple yet powerful syntax.
//! #### Data Types
//! - String
//! - Number(Signed Unsigned and Float)
//! - Boolean
//! - Void
//! - Object
//! - Array
//!
//! #### Basic Syntax
//! In `Marquage`, each valid statement should be:
//! ``<key> <value>;``.
//! Behind a statement must be a semicolon.
//! Plus, it is necessary to add a semicolon behind brace as well.
//! Here's a example:
//! ```marquage
//! string "I am a string";
//! number_signed -100;
//! number_unsigned 100;
//! number_float 100.0;
//! boolean true;
//! null_value void;
//!
//! array ["Supporting different types", 1, false, void];
//! object {
//!   field "I am a child field";
//! };
//! ```
//!
//! #### Notes
//! Every string without quotes must not contain symbols below:
//! - semicolon(;)
//! - space
//! - Brace
//! - Bracket
//! - Paren
//!
//! String with quotes support escaping quotation marks.
//!
//! Number may be overflowed.
//!
//! Every entry key cannot be `void`, which will be parsed as `Void` type.
//!
//! #### Reference
//! It provides the ability of parsing references in files(only support in plaintext form).
//! When parser comes to a ref, it will replace it with specific values you defined.
//!
//! The following example will show you how to use:
//! ```marquage
//! &ref = "I am a ref";
//!
//! who_am_i *ref;
//! ```
//! **Note**: reference cannot store array or object types.
//! Every reference must defined before it is dereferenced.
//!
//! #### Comment
//! `Marquage` support native comment support(only in plaintext form). To write a comment in files,
//! please start with double slash, then contents after it will be automatically ignored.
//! Here is an example:
//! ```marquage
//! // I am a comment
//! ```
//!
//! ### Example
//!
//! ```marquage
//! name "Marquage";
//!
//! description "A simple mark language mainly used as config files";
//!
//! version [0,1,0];
//!
//! &name "Alice";
//!
//! addresses {
//!   Bob "Somewhere";
//!   Lily "Somewhere";
//! };
//!
//! allowlists ["Henry", *ref];
//! ```
//! ## How to use this library
//!
//!
mod config;
mod deserializable;
pub mod deserializer;
pub mod map;
pub mod parser;
mod serializable;
pub mod serializer;
pub mod value;
#[macro_use]
mod macros;

pub use config::Config;
pub use deserializable::{Deserializable, DeserializableError};
pub use serializable::Serializable;
