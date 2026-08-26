use convert_case::{Case, Casing};
use proc_macro2::Span;
use syn::{
  Attribute, Error, Expr, GenericParam, Generics, Lit, LitFloat, LitInt, Meta, Token, Type,
  TypeParamBound, UnOp, parse_quote_spanned, punctuated::Punctuated, spanned::Spanned,
};

pub fn add_trait_bounds(mut generics: Generics, bound: TypeParamBound) -> Generics {
  for param in &mut generics.params {
    if let GenericParam::Type(type_param) = param {
      type_param.bounds.push(bound.clone());
    }
  }
  generics
}

/// Check if a type is `Option<T>`.
///
/// Only the last path segment is matched, so fully-qualified paths like
/// `std::option::Option<T>` are recognized as well.
pub fn is_option(ty: &Type) -> bool {
  match ty {
    Type::Path(path) if path.qself.is_none() => {
      path.path.segments.last().is_some_and(|seg| seg.ident == "Option")
    },
    _ => false,
  }
}

/// Invoke `logic` for every nested meta inside every `#[marquage(...)]` attribute.
///
/// A `#[marquage(...)]` attribute must be a list. Keys not handled by `logic`
/// are simply ignored, which allows multiple directives (e.g. `rename`, `skip`,
/// `default`) to coexist in a single attribute.
fn each_marquage_meta(
  attributes: &[Attribute], mut logic: impl FnMut(&Meta) -> Result<(), Error>,
) -> Result<(), Error> {
  for attribute in attributes {
    if !attribute.path().is_ident("marquage") {
      continue;
    }

    let meta_list = match &attribute.meta {
      Meta::List(meta_list) => meta_list,
      _ => {
        return Err(Error::new(attribute.meta.span(), "`marquage` field only accept a list"));
      },
    };

    let metas: Punctuated<Meta, Token![,]> =
      meta_list.parse_args_with(Punctuated::parse_terminated)?;

    for meta in metas {
      logic(&meta)?;
    }
  }

  Ok(())
}

pub fn get_rename(attributes: &[Attribute], ident_name: String) -> Result<String, Error> {
  let mut res: Option<String> = None;
  each_marquage_meta(attributes, |meta| match meta {
    Meta::NameValue(nv) if nv.path.is_ident("rename") => match &nv.value {
      Expr::Lit(lit) => match &lit.lit {
        Lit::Str(s) => {
          res = Some(s.value());
          Ok(())
        },
        _ => Err(Error::new(lit.span(), "`rename` must be a string literal")),
      },
      _ => Err(Error::new(nv.value.span(), "`rename` must be a string literal")),
    },
    _ => Ok(()),
  })?;

  Ok(res.unwrap_or(ident_name))
}

/// Parse the `rename_all` directive inside `#[marquage(...)]` into a case converter.
///
/// Supported rules: `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`,
/// `kebab-case`, `PascalCase`, `SCREAMING_SNAKE_CASE`.
pub fn get_rename_all(attributes: &[Attribute]) -> Result<Option<Case<'static>>, Error> {
  let mut res: Option<Case<'static>> = None;
  each_marquage_meta(attributes, |meta| match meta {
    Meta::NameValue(nv) if nv.path.is_ident("rename_all") => match &nv.value {
      Expr::Lit(lit) => match &lit.lit {
        Lit::Str(s) => {
          res = Some(parse_rename_all(&s.value(), s.span())?);
          Ok(())
        },
        _ => Err(Error::new(lit.span(), "`rename_all` must be a string literal")),
      },
      _ => Err(Error::new(nv.value.span(), "`rename_all` must be a string literal")),
    },
    _ => Ok(()),
  })?;

  Ok(res)
}

fn parse_rename_all(rule: &str, span: Span) -> Result<Case<'static>, Error> {
  let case = match rule {
    "lowercase" => Case::Flat,
    "UPPERCASE" => Case::UpperFlat,
    "camelCase" => Case::Camel,
    "snake_case" => Case::Snake,
    "kebab-case" => Case::Kebab,
    "PascalCase" => Case::Pascal,
    "SCREAMING_SNAKE_CASE" => Case::UpperSnake,
    _ => return Err(Error::new(span, format!("unknown rename rule: `{rule}`"))),
  };

  Ok(case)
}

/// Get the serialized name of a field or variant.
///
/// An explicit `#[marquage(rename = "...")]` takes precedence. Otherwise, if a
/// `rename_all` case is provided, the identifier is converted into that case;
/// otherwise the identifier is used as-is.
pub fn get_rename_with(
  attributes: &[Attribute], ident_name: &str, rename_all: Option<Case<'static>>,
) -> Result<String, Error> {
  let fallback = match rename_all {
    Some(case) => ident_name.to_case(case),
    None => ident_name.to_string(),
  };
  get_rename(attributes, fallback)
}

