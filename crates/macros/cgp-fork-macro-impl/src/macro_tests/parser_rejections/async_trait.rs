//! `#[async_trait]` takes no arguments. Before 0.10 it silently ignored them, so
//! `#[async_trait(?Send)]` compiled and looked like it relaxed a `Send` bound that
//! the desugaring never adds in the first place.

use quote::quote;

use super::assert_macro_rejects_with;

const MESSAGE: &str = "`#[async_trait]` takes no arguments; the desugared future is not `Send`. \
                       Use `#[cgp_async]` instead, or `#[cgp_async(Send)]` when the future must be `Send`";

const CGP_ASYNC_MESSAGE: &str = "`#[cgp_async]` accepts no arguments, or `Send` \
     (`#[cgp_async(Send)]` adds `+ Send` to every desugared future)";

#[test]
fn rejects_question_send_argument() {
    assert_macro_rejects_with("async_trait(?Send)", MESSAGE, || {
        crate::async_impl::impl_async(
            quote!(?Send),
            quote!(
                pub trait CanFetch {
                    async fn fetch(&self) -> u32;
                }
            ),
        )
    });
}

#[test]
fn rejects_arbitrary_arguments() {
    assert_macro_rejects_with("async_trait(whatever = 1)", MESSAGE, || {
        crate::async_impl::impl_async(
            quote!(whatever = 1),
            quote!(
                pub trait CanFetch {
                    async fn fetch(&self) -> u32;
                }
            ),
        )
    });
}

#[test]
fn wraps_default_body_in_async_block() {
    let output = crate::async_impl::impl_async(
        quote!(),
        quote!(
            pub trait HasDefault {
                async fn value(&self) -> u32 {
                    7
                }
            }
        ),
    )
    .expect("a trait with a provided async method expands");

    let expected = quote!(
        pub trait HasDefault {
            fn value(&self) -> impl ::core::future::Future<Output = u32> {
                async move { 7 }
            }
        }
    );
    assert_eq!(output.to_string(), expected.to_string());
}

#[test]
fn cgp_async_without_arguments_matches_async_trait() {
    let item = quote!(
        pub trait CanFetch {
            async fn fetch(&self) -> u32;
        }
    );
    let legacy = crate::async_impl::impl_async(quote!(), item.clone()).unwrap();
    let current = crate::async_impl::impl_cgp_async(quote!(), item).unwrap();
    assert_eq!(legacy.to_string(), current.to_string());
}

#[test]
fn cgp_async_send_adds_the_send_bound() {
    let output = crate::async_impl::impl_cgp_async(
        quote!(Send),
        quote!(
            pub trait CanFetch {
                async fn fetch(&self) -> u32 {
                    1
                }
            }
        ),
    )
    .expect("#[cgp_async(Send)] expands");

    let expected = quote!(
        pub trait CanFetch {
            fn fetch(&self) -> impl ::core::future::Future<Output = u32> + ::core::marker::Send {
                async move { 1 }
            }
        }
    );
    assert_eq!(output.to_string(), expected.to_string());
}

#[test]
fn cgp_async_rejects_question_send() {
    assert_macro_rejects_with("cgp_async(?Send)", CGP_ASYNC_MESSAGE, || {
        crate::async_impl::impl_cgp_async(
            quote!(?Send),
            quote!(
                pub trait CanFetch {
                    async fn fetch(&self) -> u32;
                }
            ),
        )
    });
}

#[test]
fn cgp_async_send_on_an_impl_is_left_unchanged() {
    let item = quote!(
        impl CanFetch for App {
            async fn fetch(&self) -> u32 {
                1
            }
        }
    );
    let output = crate::async_impl::impl_cgp_async(quote!(Send), item.clone()).unwrap();
    assert_eq!(output.to_string(), item.to_string());
}

#[test]
fn leaves_non_trait_items_unchanged() {
    let item = quote!(
        impl CanFetch for App {
            async fn fetch(&self) -> u32 {
                1
            }
        }
    );
    let output = crate::async_impl::impl_async(quote!(), item.clone()).unwrap();
    assert_eq!(output.to_string(), item.to_string());
}

#[test]
fn component_rewrites_bare_async_trait_once() {
    let output = crate::macro_lib::cgp_component(
        quote!(Fetcher),
        quote!(
            #[async_trait]
            pub trait CanFetch {
                async fn fetch(&self) -> u32;
            }
        ),
    )
    .expect("#[cgp_component] forwards #[async_trait]");

    let rendered = output.to_string();
    assert!(
        !rendered.contains("async_trait"),
        "the old attribute must be rewritten, got {rendered}"
    );
    assert!(rendered.contains("cgp_async"));
    assert!(rendered.contains("ASYNC_TRAIT"));
}

#[test]
fn component_send_impl_returns_the_delegate_future() {
    let output = crate::macro_lib::cgp_component(
        quote!(Runner),
        quote!(
            #[cgp_async(Send)]
            pub trait CanRun {
                async fn run(&self) -> u32;
            }
        ),
    )
    .expect("#[cgp_async(Send)] on a component expands");

    let rendered = output.to_string();
    assert!(
        rendered.contains("+ :: core :: marker :: Send"),
        "forwarding impls must return a Send future, got {rendered}"
    );
    assert!(
        !rendered.contains(".await") && !rendered.contains(". await"),
        "a Send forwarder must not await, or the generic context is captured, got {rendered}"
    );
}
