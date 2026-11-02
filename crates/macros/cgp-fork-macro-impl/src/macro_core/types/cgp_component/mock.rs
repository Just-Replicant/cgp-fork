//! `#[cgp_component(Name, mock)]` generates `MockName` and `NameMock`.

use proc_macro2::TokenStream;
use quote::quote;
use syn::ext::IdentExt;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{
    Error, FnArg, GenericParam, Ident, Pat, ReturnType, TraitItem, TraitItemFn, Type, WhereClause,
};

use crate::ir::Diagnostics;
use crate::macro_core::exports::{HasField, IsProviderFor};
use crate::macro_core::visitors::ReplaceSelfTypeVisitor;
use crate::macro_core::functions::to_snake_case_str;
use crate::macro_core::types::cgp_component::ItemCgpComponent;

/// Records every reason `mock` cannot be generated for this component.
pub fn validate_mock(component: &ItemCgpComponent, diagnostics: &mut Diagnostics) {
    if component.args.mock
        && let Err(error) = mock_items(component)
    {
        diagnostics.push(error);
    }
}

/// The mock provider, its state struct, and the `IsProviderFor` impl.
///
/// Empty when the attribute did not ask for a mock. The tokens are wrapped in
/// `__cgp_mock!`, which compiles them only when the facade's `mock` feature is on.
pub fn mock_items(component: &ItemCgpComponent) -> syn::Result<TokenStream> {
    if !component.args.mock {
        return Ok(TokenStream::new());
    }

    let mut diagnostics = Diagnostics::new();

    for param in &component.item_trait.generics.params {
        let message = match param {
            GenericParam::Type(_) | GenericParam::Const(_) => {
                "`mock` does not support type or const generics on the component"
            }
            GenericParam::Lifetime(_) => {
                "`mock` does not support lifetime parameters on the component"
            }
        };
        diagnostics.push(Error::new(param.span(), message));
    }

    let mut methods = Vec::new();

    for item in &component.item_trait.items {
        match item {
            TraitItem::Fn(method) => match method_signature(method) {
                Ok(method) => methods.push(method),
                Err(error) => diagnostics.push(error),
            },
            TraitItem::Type(assoc) => diagnostics.push(Error::new(
                assoc.span(),
                "`mock` does not support associated types",
            )),
            TraitItem::Const(assoc) => diagnostics.push(Error::new(
                assoc.span(),
                "`mock` does not support associated constants",
            )),
            other => diagnostics.push(Error::new(
                other.span(),
                "`mock` supports traits made of methods",
            )),
        }
    }

    if methods.is_empty() && diagnostics.is_empty() {
        diagnostics.push(Error::new(
            component.item_trait.span(),
            "`mock` requires at least one method",
        ));
    }

    if let Some(clause) = &component.item_trait.generics.where_clause {
        diagnostics.push(Error::new(
            clause.span(),
            "`mock` does not support a where clause on the component trait",
        ));
    }

    diagnostics.finish()?;

    let provider = &component.args.provider_ident;
    let plain = provider.unraw().to_string();
    let mock_provider = Ident::new(&format!("Mock{plain}"), provider.span());
    let state = Ident::new(&format!("{plain}Mock"), provider.span());
    let field = format!("{}_mock", to_snake_case_str(&plain));
    let context = &component.args.context_ident;
    let context_ty: Type = syn::parse_quote!(#context);
    for method in &mut methods {
        if let Some(clause) = &mut method.where_clause {
            ReplaceSelfTypeVisitor {
                replaced_type: &context_ty,
                skip_assoc_types: &Vec::new(),
            }
            .visit_where_clause_mut(clause);
        }
    }
    let component_name = &component.args.component_name;
    let root = crate::crate_path::crate_root();
    let has_field = HasField;
    let is_provider_for = IsProviderFor;
    let supertraits = &component.item_trait.supertraits;

    let fields = methods.iter().map(|method| {
        let queue = method.queue_field();
        let handler = method.handler_field();
        let ret = &method.output;
        let args = method.arg_types();
        quote! {
            #queue: #root::mock::MockReturns<#ret>,
            #handler: ::core::cell::RefCell<
                ::core::option::Option<#root::mock::Box<dyn ::core::ops::FnMut(#(#args),*) -> #ret>>,
            >,
        }
    });

    let inits = methods.iter().map(|method| {
        let queue = method.queue_field();
        let handler = method.handler_field();
        quote! {
            #queue: #root::mock::MockReturns::new(),
            #handler: ::core::cell::RefCell::new(::core::option::Option::None),
        }
    });

    let api = methods.iter().map(|method| {
        let name = &method.name;
        let returns = Ident::new(&format!("{name}_returns"), name.span());
        let with = Ident::new(&format!("{name}_with"), name.span());
        let count = Ident::new(&format!("{name}_call_count"), name.span());
        let queue = method.queue_field();
        let handler = method.handler_field();
        let ret = &method.output;
        let args = method.arg_types();
        let arg_names = method.arg_names();
        let call = quote! {
            pub fn #name(&self, #(#arg_names: #args),*) -> #ret {
                if self.#handler.borrow().is_some() {
                    self.#queue.record();
                    return #root::mock::invoke_handler(
                        &self.#handler,
                        |handler| handler(#(#arg_names),*),
                    ).expect(::core::concat!(stringify!(#mock_provider), "::", stringify!(#name)));
                }
                self.#queue.next(::core::concat!(stringify!(#mock_provider), "::", stringify!(#name)))
            }
        };
        quote! {
            /// Queue one return value. The next call that has no closure takes it.
            pub fn #returns(&self, value: #ret) -> &Self {
                self.#queue.returning(value);
                self
            }

            /// Handle every later call with `f`. A closure wins over queued values.
            pub fn #with(&self, handler: impl ::core::ops::FnMut(#(#args),*) -> #ret + 'static) -> &Self {
                *self.#handler.borrow_mut() = ::core::option::Option::Some(#root::mock::Box::new(handler));
                self
            }

            /// How many times this method has been called.
            pub fn #count(&self) -> usize {
                self.#queue.call_count()
            }

            #call
        }
    });

    let provider_methods = methods.iter().map(|method| {
        let name = &method.name;
        let ret = &method.output;
        let args = method.arg_types();
        let arg_names = method.arg_names();
        let where_clause = match &method.where_clause {
            Some(clause) => quote!(#clause),
            None => TokenStream::new(),
        };
        let reference = if method.receiver_mut {
            quote!(&mut)
        } else {
            quote!(&)
        };
        let ctx_name = method_context_name(method);
        let ctx_ref = if method.receiver_mut {
            quote!(&*#ctx_name)
        } else {
            quote!(#ctx_name)
        };
        quote! {
            fn #name(#ctx_name: #reference #context, #(#arg_names: #args),*) -> #ret
            #where_clause
            {
                let __cgp_mock_tag: ::core::marker::PhantomData<#root::prelude::Symbol!(#field)> =
                    ::core::marker::PhantomData;
                #has_field::get_field(#ctx_ref, __cgp_mock_tag).#name(#(#arg_names),*)
            }
        }
    });

    let context_bounds = if supertraits.is_empty() {
        quote! { #context: #has_field<#root::prelude::Symbol!(#field), Value = #state> }
    } else {
        quote! {
            #context: #has_field<#root::prelude::Symbol!(#field), Value = #state> + #supertraits
        }
    };

    Ok(quote! {
        #root::__cgp_mock! {
            /// Return values and call counts for [`#mock_provider`].
            ///
            /// Store it on the test context in a field named `#field` and derive `HasField`.
            pub struct #state {
                #(#fields)*
            }

            impl #state {
                /// An empty mock: each method panics until a value or closure is set.
                pub fn new() -> Self {
                    Self { #(#inits)* }
                }

                #(#api)*
            }

            impl ::core::default::Default for #state {
                fn default() -> Self {
                    Self::new()
                }
            }

            /// Provider that reads [`#state`] from the context field `#field`.
            pub struct #mock_provider;

            impl<#context> #provider<#context> for #mock_provider
            where
                #context_bounds
            {
                #(#provider_methods)*
            }

            impl<#context> #is_provider_for<#component_name, #context, ()> for #mock_provider
            where
                #context_bounds
            {}
        }
    })
}

