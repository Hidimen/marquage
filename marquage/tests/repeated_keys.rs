use marquage::{Parse, data::Value, error::CastError, error::Error, from_str};

/// A key occurring once stays a scalar, but a `Vec` field accepts it as a
/// one-element array.
#[derive(Parse, Debug, PartialEq)]
struct Tags {
  tag: Vec<String>,
}

#[test]
fn single_key_is_promoted_to_vec() {
  let parsed: Tags = from_str("tag = rust;").unwrap();
  assert_eq!(parsed, Tags { tag: vec!["rust".to_string()] });
}

/// A key occurring several times is collected into an array by the parser.
#[test]
fn repeated_keys_are_collected_into_vec() {
  let parsed: Tags = from_str("tag = rust;\ntag = marquage;").unwrap();
  assert_eq!(parsed, Tags { tag: vec!["rust".to_string(), "marquage".to_string()] });
}

/// An explicit array literal is left untouched by both the parser and the
/// promotion.
#[test]
fn array_literal_is_unchanged() {
  let parsed: Tags = from_str(r#"tag = ["rust", "marquage"];"#).unwrap();
  assert_eq!(parsed, Tags { tag: vec!["rust".to_string(), "marquage".to_string()] });
}

/// The parser collects repeated keys into an array, keeping the insertion order.
#[test]
fn repeated_keys_are_collected_by_the_parser() {
  let parsed: Value = from_str("key = 1;\nkey = 2;\nkey = 3;").unwrap();
  let map = parsed.as_object_ref().unwrap();
  assert_eq!(
    map.get("key"),
    Some(&Value::Array(vec![
      Value::UnsignedIntegerNumber(1),
      Value::UnsignedIntegerNumber(2),
      Value::UnsignedIntegerNumber(3),
    ]))
  );
}

/// An existing array is not flattened, so the array's length always equals the
/// count of the key's definitions.
#[test]
fn nested_arrays_are_not_flattened() {
  let parsed: Value = from_str("key = [1, 2];\nkey = [3];").unwrap();
  let map = parsed.as_object_ref().unwrap();
  assert_eq!(
    map.get("key"),
    Some(&Value::Array(vec![
      Value::Array(vec![Value::UnsignedIntegerNumber(1), Value::UnsignedIntegerNumber(2)]),
      Value::Array(vec![Value::UnsignedIntegerNumber(3)]),
    ]))
  );
}

/// A key occurring once keeps its own value, so it is still parsed by a
/// non-`Vec` field.
#[test]
fn single_key_is_not_wrapped_for_scalar_fields() {
  let parsed: Value = from_str("key = 1;").unwrap();
  let map = parsed.as_object_ref().unwrap();
  assert_eq!(map.get("key"), Some(&Value::UnsignedIntegerNumber(1)));
}

/// Repeated keys cannot be parsed into a scalar field anymore, for they are
/// collected into an array.
#[derive(Parse, Debug, PartialEq)]
struct Scalar {
  key: String,
}

#[test]
fn repeated_keys_rejected_by_scalar_field() {
  let parsed = from_str::<Scalar>("key = first;\nkey = second;");
  match parsed {
    Err(Error::Cast(CastError::IncompatibleType)) => {},
    other => panic!("expected `IncompatibleType`, got {other:?}"),
  }
}

/// Repeated keys also work on nested objects.
#[derive(Parse, Debug, PartialEq)]
struct Item {
  name: String,
}

#[derive(Parse, Debug, PartialEq)]
struct Items {
  item: Vec<Item>,
}

#[test]
fn repeated_object_keys_are_collected() {
  let data = r###"
item = {
  name = "first";
}
item = {
  name = "second";
}
"###;
  let parsed: Items = from_str(data).unwrap();
  assert_eq!(
    parsed,
    Items { item: vec![Item { name: "first".to_string() }, Item { name: "second".to_string() }] }
  );
}

/// `Option<Vec<T>>` accepts both a missing key and a key occurring once or
/// several times.
#[derive(Parse, Debug, PartialEq)]
struct OptionalTags {
  tag: Option<Vec<String>>,
}

#[test]
fn optional_vec_absent_is_none() {
  let parsed: OptionalTags = from_str("unused = 0;").unwrap();
  assert_eq!(parsed, OptionalTags { tag: None });
}

#[test]
fn optional_vec_accepts_single_key() {
  let parsed: OptionalTags = from_str("tag = rust;").unwrap();
  assert_eq!(parsed, OptionalTags { tag: Some(vec!["rust".to_string()]) });
}

#[test]
fn optional_vec_accepts_repeated_keys() {
  let parsed: OptionalTags = from_str("tag = rust;\ntag = marquage;").unwrap();
  assert_eq!(parsed, OptionalTags { tag: Some(vec!["rust".to_string(), "marquage".to_string()]) });
}

/// `void` is not promoted, so an `Option<Vec<T>>` field still reads it as `None`.
#[test]
fn optional_vec_reads_void_as_none() {
  let parsed: OptionalTags = from_str("tag = void;").unwrap();
  assert_eq!(parsed, OptionalTags { tag: None });
}

/// A `Vec` field without a default still reports a missing key.
#[test]
fn vec_field_without_key_reports_missing_field() {
  let parsed = from_str::<Tags>("unused = 0;");
  match parsed {
    // The field's name is stringified, so it keeps its quotes.
    Err(Error::Cast(CastError::FieldNotFound(field))) => assert_eq!(field, "\"tag\""),
    other => panic!("expected `FieldNotFound`, got {other:?}"),
  }
}

/// `#[marquage(default)]` gives a `Vec` field an empty vector.
#[derive(Parse, Debug, PartialEq)]
struct DefaultedTags {
  #[marquage(default)]
  tag: Vec<String>,
}

#[test]
fn vec_field_defaults_to_empty() {
  let parsed: DefaultedTags = from_str("unused = 0;").unwrap();
  assert_eq!(parsed, DefaultedTags { tag: Vec::new() });
}

/// Named fields of enum variants get the same treatment.
#[derive(Parse, Debug, PartialEq)]
enum Message {
  Tags { tag: Vec<String> },
}

#[test]
fn enum_variant_vec_field_accepts_single_key() {
  let parsed: Message = from_str("tag = rust;").unwrap();
  assert_eq!(parsed, Message::Tags { tag: vec!["rust".to_string()] });
}
