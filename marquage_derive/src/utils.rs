use convert_case::{Case, Casing};
use proc_macro2::Span;
use syn::{
  Attribute, Error, Expr, GenericParam, Generics, Lit, Meta, Token, Type, TypeParamBound,
  parse_quote_spanned, punctuated::Punctuated, spanned::Spanned,
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
    Meta::NameValue(nv) if nv.path.is_ident("default") => match &nv.value {
      Expr::Lit(lit) => {
        res = Some(generate_conversion(ty, lit.lit.clone(), lit.span()));
        Ok(())
      },
      _ => Err(Error::new(nv.value.span(), "`default` must be a literal")),
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
