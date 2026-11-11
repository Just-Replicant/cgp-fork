use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Error, FnArg, Ident, ImplItemFn, Pat, Signature, Type, Visibility};

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

    // A `mut` on a parameter or a by-value receiver belongs to the default body that mutates it.
    // The forwarding body only passes the argument on, where the `mut` would be unused. A
    // `&mut self` receiver keeps its mutability, because that is part of the type.
    strip_forwarded_mut(&mut signature);

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

/// The name an argument is forwarded by, a `mut value` parameter's included. A parameter without a
/// name of its own (`_`, `ref value`, a destructuring pattern) has none, so it is rejected with a
/// message saying what to write.
fn arg_to_ident(arg: &FnArg) -> syn::Result<Ident> {
    match arg {
        FnArg::Receiver(_) => Ok(Ident::new("self", Span::call_site())),
        FnArg::Typed(arg) => match arg.pat.as_ref() {
            Pat::Ident(pat) if pat.by_ref.is_none() && pat.subpat.is_none() => {
                Ok(pat.ident.clone())
            }
            pat => Err(Error::new_spanned(
                pat,
                "a component method parameter needs a plain name, since the generated impls forward each argument by name; write a name such as `_value`, and destructure inside a default body",
            )),
        },
    }
}

/// Drop `mut` bindings the forwarding impl does not use. `mut self` stores its `mut` on the
/// receiver; `&mut self` stores it inside the reference kind, which stays.
fn strip_forwarded_mut(signature: &mut Signature) {
    for input in &mut signature.inputs {
        match input {
            FnArg::Typed(arg) => {
                if let Pat::Ident(pat) = arg.pat.as_mut() {
                    pat.mutability = None;
                }
            }
            FnArg::Receiver(receiver) => {
                if matches!(receiver.kind, syn::ReceiverKind::Value) {
                    receiver.mutability = None;
                }
            }
        }
    }
}
