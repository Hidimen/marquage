mod utils;

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{Data, DeriveInput, Error, parse_macro_input, parse_quote, spanned::Spanned};

#[proc_macro_derive(Parseable)]
pub fn parseable_derive(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let name = ast.ident.clone();
  let generic = utils::add_trait_bounds(ast.generics.clone(), parse_quote!(Parseable));
  let (impl_generics, ty_generics, where_clause) = generic.split_for_impl();
  let span = name.span();

  let fields = match &ast.data {
    Data::Struct(data_struct) => {
      match &data_struct.fields {
        syn::Fields::Named(field) => {
          &field.named
        },
        _ => return Error::new_spanned(ast, "Parseable trait is applicable only to named field").to_compile_error().into()
      }
    },
    _ => return Error::new_spanned(ast, "Parseable trait is applicable only to structs").to_compile_error().into()
  };
  let parseable_fields = fields.iter().map(|f| {
    let name = f.ident.as_ref().unwrap();
    let f_span = f.span();

    quote_spanned! { f_span =>
      #name: {
        if let Some(data) = map.swap_remove(stringify!(#name)) {
          ::marquage::Parseable::parse(data)?
        }else{
          return Err(::marquage::error::CastError::FieldNotFound(stringify!(#name).to_string()));
        }
      }
    }
  });

  let expanded = quote_spanned! { span =>
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
  };

  TokenStream::from(expanded)
}