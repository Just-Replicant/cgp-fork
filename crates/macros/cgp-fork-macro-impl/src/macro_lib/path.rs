use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{Namespace, Pipeline, run};

/// `Path!`: a type-level namespace path.
pub struct PathMacro;

impl Pipeline for PathMacro {
    const NAME: &'static str = "Path";
    type Ir = Namespace;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(Namespace::Path)
    }
}

/// `Path!` entry point.
#[allow(non_snake_case)]
pub fn Path(body: TokenStream) -> syn::Result<TokenStream> {
    run::<PathMacro>(TokenStream::new(), body)
}
