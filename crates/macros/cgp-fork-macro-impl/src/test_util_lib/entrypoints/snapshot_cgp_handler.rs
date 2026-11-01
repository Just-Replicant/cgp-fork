use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemFn, parse2};

use crate::test_util_lib::keywords::CgpHandler;
use crate::test_util_lib::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_handler(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpHandler, ItemFn> = parse2(body)?;

    let output = crate::extra_macro_lib::cgp_handler(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}
