use convert_case::{Case, Casing};
use proc_macro2::Span;
use syn::{
  Attribute, Error, Expr, GenericParam, Generics, Lit, Meta, Type, TypeParamBound,
  parse_quote_spanned, spanned::Spanned,
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

pub fn get_rename(attributes: &[Attribute], ident_name: String) -> Result<String, Error> {
  for attribute in attributes {
    if !attribute.path().is_ident("rename") {
      continue;
    }

    match &attribute.meta {
      Meta::NameValue(name_value) => match &name_value.value {
        Expr::Lit(literal) => match &literal.lit {
          Lit::Str(s) => {
            return Ok(s.value());
          },
          _ => {
            return Err(Error::new(literal.span(), "expect string literal, but found others"));
          },
        },
        _ => {
          return Err(Error::new(
            name_value.value.span(),
            "`rename` field only accept a string literal",
          ));
        },
      },
      Meta::List(meta_list) => {
        let mut res: Option<String> = None;
        meta_list.parse_nested_meta(|meta| {
          if meta.path.is_ident("value") {
            let val: Lit = meta.value()?.parse()?;
            match &val {
              Lit::Str(s) => {
                res = Some(s.value());
                Ok(())
              },
              _ => Err(Error::new(meta.path.span(), "`rename` field must be a string literal")),
            }
          } else {
            Err(Error::new(meta.path.span(), "expecting `value` field"))
          }
        })?;

        return res.ok_or_else(|| Error::new(meta_list.span(), "no field in `rename`"));
      },
      _ => continue,
    }
  }

  Ok(ident_name)
}

/// Parse the `rename_all` attribute into a case converter.
///
/// Supported rules: `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`,
/// `kebab-case`, `PascalCase`, `SCREAMING_SNAKE_CASE`.
pub fn get_rename_all(attributes: &[Attribute]) -> Result<Option<Case<'static>>, Error> {
  for attribute in attributes {
    if !attribute.path().is_ident("rename_all") {
      continue;
    }

    match &attribute.meta {
      Meta::NameValue(name_value) => match &name_value.value {
        Expr::Lit(literal) => match &literal.lit {
          Lit::Str(s) => {
            return Ok(Some(parse_rename_all(&s.value(), s.span())?));
          },
          _ => {
            return Err(Error::new(literal.span(), "expect string literal, but found others"));
          },
        },
        _ => {
          return Err(Error::new(
            name_value.value.span(),
            "`rename_all` field only accept a string literal",
          ));
        },
      },
      Meta::List(meta_list) => {
        let mut res: Option<Case<'static>> = None;
        meta_list.parse_nested_meta(|meta| {
          if meta.path.is_ident("value") {
            let val: Lit = meta.value()?.parse()?;
            match &val {
              Lit::Str(s) => {
                res = Some(parse_rename_all(&s.value(), s.span())?);
                Ok(())
              },
              _ => Err(Error::new(meta.path.span(), "`rename_all` field must be a string literal")),
            }
          } else {
            Err(Error::new(meta.path.span(), "expecting `value` field"))
          }
        })?;

        return Ok(Some(
          res.ok_or_else(|| Error::new(meta_list.span(), "no field in `rename_all`"))?,
        ));
      },
      _ => {
        return Err(Error::new(
          attribute.meta.span(),
          "`rename_all` field only accept a string literal",
        ));
      },
    }
  }

  Ok(None)
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
/// An explicit `#[rename = "..."]` takes precedence. Otherwise, if a
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
  for attribute in attributes {
    if !attribute.path().is_ident("default") {
      continue;
    }

    match &attribute.meta {
      Meta::NameValue(name_value) => match &name_value.value {
        Expr::Lit(lit) => {
          return Ok(Some(generate_conversion(ty, lit.lit.clone(), lit.span())));
        },
        _ => {
          return Err(Error::new(name_value.value.span(), "#[default = ...] only accept literal"));
        },
      },
      Meta::List(meta_list) => {
        let mut res: Option<Expr> = None;
        meta_list.parse_nested_meta(|meta| {
          if meta.path.is_ident("value") {
            let val: Expr = match meta.value()?.parse()? {
              Expr::Lit(lit) => generate_conversion(ty, lit.lit.clone(), lit.span()),
              _ => {
                return Err(Error::new(
                  meta.value()?.span(),
                  "#[default(value = ...)] only accept literal",
                ));
              },
            };
            res = Some(val);
            Ok(())
          } else {
            Err(Error::new(meta.path.span(), "expecting `value` field"))
          }
        })?;

        return Ok(res);
      },
      Meta::Path(path) => {
        let span = path.span();
        return Ok(Some(parse_quote_spanned! {span => {
          <#ty as Default>::default()
        }}));
      },
    }
  }

  Ok(None)
}

pub fn is_skip(attributes: &[Attribute]) -> Result<bool, Error> {
  for attribute in attributes {
    if !attribute.path().is_ident("skip") {
      continue;
    }

    match &attribute.meta {
      Meta::Path(_) => return Ok(true),
      _ => {
        return Err(Error::new(attribute.meta.span(), "#[skip] receive nothing"));
      },
    }
  }

  Ok(false)
}

fn generate_conversion(ty: &Type, lit: Lit, span: Span) -> Expr {
  if let Type::Reference(r) = ty
    && r.mutability.is_none()
    && let Type::Path(p) = &*r.elem
    && p.path.is_ident("str")
  {
    return parse_quote_spanned! {span => #lit};
  }

  parse_quote_spanned! {span => <#ty>::from(#lit)}
}
