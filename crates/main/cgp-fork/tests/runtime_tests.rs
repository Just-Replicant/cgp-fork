//! Entrypoint for the `runtime` concept.
//!
//! `HasRuntimeType` names the runtime, `HasRuntime` borrows it, and
//! `HasRuntimeMut` borrows it mutably. Both getters read the same field when
//! wired with `UseField`. The `tokio` and `smol` features add `AsyncRuntime`
//! (spawn, sleep, timeout) for those runtimes.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_getter.md.
#![allow(dead_code)]

pub mod runtime;
