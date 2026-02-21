use syn::{GenericParam, Generics, TypeParamBound};

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
