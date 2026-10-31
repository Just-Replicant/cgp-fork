use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::ir::{Diagnostics, IrFamily, Pipeline, run};
use crate::macro_core::types::delegate_component::ValidateAttributes;
use crate::macro_core::types::namespace::NamespaceTable;

/// `cgp_namespace!`: a namespace and the components it redirects.
pub struct CgpNamespaceMacro;

impl Pipeline for CgpNamespaceMacro {
    const NAME: &'static str = "cgp_namespace";
    const FAMILY: IrFamily = IrFamily::Namespace;
    type Ir = NamespaceTable;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body)
    }

    /// A namespace body reuses the `delegate_components!` entry grammar, which
    /// supports no attributes, so reject any on its entries instead of silently
    /// parsing and discarding them — matching `delegate_components!`.
    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        diagnostics.check(ir.entries.validate_attributes());
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        Ok(ir.eval()?.to_token_stream())
    }
}

/// `cgp_namespace!` entry point.
pub fn cgp_namespace(body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpNamespaceMacro>(TokenStream::new(), body)
}
