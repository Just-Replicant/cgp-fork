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
//! [`run`] drives the three stages, after rejecting input nested too deeply for `syn` to parse
//! without overflowing the stack ([`check_nesting_depth`]); the public functions in `macro_lib`, `extra_macro_lib`
//! and `async_impl` are thin wrappers around `run::<SomePipeline>`, so the proc-macro shim
//! and the fuzzers keep calling plain functions.
//!
//! A pipeline only parses: its `Ir` is one of the seven shared family types ([`ComponentDef`],
//! [`Getter`], [`ProviderImpl`], [`WiringTable`], [`Namespace`], [`DataShape`],
//! [`TraitRewrite`]), and validation and emission live on that type through [`FamilyIr`]. The
//! families are the groups of the rewrite plan (`docs/rewrite-plan.md`, §7.1);
//! `docs/adr/0004-shared-ir-pipeline.md` explains the design.

mod component_def;
mod data_shape;
mod diagnostics;
mod family;
mod getter;
mod namespace;
mod nesting;
mod pipeline;
mod provider_impl;
mod trait_rewrite;
mod wiring_table;

pub use component_def::ComponentDef;
pub use data_shape::{DataShape, RecordDerive, VariantDerive};
pub use diagnostics::Diagnostics;
pub use family::{FamilyIr, IrFamily};
pub use getter::Getter;
pub use namespace::Namespace;
pub use nesting::{MAX_IDENT_LEN, MAX_NESTING_DEPTH, check_nesting_depth};
pub use pipeline::{Pipeline, parse_body_without_args, run};
pub use provider_impl::ProviderImpl;
pub use trait_rewrite::TraitRewrite;
pub use wiring_table::WiringTable;