struct MockMethod {
    name: Ident,
    receiver_mut: bool,
    args: Vec<(Ident, Type)>,
    output: Type,
    where_clause: Option<WhereClause>,
}

impl MockMethod {
    fn queue_field(&self) -> Ident {
        Ident::new(&format!("{}_queue", self.name), self.name.span())
    }

    fn handler_field(&self) -> Ident {
        Ident::new(&format!("{}_handler", self.name), self.name.span())
    }

    fn arg_types(&self) -> Vec<&Type> {
        self.args.iter().map(|(_, ty)| ty).collect()
    }

    fn arg_names(&self) -> Vec<&Ident> {
        self.args.iter().map(|(name, _)| name).collect()
    }
}

fn method_signature(method: &TraitItemFn) -> syn::Result<MockMethod> {
    if method.sig.asyncness.is_some() {
        return Err(Error::new(
            method.sig.span(),
            "`mock` does not support async methods",
        ));
    }

    if matches!(method.sig.safety, syn::Safety::Unsafe(_)) {
        return Err(Error::new(
            method.sig.span(),
            "`mock` does not support unsafe methods",
        ));
    }

    if !method.sig.generics.params.is_empty() {
        return Err(Error::new(
            method.sig.generics.span(),
            "`mock` does not support generic methods",
        ));
    }

    let mut receiver_mut = false;
    let mut saw_receiver = false;
    let mut args = Vec::new();

    for (index, input) in method.sig.inputs.iter().enumerate() {
        match input {
            FnArg::Receiver(receiver) => match &receiver.kind {
                syn::ReceiverKind::Reference(_, _, mutability) => {
                    receiver_mut = mutability.is_some();
                    saw_receiver = true;
                }
                _ => {
                    return Err(Error::new(
                        receiver.span(),
                        "`mock` methods take `&self` or `&mut self`",
                    ));
                }
            },
            FnArg::Typed(typed) => {
                if mentions_self(&typed.ty) {
                    return Err(Error::new(
                        typed.ty.span(),
                        "`mock` does not support `Self` in a method argument",
                    ));
                }
                let name = match &*typed.pat {
                    Pat::Ident(pat) => pat.ident.clone(),
                    _ => Ident::new(&format!("arg{index}"), typed.pat.span()),
                };
                args.push((name, (*typed.ty).clone()));
            }
        }
    }

    if !saw_receiver {
        return Err(Error::new(
            method.sig.span(),
            "`mock` methods take `&self` or `&mut self`",
        ));
    }

    let output = match &method.sig.output {
        ReturnType::Default => syn::parse_quote!(()),
        ReturnType::Type(_, ty) => {
            if matches!(ty.as_ref(), Type::Reference(_) | Type::ImplTrait(_)) {
                return Err(Error::new(
                    ty.span(),
                    "`mock` returns an owned value; a reference or `impl Trait` cannot be queued",
                ));
            }
            if mentions_self(ty) {
                return Err(Error::new(
                    ty.span(),
                    "`mock` does not support `Self` in a return type",
                ));
            }
            (**ty).clone()
        }
    };

    Ok(MockMethod {
        name: method.sig.ident.clone(),
        receiver_mut,
        args,
        output,
        where_clause: method.sig.generics.where_clause.clone(),
    })
}

fn mentions_self(ty: &Type) -> bool {
    struct Finder(bool);

    impl<'ast> Visit<'ast> for Finder {
        fn visit_ident(&mut self, ident: &'ast Ident) {
            if ident == "Self" {
                self.0 = true;
            }
        }
    }

    let mut finder = Finder(false);
    finder.visit_type(ty);
    finder.0
}

fn method_context_name(method: &MockMethod) -> Ident {
    if method.args.iter().any(|(name, _)| name == "context") {
        Ident::new("cgp_context", method.name.span())
    } else {
        Ident::new("context", method.name.span())
    }
}
