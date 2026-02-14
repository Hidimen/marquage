use indexmap::IndexMap;

use crate::value::Value;

/// The internal implementation is provided by
/// [IndexMap](https://docs.rs/indexmap/latest/indexmap/index.html).
/// All methods are from `IndexMap`.
/// Go to crate's doc to get more information.
pub type ObjectImpl = IndexMap<String, Value>;

/// The internal implementation is provided by std
/// All methods are from [`Vec`].
/// Go to std's doc to get more information.
pub type ArrayImpl = Vec<Value>;
