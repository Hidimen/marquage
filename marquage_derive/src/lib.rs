mod utils;

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{Data, DeriveInput, Error, parse_macro_input, parse_quote, spanned::Spanned};

/// A derive macro that automatically implement `Parseable` for a struct or enum.
#[proc_macro_derive(Parse, attributes(marquage))]
pub fn parseable_derive(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let name = ast.ident.clone();
  let generic = utils::add_trait_bounds(ast.generics.clone(), parse_quote!(Parseable));
  let (impl_generics, ty_generics, where_clause) = generic.split_for_impl();
  let span = name.span();

  let rename_all = match utils::get_rename_all(&ast.attrs) {
    Ok(c) => c,
    Err(e) => return e.to_compile_error().into(),
  };

  let expanded = match &ast.data {
    Data::Struct(data_struct) => {
      let fields = match &data_struct.fields {
        syn::Fields::Named(field) => &field.named,
        _ => {
          return Error::new_spanned(ast, "Parseable trait is applicable only to named field")
            .to_compile_error()
            .into();
        },
      };

      let parseable_fields: Vec<_> = fields
        .iter()
        .map(|f| {
          let name = f.ident.as_ref().unwrap();
          let rename = match utils::get_rename_with(&f.attrs, &name.to_string(), rename_all) {
            Ok(n) => n,
            Err(e) => return e.to_compile_error(),
          };
          let f_span = f.span();

          let expr = match generate_field_parse(f, &rename) {
            Ok(e) => e,
            Err(e) => return e.to_compile_error(),
          };

          quote_spanned! { f_span =>
            #name: #expr
          }
        })
        .collect();

      if parseable_fields.is_empty() {
        quote_spanned! { span =>
          impl #impl_generics ::marquage::Parseable for #name #ty_generics #where_clause {
            fn parse(v: ::marquage::data::Value) -> Result<Self, ::marquage::error::CastError>{
              match v {
                ::marquage::data::Value::Object(_) => {
                  Ok(
                    Self {}
                  )
                },
                _ => Err(::marquage::error::CastError::IncompatibleType)
              }
            }
          }
        }
      } else {
        quote_spanned! { span =>
          impl #impl_generics ::marquage::Parseable for #name #ty_generics #where_clause {
            fn parse(v: ::marquage::data::Value) -> Result<Self, ::marquage::error::CastError>{
              match v {
                ::marquage::data::Value::Object(mut map) => {
                  Ok(
                    Self {
                      #(#parseable_fields),*
                    }
                  )
                },
                _ => Err(::marquage::error::CastError::IncompatibleType)
              }
            }
          }
        }
      }
    },
    Data::Enum(data_enum) => {
      let mut unit_patterns = Vec::new();
      let mut unnamed_attempts = Vec::new();
      let mut named_attempts = Vec::new();

      for v in &data_enum.variants {
        let variant_name = &v.ident;
        let v_span = v.span();
        let variant_rename =
          match utils::get_rename_with(&v.attrs, &v.ident.to_string(), rename_all) {
            Ok(n) => n,
            Err(e) => return e.to_compile_error().into(),
          };

        match &v.fields {
          syn::Fields::Unit => {
            unit_patterns.push(quote_spanned! { v_span =>
              #variant_rename => Ok(Self::#variant_name),
            });
          },
          syn::Fields::Unnamed(fields) => {
            let field_count = fields.unnamed.len();
            let bind_idents: Vec<_> = (0..field_count)
              .map(|i| syn::Ident::new(&format!("f{i}"), proc_macro2::Span::call_site()))
              .collect();
            let field_lets: Vec<_> = fields
              .unnamed
              .iter()
              .zip(&bind_idents)
              .map(|(f, bind)| {
                let ty = &f.ty;
                quote_spanned! { f.span() =>
                  let #bind = match __iter.next() {
                    Some(v) => <#ty as ::marquage::Parseable>::parse(v.clone())?,
                    None => return Err(::marquage::error::CastError::IncompatibleType),
                  };
                }
              })
              .collect();

            unnamed_attempts.push(quote_spanned! { v_span =>
              if arr.len() == #field_count {
                let __result: Result<Self, ::marquage::error::CastError> = (|| {
                  let mut __iter = arr.iter();
                  #(#field_lets)*
                  Ok(Self::#variant_name(#(#bind_idents),*))
                })();
                if let Ok(__value) = __result {
                  return Ok(__value);
                }
              }
            });
          },
          syn::Fields::Named(fields) => {
            let field_inits: Vec<_> = fields
              .named
              .iter()
              .map(|f| {
                let f_ident = f.ident.as_ref().unwrap();
                let rename = match utils::get_rename(&f.attrs, f_ident.to_string()) {
                  Ok(n) => n,
                  Err(e) => return e.to_compile_error(),
                };

                let expr = match generate_field_parse(f, &rename) {
                  Ok(e) => e,
                  Err(e) => return e.to_compile_error(),
                };

                quote_spanned! { f.span() =>
                  #f_ident: #expr
                }
              })
              .collect();

            named_attempts.push(quote_spanned! { v_span =>
              {
                let __result: Result<Self, ::marquage::error::CastError> = (|| {
                  let mut map = map.clone();
                  Ok(Self::#variant_name {
                    #(#field_inits),*
                  })
                })();
                if let Ok(__value) = __result {
                  return Ok(__value);
                }
              }
            });
          },
        }
      }

      let string_arm = if unit_patterns.is_empty() {
        quote_spanned! { span => }
      } else {
        quote_spanned! { span =>
          ::marquage::data::Value::RawString(s) | ::marquage::data::Value::QuotedString(s) => {
            match s.as_str() {
              #(#unit_patterns)*
              _ => Err(::marquage::error::CastError::IncompatibleType),
            }
          },
        }
      };
      let array_arm = if unnamed_attempts.is_empty() {
        quote_spanned! { span => }
      } else {
        quote_spanned! { span =>
          ::marquage::data::Value::Array(arr) => {
            #(#unnamed_attempts)*
            Err(::marquage::error::CastError::IncompatibleType)
          },
        }
      };
      let object_arm = if named_attempts.is_empty() {
        quote_spanned! { span => }
      } else {
        quote_spanned! { span =>
          ::marquage::data::Value::Object(map) => {
            #(#named_attempts)*
            Err(::marquage::error::CastError::IncompatibleType)
          },
        }
      };

      quote_spanned! { span =>
        impl #impl_generics ::marquage::Parseable for #name #ty_generics #where_clause {
          fn parse(v: ::marquage::data::Value) -> Result<Self, ::marquage::error::CastError> {
            match v {
              #string_arm
              #array_arm
              #object_arm
              _ => Err(::marquage::error::CastError::IncompatibleType),
            }
          }
        }
      }
    },
    _ => {
      return Error::new_spanned(ast, "Parseable trait is applicable only to structs or enums")
        .to_compile_error()
        .into();
    },
  };

  TokenStream::from(expanded)
}

