use marquage::{Parse, from_str};

/// Unsuffixed integer literals must be typed by the field, not defaulted to
/// `i32` (which made `usize`, `u8`, ... fields fail to compile before).
#[derive(Parse, Debug, PartialEq)]
struct UnsuffixedInts {
  #[marquage(default = 1024)]
  count: usize,
  #[marquage(default = 42)]
  tiny: u8,
  #[marquage(default = 7)]
  seven: i32,
  #[marquage(default = 1000000000000)]
  big: u64,
}

#[test]
fn unsuffixed_int_defaults_are_typed_by_the_field() {
  let data = r###"
count = 5;
"###;
  let parsed: UnsuffixedInts = from_str(data).unwrap();
  assert_eq!(
    parsed,
    UnsuffixedInts { count: 5, tiny: 42, seven: 7, big: 1000000000000 }
  );

  let parsed: UnsuffixedInts = from_str("unused = 0;").unwrap();
  assert_eq!(
    parsed,
    UnsuffixedInts { count: 1024, tiny: 42, seven: 7, big: 1000000000000 }
  );
}

/// Literals already suffixed with the field type must not be wrapped in a
/// redundant `From` conversion (previously `clippy::useless_conversion`).
#[derive(Parse, Debug, PartialEq)]
struct Suffixed {
  #[marquage(default = 1024_usize)]
  count: usize,
  #[marquage(default = 1_u8)]
  tiny: u8,
  #[marquage(default = 0.5_f64)]
  ratio: f64,
}

#[test]
fn suffixed_defaults_matching_the_field_type_work() {
  let parsed: Suffixed = from_str("unused = 0;").unwrap();
  assert_eq!(parsed, Suffixed { count: 1024, tiny: 1, ratio: 0.5 });
}

/// A literal with a *different* explicit suffix still converts, e.g.
/// `u64: From<u32>`.
#[derive(Parse, Debug, PartialEq)]
struct FromConversion {
  #[marquage(default = 1024_u32)]
  wide: u64,
}

#[test]
fn mismatched_suffix_defaults_still_convert() {
  let parsed: FromConversion = from_str("unused = 0;").unwrap();
  assert_eq!(parsed, FromConversion { wide: 1024 });
}

/// Unsuffixed float literals default to `f64`, so `f32` fields failed before
/// (`f32: From<f64>` is not implemented).
#[derive(Parse, Debug, PartialEq)]
struct Floats {
  #[marquage(default = 0.5)]
  a: f32,
  #[marquage(default = 0.25)]
  b: f64,
}

#[test]
fn float_defaults_are_typed_by_the_field() {
  let parsed: Floats = from_str("unused = 0;").unwrap();
  assert_eq!(parsed, Floats { a: 0.5, b: 0.25 });
}

/// `default = -1` parses as a unary negation and must be accepted for signed
/// integer/float fields.
#[derive(Parse, Debug, PartialEq)]
struct Negatives {
  #[marquage(default = -1)]
  a: i32,
  #[marquage(default = -128)]
  b: i8,
  #[marquage(default = -0.5)]
  c: f32,
  #[marquage(default = -2_i64)]
  d: i64,
}

#[test]
fn negative_defaults_work() {
  let parsed: Negatives = from_str("unused = 0;").unwrap();
  assert_eq!(parsed, Negatives { a: -1, b: -128, c: -0.5, d: -2 });
}

#[derive(Parse, Debug, PartialEq)]
struct Flags {
  #[marquage(default = true)]
  on: bool,
}

#[test]
fn bool_default_works() {
  let parsed: Flags = from_str("unused = 0;").unwrap();
  assert_eq!(parsed, Flags { on: true });
}

#[derive(Parse, Debug, PartialEq)]
struct Text {
  #[marquage(default = "hello")]
  greeting: String,
}

#[test]
fn string_default_works() {
  let parsed: Text = from_str("unused = 0;").unwrap();
  assert_eq!(parsed, Text { greeting: "hello".to_string() });
}

#[derive(Parse, Debug, PartialEq)]
struct PathDefault {
  #[marquage(default)]
  count: u32,
}

#[test]
fn bare_default_uses_trait_default() {
  let parsed: PathDefault = from_str("unused = 0;").unwrap();
  assert_eq!(parsed, PathDefault { count: 0 });
}

/// Defaults also apply to named fields of enum variants.
#[derive(Parse, Debug, PartialEq)]
enum Message {
  Text {
    #[marquage(default = "hi")]
    body: String,
    #[marquage(default = 0)]
    priority: u8,
  },
}

#[test]
fn enum_variant_defaults_work() {
  let data = r###"
body = "x";
"###;
  let parsed: Message = from_str(data).unwrap();
  assert_eq!(parsed, Message::Text { body: "x".to_string(), priority: 0 });
}
