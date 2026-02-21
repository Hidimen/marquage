use syn::{
  Attribute, Error, Expr, GenericParam, Generics, Lit, Meta, TypeParamBound,
  spanned::Spanned,
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
            let span = literal.span();
            return Err(Error::new(
              span,
              "rename field must be a string literal",
            ));
          },
        },
        _ => continue,
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
            Err(Error::new(
              meta.path.span(),
              "expecting `value` field, but found unexpected token",
            ))
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
