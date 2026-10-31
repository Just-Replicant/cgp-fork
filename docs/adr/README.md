# Architecture decision records

Each file records one decision that shapes the codebase: the context, the options, the decision,
and its consequences. A record is never rewritten after it is accepted; a later record supersedes it.

| ADR | Title | Status |
|---|---|---|
| [0001](0001-hard-fork.md) | Become an independent project (hard fork) | Proposed |
| [0002](0002-macro-impl-crate.md) | Move the macro implementation into a library crate | Accepted |
| [0003](0003-crate-path-resolution.md) | Resolve the facade's path in generated code | Accepted |
| [0004](0004-shared-ir-pipeline.md) | One `parse → IR → validate → emit` pipeline for every macro entrypoint | Accepted |
| [0005](0005-diagnostics.md) | Readable diagnostics with `#[diagnostic::on_unimplemented]` | Accepted |

Statuses: Proposed, Accepted, Superseded by NNNN, Rejected. New records copy the headings of an
existing one and take the next number.