pub fn get_default(attributes: &[Attribute], ty: &Type) -> Result<Option<Expr>, Error> {
  let mut res: Option<Expr> = None;
  each_marquage_meta(attributes, |meta| match meta {
    Meta::NameValue(nv) if nv.path.is_ident("default") => {
      let lit = match &nv.value {
        Expr::Lit(lit) => lit.lit.clone(),
        // `default = -1` parses as a unary negation rather than a literal;
        // fold it back into a negative literal so the type-aware conversion
        // below is applied to the value as a whole.
        Expr::Unary(unary) if matches!(unary.op, UnOp::Neg(_)) => match &*unary.expr {
          Expr::Lit(lit) if matches!(lit.lit, Lit::Int(_) | Lit::Float(_)) => {
            negate_literal(lit.lit.clone(), unary.span())
          },
          _ => {
            return Err(Error::new(
              nv.value.span(),
              "`default` must be a numeric literal",
            ));
          },
        },
        _ => return Err(Error::new(nv.value.span(), "`default` must be a literal")),
      };
      res = Some(generate_conversion(ty, lit, nv.value.span()));
      Ok(())
    },
    Meta::Path(path) if path.is_ident("default") => {
      let span = path.span();
      res = Some(parse_quote_spanned! {span => {
        <#ty as Default>::default()
      }});
      Ok(())
    },
    _ => Ok(()),
  })?;

  Ok(res)
}

pub fn is_skip(attributes: &[Attribute]) -> Result<bool, Error> {
  let mut skip = false;
  each_marquage_meta(attributes, |meta| {
    if let Meta::Path(path) = meta
      && path.is_ident("skip")
    {
      skip = true;
    }
    Ok(())
  })?;

  Ok(skip)
}

/// Generate the default-value expression for a field of type `ty` from a literal.
///
/// Primitive numeric, `bool` and `char` fields keep the literal as-is: its type
/// is then inferred from the field, or taken from the suffix the user wrote.
/// Wrapping such literals in `<#ty>::from(#lit)` would force unsuffixed
/// integer/float literals to default to `i32`/`f64`, which breaks fields like
/// `usize` (`usize: From<i32>` is not implemented) or `f32` (`f32: From<f64>`
/// is not implemented), and would trip `clippy::useless_conversion` when the
/// literal already carries the field type's suffix (e.g. `1024_usize`).
///
/// Everything else falls back to `<#ty>::from(#lit)`, e.g. `String` from a
/// string literal, or a `u64` field from an explicitly `u32`-suffixed literal.
fn generate_conversion(ty: &Type, lit: Lit, span: Span) -> Expr {
  // `&str` fields keep the literal as-is.
  if let Type::Reference(r) = ty
    && r.mutability.is_none()
    && let Type::Path(p) = &*r.elem
    && p.path.is_ident("str")
  {
    return parse_quote_spanned! {span => #lit};
  }

  if let Some(suffix) = primitive_numeric_suffix(ty) {
    if matches!(lit, Lit::Int(_) | Lit::Float(_)) {
      let lit_suffix = match &lit {
        Lit::Int(int) => int.suffix(),
        Lit::Float(float) => float.suffix(),
        _ => unreachable!(),
      };
      // A literal with a different explicit suffix still needs a conversion,
      // e.g. `default = 1024_u32` on a `u64` field keeps `<u64>::from(1024_u32)`.
      if lit_suffix.is_empty() || lit_suffix == suffix {
        return parse_quote_spanned! {span => #lit};
      }
    }
  } else if (is_primitive_ident(ty, "bool") && matches!(lit, Lit::Bool(_)))
    || (is_primitive_ident(ty, "char") && matches!(lit, Lit::Char(_)))
  {
    return parse_quote_spanned! {span => #lit};
  }

  parse_quote_spanned! {span => <#ty>::from(#lit)}
}

/// Check if `ty` is a plain (unqualified, argument-free) path of the given ident.
fn is_primitive_ident(ty: &Type, ident: &str) -> bool {
  matches!(ty, Type::Path(path) if path.qself.is_none() && path.path.is_ident(ident))
}

/// The literal suffix of a primitive numeric type (`i8`..`i128`, `isize`,
/// `u8`..`u128`, `usize`, `f32`, `f64`), if `ty` is one of them.
fn primitive_numeric_suffix(ty: &Type) -> Option<&'static str> {
  const SUFFIXES: [&str; 14] = [
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32",
    "f64",
  ];
  SUFFIXES
    .iter()
    .copied()
    .find(|&suffix| is_primitive_ident(ty, suffix))
}

/// Rebuild a numeric literal with a leading minus sign, e.g. `1` -> `-1`.
fn negate_literal(lit: Lit, span: Span) -> Lit {
  match lit {
    Lit::Int(int) => Lit::Int(LitInt::new(&format!("-{int}"), span)),
    Lit::Float(float) => Lit::Float(LitFloat::new(&format!("-{float}"), span)),
    _ => lit,
  }
}
