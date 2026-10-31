# ADR 0004: One `parse → IR → validate → emit` pipeline for every macro entrypoint

- Status: Accepted
- Date: 2026-10-31
- Plan: [rewrite plan §7.1](../rewrite-plan.md#71-macro-pipeline-cgp-fork-macro-impl), phase 1b

## Context

After phase 1 every macro entrypoint in `cgp-fork-macro-impl` was a free function that parsed its
input, ran its own checks wherever they happened to sit, and generated code, each in its own
shape. Some checks ran before parsing the body (the "takes no arguments" checks), some inside
`preprocess` (the component-name parameter check), some between parse and emit
(`validate_attributes`). Every check stopped at the first error. The plan asks for one pipeline,
`parse → IR → validate → emit`, ported one entrypoint per commit with every snapshot unchanged.

Rewriting the ≈40 per-entrypoint AST types under `macro_core/types/` into a handful of shared IR
structs in the same step would touch most of the 25k-line crate at once, and could not keep the
"snapshots unchanged after each ported entrypoint" exit criterion reviewable.

## Decision

- Add `crate::ir` with a `Pipeline` trait: `NAME`, `FAMILY`, an associated `Ir` type, and
  `parse`, `validate`, `emit`. `ir::run::<P>` drives the stages; validation errors go to a
  `Diagnostics` accumulator and are reported together (`syn::Error::combine`), and `emit` never
  runs on an IR that failed validation.
- Every one of the 38 entrypoints is a zero-sized `…Macro` type implementing `Pipeline`. The
  existing public functions (`macro_lib::cgp_component`, …) stay and call `run`, so the proc-macro
  shim, the fuzzers and the tests are unchanged.
- Each entrypoint's `Ir` is, for now, the typed input it already had (for example
  `ItemCgpComponent`, `DelegateTable`, `ItemCgpRecord`), or a small enum where an entrypoint has
  two shapes (`CgpGetterIr`, `HasFieldsIr`, `AsyncTraitIr`). Entrypoints that describe the same
  thing already share one: `cgp_component`, `cgp_type`, `cgp_getter` and `derive_provider` all
  parse to `ItemCgpComponent`.
- `IrFamily` names the group each IR belongs to: the six groups of plan §7.1 (`ComponentDef`,
  `Getter`, `ProviderImpl`, `WiringTable`, `Namespace`, `DataShape`) plus `TraitRewrite` for the
  attribute macros that rewrite a trait in place (`async_trait`, `blanket_trait`,
  `cgp_auto_impl`, `cgp_auto_log`), which the plan did not place.
- The entrypoint registry is generated from the pipeline types, so a registry name cannot drift
  from `Pipeline::NAME`.
- Checks move into `validate` when they are semantic rather than grammar: the component-name
  parameter check (now reporting every undeclared parameter) and the attribute rejections of
  `delegate_components!` and `cgp_namespace!`. "Takes no arguments" stays a parse rule, but when
  the body is malformed as well, both errors are reported (`ir::parse_body_without_args`).

## Consequences

- Every expansion snapshot passed unchanged after each ported entrypoint; each port is its own
  commit and every commit builds.
- A new entrypoint implements `Pipeline` and is added to the list in `registry.rs`; the
  `registry_tests` pin the 38 names and check every family is used.
- Error output changes only when an input has several errors: more of them are reported, and the
  first one is the one reported before.
- Left for later: merging the per-entrypoint AST types inside a family into shared IR structs,
  moving the remaining checks that still run during `emit` into `validate`, and the JSON wiring
  metadata of §7.1. The `Pipeline` boundary is where that work plugs in; each step can again be
  done one family at a time with snapshot parity.
