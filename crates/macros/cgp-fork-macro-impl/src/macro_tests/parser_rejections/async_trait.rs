//! `#[async_trait]` takes no arguments. Before 0.10 it silently ignored them, so
//! `#[async_trait(?Send)]` compiled and looked like it relaxed a `Send` bound that
//! the desugaring never adds in the first place.

use quote::quote;

use super::assert_macro_rejects_with;

const MESSAGE: &str = "`#[async_trait]` takes no arguments; the desugared future is not `Send`. \
                       Use `CanSendRun` or a `Send` bound at the call site when a `Send` future is needed";

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
