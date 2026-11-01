//! Entrypoint for the `error_backends` concept.
//!
//! Covers the error backends behind the `anyhow`, `eyre`, and `std-error` features
//! (`cgp_fork::anyhow`, `cgp_fork::eyre`, `cgp_fork::std_error`): wiring each as a
//! context's abstract error type, raising standard and non-standard source errors
//! through their providers, wrapping details, and the output each produces. The
//! knowledge base documents the backends as a project of their own.
//!
//! See cgp-knowledge-base-fork/projects/error/README.md and
//! cgp-knowledge-base-fork/cgp/concepts/modular-error-handling.md.
#![cfg(all(feature = "anyhow", feature = "eyre", feature = "std-error"))]
#![allow(dead_code)]

pub mod error_backends;
