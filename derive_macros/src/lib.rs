mod utils;

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{
  Data, DeriveInput, Error, parse_macro_input, parse_quote, spanned::Spanned,
};

#[proc_macro_derive(Parse, attributes(rename, default, skip))]
pub fn parseable_derive(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let name = ast.ident.clone();
  let generic =
    utils::add_trait_bounds(ast.generics.clone(), parse_quote!(Parseable));
  let (impl_generics, ty_generics, where_clause) = generic.split_for_impl();
  let span = name.span();

  let fields = match &ast.data {
    Data::Struct(data_struct) => match &data_struct.fields {
      syn::Fields::Named(field) => &field.named,
      _ => {
        return Error::new_spanned(
          ast,
          "Parseable trait is applicable only to named field",
        )
        .to_compile_error()
        .into();
      },
    },
    _ => {
      return Error::new_spanned(
        ast,
        "Parseable trait is applicable only to structs",
      )
      .to_compile_error()
      .into();
    },
  };
  let parseable_fields = fields.iter().map(|f|  {
    let name = f.ident.as_ref().unwrap();
    let rename = match utils::get_rename(&f.attrs, name.to_string()) {
      Ok(n) => n,
      Err(e) => return e.to_compile_error()
    };
    let f_span = f.span();
    let ty = &f.ty;

    let raw = match utils::get_default(&f.attrs, ty){
      Ok(e) => e,
      Err(e) => return e.to_compile_error()
    };

    match utils::is_skip(&f.attrs){
      Ok(true) => {
        return quote_spanned! { f_span =>
          #name: {
            <#ty as Default>::default()
          }
        };
      },
      Ok(false) => {/* Do nothing */},
      Err(e) => return e.to_compile_error()
    }

    if let Some(expr) = raw{
      return quote_spanned! {f_span =>
        #name: {
          if let Some(data) = map.swap_remove(#rename) {
            ::marquage::Parseable::parse(data)?
          }else{
            #expr
          }
        }
      }
    }

    quote_spanned! { f_span =>
      #name: {
        if let Some(data) = map.swap_remove(#rename) {
          ::marquage::Parseable::parse(data)?
        }else{
          return Err(::marquage::error::CastError::FieldNotFound(stringify!(#rename).to_string()));
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

#[proc_macro_derive(Generate, attributes(rename, default, skip))]
pub fn generable_derive(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let name = ast.ident.clone();
  let generic =
    utils::add_trait_bounds(ast.generics.clone(), parse_quote!(Generable));
  let (impl_generics, ty_generics, where_clause) = generic.split_for_impl();
  let span = name.span();

  let fields = match &ast.data {
    Data::Struct(data_struct) => match &data_struct.fields {
      syn::Fields::Named(field) => &field.named,
      _ => {
        return Error::new_spanned(
          ast,
          "Generable trait is applicable only to named field",
        )
        .to_compile_error()
        .into();
      },
    },
    _ => {
      return Error::new_spanned(
        ast,
        "Generable trait is applicable only to structs",
      )
      .to_compile_error()
      .into();
    },
  };
  let generable_fields = fields.iter().map(|f| {
    let name = f.ident.as_ref().unwrap();
    let rename = match utils::get_rename(&f.attrs, name.to_string()) {
      Ok(n) => n,
      Err(e) => return e.to_compile_error()
    };
    let f_span = f.span();

    match utils::is_skip(&f.attrs){
      Ok(true) => return proc_macro2::TokenStream::new(),
      Ok(false) => {/* Do nothing */},
      Err(e) => return e.to_compile_error()
    }

    quote_spanned! { f_span =>
      map.insert(#rename.to_string(), ::marquage::Generable::generate(self.#name));
    }
  });

  let generable_ref_fields = fields.iter().map(|f| {
    let name = f.ident.as_ref().unwrap();
    let rename = match utils::get_rename(&f.attrs, name.to_string()) {
      Ok(n) => n,
      Err(e) => return e.to_compile_error()
    };
    let f_span = f.span();

    match utils::is_skip(&f.attrs){
      Ok(true) => return proc_macro2::TokenStream::new(),
      Ok(false) => {/* Do nothing */},
      Err(e) => return e.to_compile_error()
    }

    quote_spanned! { f_span =>
      map.insert(#rename.to_string(), ::marquage::Generable::generate_ref(&self.#name));
    }
  });

  let expanded = quote_spanned! { span =>
    impl #impl_generics ::marquage::Generable for #name #ty_generics #where_clause {
      fn generate(self) -> ::marquage::data::Value{
        ::marquage::data::Value::Object({
          let mut map = indexmap::IndexMap::new();
          #(#generable_fields)*
          map
        })
      }

      fn generate_ref(&self) -> ::marquage::data::Value {
        ::marquage::data::Value::Object({
          let mut map = indexmap::IndexMap::new();
          #(#generable_ref_fields)*
          map
        })
      }
    }
  };

  TokenStream::from(expanded)
}
