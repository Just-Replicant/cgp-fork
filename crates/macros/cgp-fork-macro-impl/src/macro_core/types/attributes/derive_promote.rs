use quote::quote_spanned;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    Error, Expr, FnArg, Generics, Ident, ImplItem, Item, ItemImpl, ItemTrait, Pat, Signature,
    TraitItem, Type,
};

use crate::extra_macro_core::exports::UnifiedHandler;
use crate::macro_core::exports::Computer;
use crate::macro_core::parse_internal;
use crate::macro_core::types::empty_struct::EmptyStruct;

/// `#[derive_handler(PromoteAreaCalculator)]` (and the deprecated
/// `#[derive_promote(…)]`) on `#[cgp_component]`.
///
/// Generates a provider struct that implements the component by calling
/// `Handler::handle` (with `#[derive_promote]`, the 0.10 `Computer::compute`) on an
/// inner provider. Wire it as `PromoteAreaCalculator<SomeHandler>`. The same shape as
/// the `Promote…` adapters in `cgp_fork::handler`, aimed at a component instead of
/// another handler.
#[derive(Clone)]
pub struct DerivePromoteAttribute {
    pub provider: Ident,
    /// `#[derive_handler]`: call the unified `Handler`. `false` is `#[derive_promote]`.
    pub unified: bool,
    /// The attribute's name, for the deprecation warning on `#[derive_promote]`.
    pub attr_ident: Ident,
}

impl DerivePromoteAttribute {
    pub fn to_struct(&self) -> EmptyStruct {
        // `<__Provider__>` is a fixed token sequence.
        let generics: Generics = syn::parse_quote!(<__Provider__>);

        let doc = format!(" Provider `{}`.", self.provider);
        EmptyStruct::new(self.provider.clone(), generics).with_doc(doc)
    }

    /// The provider impl, plus the `IsProviderFor` impl the caller derives from it.
    pub fn to_provider_impl(&self, provider_trait: &ItemTrait) -> syn::Result<ItemImpl> {
        // With `#[derive_promote]` the impl names the deprecated `Computer`; the caller
        // allows that on the generated impls, and the user is warned once, at the
        // attribute, by `deprecation_marker`.
        let item_impl = self.to_provider_impl_inner(provider_trait);
        if self.unified {
            item_impl.map_err(rename_errors_for_derive_handler)
        } else {
            item_impl
        }
    }

