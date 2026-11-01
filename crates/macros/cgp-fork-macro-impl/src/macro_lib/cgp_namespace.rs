use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{Namespace, Pipeline, run};

/// `cgp_namespace!`: a namespace and the components it redirects.
pub struct CgpNamespaceMacro;

impl Pipeline for CgpNamespaceMacro {
    const NAME: &'static str = "cgp_namespace";
    type Ir = Namespace;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(|table| Namespace::Table(Box::new(table)))
    }
}

/// `cgp_namespace!` entry point.
pub fn cgp_namespace(body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpNamespaceMacro>(TokenStream::new(), body)
}
