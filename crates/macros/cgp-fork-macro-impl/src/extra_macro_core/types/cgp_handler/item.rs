use proc_macro2::Span;
use syn::{Ident, ItemFn, ItemImpl};

use crate::extra_macro_core::exports::{
    AsyncComputerComponent, AsyncComputerRefComponent, Compat, ComputerComponent,
    ComputerRefComponent, HandlerComponent, HandlerRefComponent, ProducerComponent,
    TryComputerComponent, TryComputerRefComponent, UnifiedAsyncHandler,
    UnifiedAsyncHandlerComponent, UnifiedHandler, UnifiedHandlerComponent, UnifiedPromote,
};
use crate::extra_macro_core::types::cgp_computer::{ItemCgpComputer, PreprocessedCgpComputer};
use crate::macro_core::functions::override_item_span;
use crate::macro_core::parse_internal;
use crate::macro_core::traits::ToTypeParamBounds;
use crate::macro_core::types::cgp_provider::{ItemCgpProvider, ProviderArgs};
use crate::macro_core::types::delegate_component::DelegateTable;
use crate::macro_core::types::keyword::Keyword;

/// Raw input stage of `#[cgp_handler]`: the optional provider name and the function.
pub struct ItemCgpHandler {
    pub ident: Option<Ident>,
    pub item_fn: ItemFn,
    /// Also wire the provider to the nine 0.10 handler components through `Compat`, when
    /// the facade has `compat-0.10`. Set by `#[cgp_auto_dispatch]`, whose per-method
    /// providers were 0.10 `Computer`s; `#[cgp_handler]` itself emits the unified family
    /// only.
    pub legacy_wiring: bool,
}

/// The evaluated `#[cgp_handler]`: the function, the base provider, and its wiring.
pub struct EvaluatedCgpHandler {
    /// The annotated function, emitted unchanged.
    pub item_fn: ItemFn,
    /// `Handler` for a synchronous function, `AsyncHandler` for an `async` one.
    pub provider: ItemCgpProvider,
    /// For a synchronous function, `AsyncHandlerComponent: Promote<Self>`, so the same
    /// provider also answers the async component.
    pub delegate_table: Option<DelegateTable>,
    /// The 0.10 components routed to `Compat<Self>` (see [`ItemCgpHandler::legacy_wiring`]).
    pub legacy_table: Option<DelegateTable>,
}

impl ItemCgpHandler {
    /// Read the function as `#[cgp_computer]` does: explicit parameters form the input
    /// tuple, `#[implicit]`/`#[field]` parameters are read from the context.
    pub fn preprocess(&self) -> syn::Result<PreprocessedCgpComputer> {
        ItemCgpComputer {
            ident: self.ident.clone(),
            item_fn: self.item_fn.clone(),
        }
        .preprocess_for("Handler", false)
    }

    pub fn eval(&self) -> syn::Result<EvaluatedCgpHandler> {
        let preprocessed = self.preprocess()?;
        eval_handler(&preprocessed, self.legacy_wiring)
    }
}

/// The unified expansion of a preprocessed handler function.
pub fn eval_handler(
    preprocessed: &PreprocessedCgpComputer,
    legacy_wiring: bool,
) -> syn::Result<EvaluatedCgpHandler> {
    let provider_ident = &preprocessed.provider_ident;
    let fn_ident = &preprocessed.item_fn.sig.ident;
    let input_idents = &preprocessed.input_idents;
    let input_types = &preprocessed.input_types;
    let call_args = &preprocessed.call_args;
    let output = &preprocessed.output;

    // A context-field read names the receiver `context`; otherwise it stays unused.
    let context_ident = if preprocessed.context_fields.fields.is_empty() {
        Ident::new("_context", Span::call_site())
    } else {
        Ident::new("context", fn_ident.span())
    };

    let mut generics = preprocessed.item_fn.sig.generics.clone();
    generics.params.push(parse_internal!(__Context__));
    generics.params.push(parse_internal!(__Code__));

    if !preprocessed.context_fields.fields.is_empty() {
        let bounds = preprocessed.context_fields.to_type_param_bounds()?;
        generics
            .make_where_clause()
            .predicates
            .push(parse_internal!(__Context__: #bounds));
    }

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let (item_impl, component_type, delegate_table): (ItemImpl, _, Option<DelegateTable>) =
        if preprocessed.is_async {
            let item_impl = parse_internal! {
                impl #impl_generics
                    #UnifiedAsyncHandler<__Context__, __Code__, ( #input_types )>
                    for #provider_ident
                #where_clause
                {
                    type Output = #output;

                    async fn handle_async(
                        #context_ident: &__Context__,
                        _code: ::core::marker::PhantomData<__Code__>,
                        ( #input_idents ): ( #input_types ),
                    ) -> Self::Output {
                        #fn_ident( #call_args ).await
                    }
                }
            };

            (
                item_impl,
                parse_internal!(#UnifiedAsyncHandlerComponent),
                None,
            )
        } else {
            let item_impl = parse_internal! {
                impl #impl_generics
                    #UnifiedHandler<__Context__, __Code__, ( #input_types )>
                    for #provider_ident
                #where_clause
                {
                    type Output = #output;

                    fn handle(
                        #context_ident: &__Context__,
                        _code: ::core::marker::PhantomData<__Code__>,
                        ( #input_idents ): ( #input_types ),
                    ) -> Self::Output {
                        #fn_ident( #call_args )
                    }
                }
            };

            // A synchronous handler is also an asynchronous one whose future is ready.
            let delegate_table = parse_internal! {
                #provider_ident {
                    #UnifiedAsyncHandlerComponent: #UnifiedPromote<Self>,
                }
            };

            (
                item_impl,
                parse_internal!(#UnifiedHandlerComponent),
                Some(delegate_table),
            )
        };

    // Errors on the generated impl point at the function name; see `#[cgp_computer]`.
    let item_impl = override_item_span(fn_ident.span(), &item_impl)?;

    let legacy_table = if legacy_wiring {
        Some(parse_internal! {
            #provider_ident {
                [
                    #ComputerComponent,
                    #ComputerRefComponent,
                    #TryComputerComponent,
                    #TryComputerRefComponent,
                    #AsyncComputerComponent,
                    #AsyncComputerRefComponent,
                    #HandlerComponent,
                    #HandlerRefComponent,
                    #ProducerComponent,
                ]:
                    #Compat<Self>,
            }
        })
    } else {
        None
    };

    Ok(EvaluatedCgpHandler {
        item_fn: preprocessed.item_fn.clone(),
        provider: ItemCgpProvider {
            args: ProviderArgs {
                new: Some(Keyword::default()),
                component_type: Some(component_type),
            },
            item_impl,
        },
        delegate_table,
        legacy_table,
    })
}