/// Generate the expression that parses a single field out of a map.
///
/// The generated expression expects a local `map` to be in scope and evaluates
/// to the parsed field value (falling back to a default when the field is
/// absent and one is configured).
fn generate_field_parse(f: &syn::Field, rename: &str) -> Result<proc_macro2::TokenStream, Error> {
  let ty = &f.ty;
  let f_span = f.span();

  match utils::is_skip(&f.attrs) {
    Ok(true) => {
      return Ok(quote_spanned! { f_span =>
        <#ty as Default>::default()
      });
    },
    Ok(false) => { /* Do nothing */ },
    Err(e) => return Err(e),
  }

  if let Some(expr) = utils::get_default(&f.attrs, ty)? {
    return Ok(quote_spanned! { f_span =>
      if let Some(data) = map.swap_remove(#rename) {
        ::marquage::Parseable::parse(data)?
      }else{
        #expr
      }
    });
  }

  if utils::is_option(ty) {
    return Ok(quote_spanned! { f_span =>
      if let Some(data) = map.swap_remove(#rename) {
        ::marquage::Parseable::parse(data)?
      }else{
        None
      }
    });
  }

  Ok(quote_spanned! { f_span =>
    if let Some(data) = map.swap_remove(#rename) {
      ::marquage::Parseable::parse(data)?
    }else{
      return Err(::marquage::error::CastError::FieldNotFound(stringify!(#rename).to_string()));
    }
  })
}

/// A derive macro that automatically implement `Generable` for a struct.
#[proc_macro_derive(Generate, attributes(marquage))]
pub fn generable_derive(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let name = ast.ident.clone();
  let generic = utils::add_trait_bounds(ast.generics.clone(), parse_quote!(Generable));
  let (impl_generics, ty_generics, where_clause) = generic.split_for_impl();
  let span = name.span();

  let rename_all = match utils::get_rename_all(&ast.attrs) {
    Ok(c) => c,
    Err(e) => return e.to_compile_error().into(),
  };

  match &ast.data {
    Data::Struct(data_struct) => match &data_struct.fields {
      syn::Fields::Named(fields) => {
        let fields = &fields.named;

        let generable_fields: Vec<_> = fields
          .iter()
          .map(|f| {
            let name = f.ident.as_ref().unwrap();
            let rename = match utils::get_rename_with(&f.attrs, &name.to_string(), rename_all) {
              Ok(n) => n,
              Err(e) => return e.to_compile_error(),
            };
            let f_span = f.span();

            match utils::is_skip(&f.attrs) {
              Ok(true) => return proc_macro2::TokenStream::new(),
              Ok(false) => { /* Do nothing */ },
              Err(e) => return e.to_compile_error(),
            }

            quote_spanned! { f_span =>
              map.insert(#rename.to_string(), ::marquage::Generable::generate(self.#name));
            }
          })
          .collect();

        let generable_ref_fields: Vec<_> = fields
          .iter()
          .map(|f| {
            let name = f.ident.as_ref().unwrap();
            let rename = match utils::get_rename_with(&f.attrs, &name.to_string(), rename_all) {
              Ok(n) => n,
              Err(e) => return e.to_compile_error(),
            };
            let f_span = f.span();

            match utils::is_skip(&f.attrs) {
              Ok(true) => return proc_macro2::TokenStream::new(),
              Ok(false) => { /* Do nothing */ },
              Err(e) => return e.to_compile_error(),
            }

            quote_spanned! { f_span =>
              map.insert(#rename.to_string(), ::marquage::Generable::generate_ref(&self.#name));
            }
          })
          .collect();

        let expanded = if generable_fields.is_empty() || generable_ref_fields.is_empty() {
          quote_spanned! { span =>
            impl #impl_generics ::marquage::Generable for #name #ty_generics #where_clause {
              fn generate(self) -> ::marquage::data::Value{
                ::marquage::data::Value::Object({
                  ::marquage::Map::new()
                })
              }

              fn generate_ref(&self) -> ::marquage::data::Value {
                ::marquage::data::Value::Object({
                  ::marquage::Map::new()
                })
              }
            }
          }
        } else {
          quote_spanned! { span =>
            impl #impl_generics ::marquage::Generable for #name #ty_generics #where_clause {
              fn generate(self) -> ::marquage::data::Value{
                ::marquage::data::Value::Object({
                  let mut map = ::marquage::Map::new();
                  #(#generable_fields)*
                  map
                })
              }

              fn generate_ref(&self) -> ::marquage::data::Value {
                ::marquage::data::Value::Object({
                  let mut map = ::marquage::Map::new();
                  #(#generable_ref_fields)*
                  map
                })
              }
            }
          }
        };

        TokenStream::from(expanded)
      },
      fields => {
        syn::Error::new(fields.span(), "Only named fields are supported").to_compile_error().into()
      },
    },
    Data::Enum(data_enum) => {
      let generate_arms: Vec<_> = data_enum
        .variants
        .iter()
        .map(|v| {
          let variant_name = &v.ident;
          let v_span = v.span();
          let variant_rename =
            match utils::get_rename_with(&v.attrs, &v.ident.to_string(), rename_all) {
              Ok(n) => n,
              Err(e) => return e.to_compile_error(),
            };

          match &v.fields {
            syn::Fields::Unit => {
              quote_spanned! { v_span =>
                Self::#variant_name => ::marquage::data::Value::RawString(#variant_rename.to_string()),
              }
            },
            syn::Fields::Unnamed(fields) => {
              let bind_idents: Vec<_> = (0..fields.unnamed.len())
                .map(|i| syn::Ident::new(&format!("f{i}"), proc_macro2::Span::call_site()))
                .collect();
              quote_spanned! { v_span =>
                Self::#variant_name(#(#bind_idents),*) => ::marquage::data::Value::Array(vec![
                  #(::marquage::Generable::generate(#bind_idents)),*
                ]),
              }
            },
            syn::Fields::Named(fields) => {
              let mut bound = Vec::new();
              let mut insertions = Vec::new();
              for f in &fields.named {
                let f_ident = f.ident.as_ref().unwrap();
                let f_span = f.span();

                match utils::is_skip(&f.attrs) {
                  Ok(true) => continue,
                  Ok(false) => { /* Do nothing */ },
                  Err(e) => return e.to_compile_error(),
                }

                let rename = match utils::get_rename(&f.attrs, f_ident.to_string()) {
                  Ok(n) => n,
                  Err(e) => return e.to_compile_error(),
                };

                bound.push(f_ident.clone());
                insertions.push(quote_spanned! { f_span =>
                  map.insert(#rename.to_string(), ::marquage::Generable::generate(#f_ident));
                });
              }

              quote_spanned! { v_span =>
                Self::#variant_name { #(#bound,)* .. } => ::marquage::data::Value::Object({
                  let mut map = ::marquage::Map::new();
                  #(#insertions)*
                  map
                }),
              }
            },
          }
        })
        .collect();

      let generate_ref_arms: Vec<_> = data_enum
        .variants
        .iter()
        .map(|v| {
          let variant_name = &v.ident;
          let v_span = v.span();
          let variant_rename =
            match utils::get_rename_with(&v.attrs, &v.ident.to_string(), rename_all) {
              Ok(n) => n,
              Err(e) => return e.to_compile_error(),
            };

          match &v.fields {
            syn::Fields::Unit => {
              quote_spanned! { v_span =>
                Self::#variant_name => ::marquage::data::Value::RawString(#variant_rename.to_string()),
              }
            },
            syn::Fields::Unnamed(fields) => {
              let bind_idents: Vec<_> = (0..fields.unnamed.len())
                .map(|i| syn::Ident::new(&format!("f{i}"), proc_macro2::Span::call_site()))
                .collect();
              quote_spanned! { v_span =>
                Self::#variant_name(#(#bind_idents),*) => ::marquage::data::Value::Array(vec![
                  #(::marquage::Generable::generate_ref(#bind_idents)),*
                ]),
              }
            },
            syn::Fields::Named(fields) => {
              let mut bound = Vec::new();
              let mut insertions = Vec::new();
              for f in &fields.named {
                let f_ident = f.ident.as_ref().unwrap();
                let f_span = f.span();

                match utils::is_skip(&f.attrs) {
                  Ok(true) => continue,
                  Ok(false) => { /* Do nothing */ },
                  Err(e) => return e.to_compile_error(),
                }

                let rename = match utils::get_rename(&f.attrs, f_ident.to_string()) {
                  Ok(n) => n,
                  Err(e) => return e.to_compile_error(),
                };

                bound.push(f_ident.clone());
                insertions.push(quote_spanned! { f_span =>
                  map.insert(#rename.to_string(), ::marquage::Generable::generate_ref(#f_ident));
                });
              }

              quote_spanned! { v_span =>
                Self::#variant_name { #(#bound,)* .. } => ::marquage::data::Value::Object({
                  let mut map = ::marquage::Map::new();
                  #(#insertions)*
                  map
                }),
              }
            },
          }
        })
        .collect();

      let expanded = quote_spanned! { span =>
        impl #impl_generics ::marquage::Generable for #name #ty_generics #where_clause {
          fn generate(self) -> ::marquage::data::Value {
            match self {
              #(#generate_arms)*
            }
          }

          fn generate_ref(&self) -> ::marquage::data::Value {
            match self {
              #(#generate_ref_arms)*
            }
          }
        }
      };

      TokenStream::from(expanded)
    },
    _ => syn::Error::new(ast.span(), "Generable trait is applicable only to structs or enums")
      .to_compile_error()
      .into(),
  }
}
