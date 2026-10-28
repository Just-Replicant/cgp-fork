use syn::parse::{Parse, ParseStream};

use crate::macro_core::types::generics::ImplGenerics;
use crate::macro_core::types::path::PathElement;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct PathElementWithGenerics {
    pub generics: ImplGenerics,
    pub element: PathElement,
}

impl Parse for PathElementWithGenerics {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generics = input.parse()?;
        let element = input.parse()?;
        Ok(Self { generics, element })
    }
}
