# Draft knowledge-base pages

Comments and docs in this repository point at pages of the separate
[cgp-knowledge-base-fork](https://github.com/Just-Replicant/cgp-knowledge-base-fork) repository.
The pages here are referenced from this repository but did not exist in the knowledge base yet
(checked against commit `8c2f131`). They are drafts in the knowledge base's own layout, ready to
be reviewed and copied there under the same relative paths:

| Draft | Covers |
|---|---|
| `cgp/reference/macros/struct.md` | `Struct!` |
| `cgp/reference/macros/enum.md` | `Enum!` |
| `cgp/reference/macros/cgp_auto_impl.md` | `#[cgp_auto_impl]` |
| `cgp/reference/macros/cgp_auto_error.md` | `#[cgp_auto_error]` |
| `cgp/reference/macros/cgp_auto_log.md` | `#[cgp_auto_log]` |
| `cgp/implementation/entrypoints/struct.md` | how `Struct!` is implemented |
| `cgp/implementation/entrypoints/enum.md` | how `Enum!` is implemented |
| `cgp/implementation/asts/shape.md` | the `StructType` / `EnumType` AST stack |

`bash scripts/check-kb-refs.sh <kb-checkout>` reports a reference that resolves only to one of
these drafts as **pending** rather than dangling. Once a page has been copied into the knowledge
base, delete its draft here.
