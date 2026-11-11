//! Canonical expansion of an async `#[cgp_fn]`.
//!
//! `#[cgp_async]` combined with `#[cgp_fn]` turns an `async fn` into an async
//! blanket-impl trait: the generated trait method is `async fn`, both the trait
//! and its blanket impl carry `#[cgp_async]`, and `#[implicit]` arguments are
//! still pulled from the context's fields. A bare `#[async_trait]` on the
//! function is rewritten to `#[cgp_async]` and warns once. This is the async
//! variant of the `#[cgp_fn]` expansion, which the `async_and_send` concept owns.
//!
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/async_trait.md.

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    #[cgp_async]
    pub async fn greet(&self, #[implicit] name: &str) -> String {
        format!("Hello, {name}!")
    }

    expand_greet(output) {
        insta::assert_snapshot!(output, @r#"
        #[cgp_async]
        /// The trait for the `greet` operation.
        pub trait Greet {
            /// The `greet` operation.
            async fn greet(&self) -> String;
        }
        #[cgp_async]
        impl<__Context__> Greet for __Context__
        where
            Self: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value: ::core::convert::AsRef<str> + 'static,
            >,
        {
            async fn greet(&self) -> String {
                let name: &str = ::core::convert::AsRef::as_ref(
                    HasField::get_field(
                        self,
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        >,
                    ),
                );
                format!("Hello, {name}!")
            }
        }
        "#)
    }
}
