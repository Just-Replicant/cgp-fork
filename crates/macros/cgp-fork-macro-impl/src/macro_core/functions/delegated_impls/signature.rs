use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FnArg, Ident, ImplItemFn, Signature, Type, Visibility};

use crate::macro_core::functions::parse_internal;

/// Build an impl method that keeps `signature` and forwards its arguments to the
/// same method on `delegate_type`, adding `.await` when the signature is `async`.
///
/// `send` is `#[cgp_async(Send)]`: the impl method is the desugared
/// `fn -> impl Future + Send` and returns the delegate's future directly. An `async`
/// block would capture `&Self`, which is not `Send` for a generic context.
pub fn signature_to_delegated_impl_item_fn(
    signature: &Signature,
    delegate_type: &Type,
    send: bool,
) -> syn::Result<ImplItemFn> {
    let fn_name = &signature.ident;

    let args: Punctuated<_, Comma> = signature_to_idents(signature)?;

    let mut signature = signature.clone();
    let await_expr: TokenStream = if send && signature.asyncness.is_some() {
        let return_type = match &signature.output {
            syn::ReturnType::Default => quote!(()),
            syn::ReturnType::Type(_, return_type) => return_type.to_token_stream(),
        };
        signature.asyncness = None;
        signature.output = parse_internal!(
            -> impl ::core::future::Future<Output = #return_type> + ::core::marker::Send
        );
        TokenStream::new()
    } else if signature.asyncness.is_some() {
        quote!( .await )
    } else {
        TokenStream::new()
    };

    let body = parse_internal!({
        #delegate_type :: #fn_name (
            #args
        ) #await_expr
    });

    let item = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        modifiers: syn::FnModifiers::default(),
        sig: signature,
        block: body,
    };

    Ok(item)
}

fn signature_to_idents(sig: &Signature) -> syn::Result<Punctuated<Ident, Comma>> {
    sig.inputs.iter().map(arg_to_ident).collect()
}

fn arg_to_ident(arg: &FnArg) -> syn::Result<Ident> {
    let ident = match arg {
        FnArg::Receiver(_) => Ident::new("self", Span::call_site()),
        FnArg::Typed(pat) => parse_internal(pat.pat.to_token_stream())?,
    };

    Ok(ident)
}
