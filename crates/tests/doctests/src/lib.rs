//! Doctests for documentation that is not part of a published package.
//!
//! The repository `README.md` sits outside `crates/main/cgp-fork`, so the
//! published `cgp-fork` cannot `include_str!` it. This crate is a workspace
//! member with `publish = false` whose only job is to compile and run those
//! examples under `cargo test --workspace --doc`. Nothing gates them on a cfg
//! flag, so `RUSTDOCFLAGS` or a missing `.cargo/config.toml` cannot drop them.
#![no_std]

/// The repository README's examples.
#[cfg(doctest)]
#[doc = include_str!("../../../../README.md")]
pub struct RepositoryReadme;
