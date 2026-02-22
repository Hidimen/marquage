use proc_macro2::Span;
use syn::{
  Attribute, Error, Expr, GenericParam, Generics, Lit, Meta, Type,
  TypeParamBound, parse_quote_spanned, spanned::Spanned,
};

pub fn add_trait_bounds(
  mut generics: Generics, bound: TypeParamBound,
) -> Generics {
  for param in &mut generics.params {
    if let GenericParam::Type(type_param) = param {
      type_param.bounds.push(bound.clone());
    }
  }
  generics
}

pub fn get_rename(
  attributes: &[Attribute], ident_name: String,
) -> Result<String, Error> {
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
            return Err(Error::new(
              literal.span(),
              "expect string literal, but found others",
            ));
          },
        },
        _ => {
          return Err(Error::new(
            name_value.value.span(),
            "rename field only accept a string literal",
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
              _ => Err(Error::new(
                meta.path.span(),
                "rename field must be a string literal",
              )),
            }
          } else {
            Err(Error::new(meta.path.span(), "expecting `value` field"))
          }
        })?;

        return res
          .ok_or_else(|| Error::new(meta_list.span(), "no field in `rename`"));
      },
      _ => continue,
    }
  }

  Ok(ident_name)
}

pub fn get_default(
  attributes: &[Attribute], ty: &Type,
) -> Result<Option<Expr>, Error> {
  for attribute in attributes {
    if !attribute.path().is_ident("default") {
      continue;
    }

    match &attribute.meta {
      Meta::NameValue(name_value) => match &name_value.value {
        Expr::Lit(lit) => {
          return Ok(Some(generate_conversion(
            ty,
            lit.lit.clone(),
            lit.span(),
          )));
        },
        _ => {
          return Err(Error::new(
            name_value.value.span(),
            "#[default = ...] only accept literal",
          ));
        },
      },
      Meta::List(meta_list) => {
        let mut res: Option<Expr> = None;
        meta_list.parse_nested_meta(|meta| {
          if meta.path.is_ident("value") {
            let val: Expr = match meta.value()?.parse()? {
              Expr::Lit(lit) => {
                generate_conversion(ty, lit.lit.clone(), lit.span())
              },
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
