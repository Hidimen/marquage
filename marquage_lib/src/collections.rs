use std::{
  borrow::Borrow,
  hash::Hash,
  ops::{Index, IndexMut},
};

use crate::data::Value;

/// Representing map's data.
#[derive(Debug, PartialEq, Clone)]
pub struct Map {
  inner: indexmap::IndexMap<String, Value>,
}

impl Map {
  /// Create a new map.
  pub fn new() -> Self {
    Self { inner: indexmap::IndexMap::new() }
  }

  /// Create a map with capacity.
  ///
  /// # Panics
  /// Panic if n is zero.
  pub fn with_capacity(n: usize) -> Self {
    assert!(n == 0, "n cannot be zero");
    Self { inner: indexmap::IndexMap::with_capacity(n) }
  }

  /// Get the capacity of map.
  ///
  /// # Returns
  /// A usize number indicating map's capacity.
  pub fn capacity(&self) -> usize {
    self.inner.capacity()
  }

  /// Get count of elements in map.
  ///
  /// # Returns
  /// A usize number indicating count of elements. It might be less than capacity.
  pub fn len(&self) -> usize {
    self.inner.len()
  }

  /// Check if map is empty.
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  /// Clear the map.
  pub fn clear(&mut self) {
    self.inner.clear();
  }

  /// Shorten the map, keeping first `len` elements.
  ///
  /// **Note**: If `len` is greater than map's current length, it has no effect.
  pub fn truncate(&mut self, len: usize) {
    self.inner.truncate(len);
  }

  /// Split the map at given index.
  ///
  /// # Returns
  /// Two maps will be returned. One contains `[0, at)` elements,
  /// while the other contains the remaining.
  ///
  /// # Panics
  /// It will panic if `at` > `len`.
  pub fn split_off(&mut self, at: usize) -> Self {
    Self { inner: self.inner.split_off(at) }
  }

  /// Insert a key-value pair.
  ///
  /// If key already exists, then it will be remained and value is changed to the new one,
  /// the old one will be returned.
  ///
  /// # Returns
  /// - `Some(v)` if key exists.
  /// - `None` if key does not exist.
  pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
    self.inner.insert(key, value)
  }

  /// Insert a key-value pair, getting the element's index.
  ///
  /// If key already exists, then it will be remained and value is changed to the new one,
  /// the old one will be returned.
  ///
  /// # Returns
  /// - `(usize, Some(v))` if key exists.
  /// - `(usize, None)` if key does not exist.
  pub fn insert_index(&mut self, key: String, value: Value) -> (usize, Option<Value>) {
    self.inner.insert_full(key, value)
  }

  /// Get entry by key.
  ///
  /// # Returns
  /// If entry exists, `Some(&v)` will be returned.
  /// If does not exist, `None` will be returned.
  pub fn get<K>(&self, key: &K) -> Option<&Value>
  where
    String: Borrow<K>,
    K: ?Sized + Hash + Ord + Eq,
  {
    self.inner.get(key)
  }

  /// Get mutable entry by key.
  ///
  /// # Returns
  /// If entry exists, `Some(&mut v)` will be returned.
  /// If does not exist, `None` will be returned.
  pub fn get_mut<K>(&mut self, key: &K) -> Option<&mut Value>
  where
    String: Borrow<K>,
    K: ?Sized + Hash + Ord + Eq,
  {
    self.inner.get_mut(key)
  }

  /// Swap and remove an entry by key.
  ///
  /// # Returns
  /// If entry exists, `Some(v)` will be returned.
  /// If does not exist, `None` will be returned.
  pub fn swap_remove<K>(&mut self, key: &K) -> Option<Value>
  where
    String: Borrow<K>,
    K: ?Sized + Hash + Ord + Eq,
  {
    self.inner.swap_remove(key)
  }
}

impl Default for Map {
  fn default() -> Self {
    Self { inner: indexmap::IndexMap::new() }
  }
}

impl<K> Index<&K> for Map
where
  String: Borrow<K>,
  K: ?Sized + Hash + Ord + Eq,
{
  type Output = Value;

  fn index(&self, index: &K) -> &Self::Output {
    self.get(index).unwrap()
  }
}

impl<K> IndexMut<&K> for Map
where
  String: Borrow<K>,
  K: ?Sized + Hash + Ord + Eq,
{
  fn index_mut(&mut self, index: &K) -> &mut Self::Output {
    self.get_mut(index).unwrap()
  }
}

pub struct Iter<'a> {
  inner: indexmap::map::Iter<'a, String, Value>,
}

impl<'a> Iterator for Iter<'a> {
  type Item = (&'a String, &'a Value);

  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next()
  }
}

pub struct IterMut<'a> {
  inner: indexmap::map::IterMut<'a, String, Value>,
}

impl<'a> Iterator for IterMut<'a> {
  type Item = (&'a String, &'a mut Value);

  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next()
  }
}

pub struct IntoIter {
  inner: indexmap::map::IntoIter<String, Value>,
}

impl Iterator for IntoIter {
  type Item = (String, Value);

  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next()
  }
}

impl FromIterator<(String, Value)> for Map {
  fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
    Self { inner: indexmap::IndexMap::from_iter(iter) }
  }
}

impl IntoIterator for Map {
  type IntoIter = IntoIter;
  type Item = (String, Value);

  fn into_iter(self) -> Self::IntoIter {
    Self::IntoIter { inner: self.inner.into_iter() }
  }
}

impl<'a> IntoIterator for &'a Map {
  type IntoIter = Iter<'a>;
  type Item = (&'a String, &'a Value);

  fn into_iter(self) -> Self::IntoIter {
    Self::IntoIter { inner: self.inner.iter() }
  }
}

impl<'a> IntoIterator for &'a mut Map {
  type IntoIter = IterMut<'a>;
  type Item = (&'a String, &'a mut Value);

  fn into_iter(self) -> Self::IntoIter {
    Self::IntoIter { inner: self.inner.iter_mut() }
  }
}

impl<const N: usize> From<[(String, Value); N]> for Map {
  fn from(value: [(String, Value); N]) -> Self {
    Self { inner: indexmap::IndexMap::from(value) }
  }
}
