//! The shared `parse → IR → validate → emit` pipeline that every macro entrypoint runs.
//!
//! Each entrypoint is a zero-sized type implementing [`Pipeline`]:
//!
//! 1. **parse** turns the attribute and body tokens into the entrypoint's IR, the typed
//!    description of what the user asked for. Grammar errors stop here.
//! 2. **validate** checks the IR for semantic mistakes and reports *all* of them through
//!    [`Diagnostics`] (combined with `syn::Error::combine`), each spanned on the user's tokens.
//! 3. **emit** generates the output tokens from an IR that passed validation.
//!
//! [`run`] drives the three stages; the public functions in `macro_lib`, `extra_macro_lib`
//! and `async_impl` are thin wrappers around `run::<SomePipeline>`, so the proc-macro shim
//! and the fuzzers keep calling plain functions.
//!
//! Every pipeline also names its [`IrFamily`], the kind of IR it produces (component
//! definition, provider impl, wiring table, …). The families are the groups the rewrite
//! plan (`docs/rewrite-plan.md`, §7.1) unifies; `docs/adr/0004-shared-ir.md` explains the
//! design and what is left to share between the entrypoints of one family.

mod diagnostics;
mod family;
mod pipeline;

pub use diagnostics::Diagnostics;
pub use family::IrFamily;
pub use pipeline::{Pipeline, parse_body_without_args, run};