    /// For `#[derive_promote]`, an item that names a deprecated marker at the attribute,
    /// so the user is warned there with the replacement. Nothing for `#[derive_handler]`.
    pub fn deprecation_marker(&self) -> Option<Item> {
        if self.unified {
            return None;
        }
        let root = crate::crate_path::crate_root();
        let span = self.attr_ident.span();
        let marker = quote_spanned! { span => #root::macro_prelude::DERIVE_PROMOTE };
        Some(syn::parse_quote! {
            const _: () = #marker;
        })
    }

    fn to_provider_impl_inner(&self, provider_trait: &ItemTrait) -> syn::Result<ItemImpl> {
        let method = single_method(provider_trait)?;
        let inputs = method_inputs(&method.sig)?;
        let output = output_type(&method.sig)?;
        let context_ident = &inputs.context_ident;
        let context_type = &inputs.context_type;

        let input_types: Vec<Type> = inputs.args.iter().map(|arg| arg.ty.clone()).collect();
        let mut input_exprs = Vec::new();
        for arg in &inputs.args {
            let ident = &arg.ident;
            input_exprs.push(parse_internal!(#ident));
        }

        let input_type = tuple_type(&input_types)?;
        let input_expr = tuple_expr(&input_exprs)?;

        let call: Expr = if self.unified {
            parse_internal! {
                < __Provider__ as #UnifiedHandler < #context_type, (), #input_type > >::handle(
                    #context_ident,
                    ::core::marker::PhantomData::< () >,
                    #input_expr,
                )
            }
        } else {
            parse_internal! {
                < __Provider__ as #Computer < #context_type, (), #input_type > >::compute(
                    #context_ident,
                    ::core::marker::PhantomData::< () >,
                    #input_expr,
                )
            }
        };

        let block: syn::Block = parse_internal! {
            { #call }
        };

        let sig = &method.sig;
        let impl_fn: ImplItem = parse_internal! {
            #sig #block
        };

        let provider = &self.provider;
        let trait_ident = &provider_trait.ident;
        let type_generics = provider_trait.generics.split_for_impl().1;

        let mut item_impl: ItemImpl = parse_internal! {
            impl #trait_ident #type_generics for #provider < __Provider__ > {
                #impl_fn
            }
        };

        let mut generics = provider_trait.generics.clone();
        generics.params.push(parse_internal!(__Provider__));
        generics
            .make_where_clause()
            .predicates
            .push(if self.unified {
                parse_internal! {
                    __Provider__ : #UnifiedHandler < #context_type, (), #input_type, Output = #output >
                }
            } else {
                parse_internal! {
                    __Provider__ : #Computer < #context_type, (), #input_type, Output = #output >
                }
            });

        item_impl.generics = generics;
        item_impl.attrs = provider_trait.attrs.clone();
        item_impl.unsafety = provider_trait.unsafety;

        Ok(item_impl)
    }
}

struct ForwardedArg {
    ident: Ident,
    ty: Type,
}

struct MethodInputs {
    context_ident: Ident,
    context_type: Type,
    args: Vec<ForwardedArg>,
}

impl DerivePromoteAttribute {
    /// Parses the arguments of `#[derive_handler(…)]` (`unified`) or `#[derive_promote(…)]`.
    pub fn parse_args(
        attr_ident: &Ident,
        unified: bool,
    ) -> impl Fn(ParseStream) -> syn::Result<Self> {
        let attr_ident = attr_ident.clone();
        move |input: ParseStream| {
            let provider: Ident = input.parse()?;

            if !input.is_empty() {
                let name = attr_ident.to_string();
                return Err(Error::new(
                    input.span(),
                    format!(
                        "`#[{name}]` takes only the provider name, as in `#[{name}(PromoteAreaCalculator)]`"
                    ),
                ));
            }

            Ok(Self {
                provider,
                unified,
                attr_ident: attr_ident.clone(),
            })
        }
    }
}

/// The shared checks are worded for `#[derive_promote]` and the 0.10 `Computer`; reword
/// them for `#[derive_handler]`.
fn rename_errors_for_derive_handler(error: Error) -> Error {
    let mut renamed: Option<Error> = None;
    for error in error {
        let message = error
            .to_string()
            .replace("`#[derive_promote]`", "`#[derive_handler]`")
            .replace("`Computer::compute`", "`Handler::handle`")
            .replace("a Computer has one `compute`", "a Handler has one `handle`")
            .replace("a Computer exposes", "a Handler exposes");
        let error = Error::new(error.span(), message);
        match &mut renamed {
            Some(renamed) => renamed.combine(error),
            None => renamed = Some(error),
        }
    }
    renamed.expect("a syn::Error holds at least one message")
}

fn single_method(provider_trait: &ItemTrait) -> syn::Result<&syn::TraitItemFn> {
    let mut method = None;

    for item in &provider_trait.items {
        match item {
            TraitItem::Fn(func) => {
                if method.is_some() {
                    return Err(Error::new(
                        func.span(),
                        "`#[derive_promote]` requires a component with exactly one method, because a Computer has one `compute`",
                    ));
                }
                method = Some(func);
            }
            TraitItem::Type(assoc) => {
                return Err(Error::new(
                    assoc.span(),
                    "`#[derive_promote]` cannot promote a component that declares an associated type; a Computer exposes its result as `Output`",
                ));
            }
            TraitItem::Const(assoc) => {
                return Err(Error::new(
                    assoc.span(),
                    "`#[derive_promote]` cannot promote a component that declares an associated const",
                ));
            }
            TraitItem::Macro(mac) => {
                return Err(Error::new(
                    mac.span(),
                    "`#[derive_promote]` cannot promote a component that contains a macro item",
                ));
            }
            TraitItem::Verbatim(tokens) => {
                return Err(Error::new_spanned(
                    tokens,
                    "`#[derive_promote]` cannot promote this component item",
                ));
            }
            _ => {
                return Err(Error::new_spanned(
                    item,
                    "`#[derive_promote]` cannot promote this component item",
                ));
            }
        }
    }

    method.ok_or_else(|| {
        Error::new(
            provider_trait.ident.span(),
            "`#[derive_promote]` requires a component with exactly one method",
        )
    })
}

fn method_inputs(sig: &Signature) -> syn::Result<MethodInputs> {
    if sig.asyncness.is_some() {
        return Err(Error::new(
            sig.asyncness.span(),
            "`#[derive_promote]` wraps a synchronous `Computer::compute`, which cannot implement an async method",
        ));
    }

    if !sig.generics.params.is_empty() {
        return Err(Error::new(
            sig.generics.span(),
            "`#[derive_promote]` does not support method-level generic parameters",
        ));
    }

    let mut inputs = sig.inputs.iter();
    let Some(first) = inputs.next() else {
        return Err(Error::new(
            sig.span(),
            "`#[derive_promote]` requires the method to take `&self`",
        ));
    };

    let (context_ident, context_type) = context_arg(first)?;

    let mut args = Vec::new();
    for arg in inputs {
        let FnArg::Typed(pat) = arg else {
            return Err(Error::new_spanned(
                arg,
                "`#[derive_promote]` requires the method to take `&self` first",
            ));
        };
        let ident = match &*pat.pat {
            Pat::Ident(pat_ident) => pat_ident.ident.clone(),
            _ => {
                return Err(Error::new_spanned(
                    &pat.pat,
                    "`#[derive_promote]` forwards each argument by name to `Computer::compute`",
                ));
            }
        };
        args.push(ForwardedArg {
            ident,
            ty: pat.ty.as_ref().clone(),
        });
    }

    Ok(MethodInputs {
        context_ident,
        context_type,
        args,
    })
}

fn context_arg(arg: &FnArg) -> syn::Result<(Ident, Type)> {
    let FnArg::Typed(pat) = arg else {
        return Err(Error::new_spanned(
            arg,
            "`#[derive_promote]` requires the method to take `&self`",
        ));
    };

    let Pat::Ident(pat_ident) = &*pat.pat else {
        return Err(Error::new_spanned(
            &pat.pat,
            "`#[derive_promote]` requires the method to take `&self`",
        ));
    };

    let Type::Reference(reference) = pat.ty.as_ref() else {
        return Err(Error::new_spanned(
            pat.ty.as_ref(),
            "`#[derive_promote]` requires the method to take `&self`",
        ));
    };

    if reference.mutability.is_some() {
        return Err(Error::new_spanned(
            pat.ty.as_ref(),
            "`#[derive_promote]` wraps `Computer::compute`, which borrows the context immutably",
        ));
    }

    Ok((pat_ident.ident.clone(), reference.elem.as_ref().clone()))
}

fn output_type(sig: &Signature) -> syn::Result<Type> {
    match &sig.output {
        syn::ReturnType::Default => {
            let ty: Type = parse_internal!(());
            Ok(ty)
        }
        syn::ReturnType::Type(_, ty) => {
            if matches!(ty.as_ref(), Type::ImplTrait(_)) {
                return Err(Error::new_spanned(
                    ty.as_ref(),
                    "`#[derive_promote]` cannot use an `impl Trait` return as a Computer `Output`",
                ));
            }
            Ok(ty.as_ref().clone())
        }
    }
}

fn tuple_type(types: &[Type]) -> syn::Result<Type> {
    match types {
        [] => {
            let ty: Type = parse_internal!(());
            Ok(ty)
        }
        [ty] => Ok(ty.clone()),
        types => {
            let mut punct: Punctuated<Type, Comma> = Punctuated::new();
            for ty in types {
                punct.push(ty.clone());
            }
            let ty: Type = parse_internal!((#punct));
            Ok(ty)
        }
    }
}

fn tuple_expr(exprs: &[Expr]) -> syn::Result<Expr> {
    match exprs {
        [] => {
            let expr: Expr = parse_internal!(());
            Ok(expr)
        }
        [expr] => Ok(expr.clone()),
        exprs => {
            let mut punct: Punctuated<Expr, Comma> = Punctuated::new();
            for expr in exprs {
                punct.push(expr.clone());
            }
            let expr: Expr = parse_internal!((#punct));
            Ok(expr)
        }
    }
}
