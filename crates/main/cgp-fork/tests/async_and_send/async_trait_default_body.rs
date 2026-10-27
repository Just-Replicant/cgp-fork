//! Regression test: `#[async_trait]` on a trait whose `async fn` has a default
//! (provided) body. Before 0.10 the body was left as-is while the signature became
//! `-> impl Future`, so the trait failed with "`{integer}` is not a future".
//!
//! The body is now wrapped in `async move { … }`. This covers a plain trait, a
//! default body that awaits another method, one that borrows its argument, and
//! an override of the default.

use cgp_fork::prelude::*;
use futures::executor::block_on;

#[async_trait]
pub trait HasDefaultValue {
    async fn base(&self) -> u32;

    async fn value(&self) -> u32 {
        self.base().await + 1
    }

    async fn unit(&self) {}

    async fn len_of(&self, input: &str) -> usize {
        input.len()
    }
}

pub struct UsesDefault;

impl HasDefaultValue for UsesDefault {
    async fn base(&self) -> u32 {
        6
    }
}

pub struct Overrides;

impl HasDefaultValue for Overrides {
    async fn base(&self) -> u32 {
        0
    }

    async fn value(&self) -> u32 {
        42
    }
}

#[test]
fn test_default_async_body_runs() {
    assert_eq!(block_on(UsesDefault.value()), 7);
    block_on(UsesDefault.unit());
    assert_eq!(block_on(UsesDefault.len_of("hello")), 5);
}

#[test]
fn test_default_async_body_can_be_overridden() {
    assert_eq!(block_on(Overrides.value()), 42);
}
