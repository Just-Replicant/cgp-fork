//! Entrypoint for the unified handler family (`cgp_fork::handler`, rewrite plan phase 3).
//!
//! Covers the two components, `Handler` and `AsyncHandler`, with borrowing, fallibility and
//! producers expressed in the input and output types; the `Promote` adapters; `Pipe` and
//! `TryPipe`; `MatchWith`/`MatchFields`/`MatchValues` over every strategy and access mode;
//! `BuildWith` with `SetField` and `Merge`; the mode-generic field traits; and, with
//! `compat-0.10`, the `Compat` bridges to the 0.10 family. Several cases port a 0.10 test
//! (named in each file) to the unified names.
//!
//! The global prelude still exports the 0.10 `Handler` and `HandlerComponent`, so the files
//! import those two names explicitly from `cgp_fork::handler`, which takes precedence over the
//! glob imports.
#![allow(dead_code)]
#![allow(clippy::disallowed_names)]

pub mod unified_handlers;
