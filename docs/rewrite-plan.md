# cgp-fork: full rewrite plan

Analysis of `main` @ `33842521` (2026-10-28, v0.10.1 prepared, unreleased).
Every claim below cites a file in that checkout. Effort figures are estimates.

## Status (2026-10-31, branch `phase-1b-2`)

| Phase | Status | Notes |
|---|---|---|
| 0 Decide and baseline | Done; the hard-fork decision is waiting for the maintainer | [ADR 0001](adr/0001-hard-fork.md) is *Proposed* (hybrid now, hard fork before phase 3); nothing upstream-related was removed. Baseline in [docs/baseline/2026-10-30.md](baseline/2026-10-30.md), `scripts/baseline.sh`. |
| 1 Internals split | Done, except the shared IR (moved to phase 1b) | `cgp-fork-macro-impl` crate ([ADR 0002](adr/0002-macro-impl-crate.md)); crate-path resolution ([ADR 0003](adr/0003-crate-path-resolution.md)); `crates/tests/renamed-dep`; 38 per-entrypoint fuzz targets. Snapshots unchanged; 867 tests pass ([after phase 1](baseline/2026-10-30-after-phase-1.md)). |
| 1b Shared IR | Done | All 38 entrypoints implement `ir::Pipeline` (parse only) into one of seven shared family IR types (the six of §7.1 plus `TraitRewrite`), which implement `ir::FamilyIr` (`validate → emit`, errors combined). Ported one entrypoint, then one family, per commit with every snapshot unchanged ([ADR 0004](adr/0004-shared-ir-pipeline.md)); the registry is generated from the pipelines and its families from the IR types. Open: JSON wiring metadata, moving the last checks out of `emit`. |
| 2 Diagnostics | Done | `#[diagnostic::do_not_recommend]` on generated `DelegateComponent` impls trims the lower error lines; what stable Rust cannot fix is listed in ADR 0005. `#[diagnostic::on_unimplemented]` on `CanUseComponent`, `IsProviderFor`, `DelegateComponent`, `HasField`, `HasFieldMut` and on the consumer, provider and auto-getter traits the macros generate ([ADR 0005](adr/0005-diagnostics.md)). Every `tests/ui` case starts without a type-level string; three new cases. 895 tests pass ([after phase 2](baseline/2026-10-31-after-phase-2.md)). |
| 3 to 6 | Not started | |

What changed from the plan as written:

- The crate path is detected from the caller's `Cargo.toml` (plus a `CGP_FORK_CRATE` override)
  instead of a per-call `crate = …` argument, so renamed projects need no source changes and no
  new dependency was added (see ADR 0003 for the options). Normal dependencies win over
  dev-dependencies, build-dependencies, and target tables whose `cfg` does not match.
- `cgp_preset!` passes the expanding crate's facade path into `with_components!`, so a preset and
  its users may import `cgp-fork` under different names.
- Phase 1b was done in two steps: first every entrypoint on one `parse → validate → emit`
  pipeline with its own input as IR, then the inputs merged into shared family IR enums one
  family per commit, each with snapshot parity (ADR 0004). A seventh family, `TraitRewrite`,
  covers `async_trait`, `blanket_trait`, `cgp_auto_impl` and `cgp_auto_log`, which rewrite a user
  trait and define no component or provider.
- Phase 2's provider-trait message covers both things its `Self` can be, a context or wiring table
  without an entry and a provider without an impl, because rustc reports a method call on an
  unwired context through the provider trait (ADR 0005). Per-symbol readable aliases (§7.2) were
  not added; the `#[cgp_auto_getter]` message names its fields instead.

## 1. What the project is

- **Idea.** cgp-fork is a fork of Context-Generic Programming (CGP, upstream `contextgeneric/cgp`): a Rust
  "language extension" built from traits and proc macros. One interface (a *component*) can have many
  implementations (*providers*); each concrete type (*context*) picks its providers in one wiring table, and
  the choice compiles to a direct call (`README.md`, example in `README.md` lines "Example").
- **Fork relationship.** It merges upstream through commit `69d2854` (`UPSTREAM_COMMIT`), records skipped
  upstream commits (`UPSTREAM_REVIEWED`), and has a weekly sync job (`.github/workflows/upstream-sync.yml`,
  `scripts/upstream-sync.sh`). Its main difference: one facade crate + one macro crate instead of upstream's
  per-concept crates, error backends as features (`README.md` "Differences from upstream").
- **Languages / stack.** Rust 2024 edition, MSRV 1.89, pinned toolchain 1.99.1 (`Cargo.toml`, `rust-toolchain.toml`),
  `#![no_std]` facade (`crates/main/cgp-fork/src/lib.rs`). Runtime dependencies: only `syn 3`, `quote`, `proc-macro2`
  (macro crate) plus optional `anyhow`/`eyre` (`crates/*/*/Cargo.toml`).
- **Size.** 993 tracked files, 842 commits. Facade `src`: 186 files / 5.8k lines. Macro crate `src`: 390 files /
  24.7k lines (incl. in-crate tests). Integration tests: 347 files / 35k lines.

### Layout

| Path | Role |
|---|---|
| `crates/main/cgp-fork/src/core/` | `component` (DelegateComponent, IsProviderFor, CanUseComponent, UseContext, UseDelegate, RedirectLookup, namespaces), `field` (HasField, builders/extractors, Cons/Nil/Either/Void), `base_types` (Symbol, Chars), `types` (abstract types), `error` |
| `crates/main/cgp-fork/src/extra/` | `handler` (9 handler components), `dispatch` (match/build with handlers), `monad`, `log`, `run`, `runtime`, `error`, `field` |
| `crates/main/cgp-fork/src/{anyhow,eyre,std_error}` | error backends behind features |
| `crates/macros/cgp-fork-macro/src/` | 40+ proc-macro entrypoints (`lib.rs`, 1526 lines, mostly docs); `macro_core` (parsers/AST), `macro_lib` (expansions), `extra_macro_*`, `async_impl.rs` |
| `crates/tests/doctests` | unpublished crate that compiles README and macro-crate examples |
| `fuzz/` | one cargo-fuzz target over all entrypoints (`fuzz/fuzz_targets/entrypoints.rs`) |
| `scripts/` | compile-time budget + trend, KB reference check, upstream sync, packaged-crate tests |
| `.github/workflows/` | tests (fmt, clippy, nextest matrix, 3 OS, canary, docs, MSRV, no_std, cargo-hack, deny, package, coverage, compile-time), release-plz, upstream-sync |

## 2. What I ran on the box

| Check | Result |
|---|---|
| `cargo test --workspace --all-features` | 824 passed, 0 failed, 56 ignored (all 56 ignored are macro-crate doc examples) |
| `cargo clippy --workspace --all-features --all-targets -D warnings` | clean |
| Examples `hello`, `swap_context`, `async_component`, `error_backends` | all run and print expected output |
| `bash scripts/compile-time.sh` (N=200 components, wired + checked) | 0.87 s (budget 30 s) |
| Renamed dependency (`cgp = { package = "cgp-fork" }`) + `#[cgp_component]` | **fails**: 3x `E0433 cannot find cgp_fork` |

## 3. Critique of the current design

### Strengths (keep)
- Macro code is defensive: 0 `unwrap()`, 0 `panic!`, 1 `expect` in non-test macro source
  (`macro_core/types/ident/path_with_type_args.rs:77`); every entrypoint returns `syn::Result` and maps errors to
  `into_compile_error` (`lib.rs`).
- Strong verification culture: 824 tests, trybuild UI tests (`tests/ui/`), insta snapshots, coverage gate 90%,
  fuzzing, MSRV/no_std/feature-powerset jobs, packaged-crate tests (`.github/workflows/tests.yml`).
- Tiny dependency footprint and true `no_std` core.

### Problems
1. **Diagnostics are the #1 user pain.** A missing field yields
   `HasField<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>>` (`tests/ui/missing_dependency.stderr`).
   Missing wiring says "this is an associated function, not a method" (`tests/ui/missing_wiring.stderr`), which is
   misleading. The crate uses `#[diagnostic::do_not_recommend]` twice (`core/field/traits/has_field.rs:75`,
   `has_field_mut.rs:18`) but never `#[diagnostic::on_unimplemented]`, though MSRV 1.89 supports it. The README points
   users to an external tool (`cargo cgp check`) for readable errors.
2. **Hard-coded crate path.** Generated code always names `::cgp_fork::macro_prelude::…`
   (`crates/macros/cgp-fork-macro/src/macro_core/macros/export.rs:7`). Renaming the dependency breaks every macro
   (verified above). No `crate = …` override exists.
3. **Combinatorial API surface.** The handler family is 9 components: Computer, ComputerRef, AsyncComputer,
   AsyncComputerRef, TryComputer, TryComputerRef, Handler, HandlerRef, Producer (`extra/handler/components/*.rs`),
   with matching promote adapters (`extra/handler/providers/promote*.rs` + `try_promote.rs`: 5 files, 417 lines). Dispatch has 7 near-copies
   (`extra/dispatch/providers/with_handlers/`: match/match_first × owned/ref/mut + build); e.g.
   `match_with_handlers_ref.rs` differs from `match_with_handlers.rs` only in `&'a Input` and `Extractor<'a>`.
   Extractor traits repeat the same split (`HasExtractor`, `HasExtractorRef`, `HasExtractorMut` in `core/prelude.rs`).
4. **Async story is split and confusingly named.** `#[async_trait]` (`async_impl.rs`) shares its name with the
   popular `async-trait` crate but behaves differently (never `Send`, rejects arguments). `Send` is handled by a
   duplicated component pair `CanRun` / `CanSendRun` (`extra/run/mod.rs`).
5. **Under-tested, under-specified extras.** `monad` (type-level monad transformers, `extra/monad/`) has 3 tests,
   `runtime` 1 test, `presets` 2, `logging` 4 (`cargo test` output). `log` is only a trait (`extra/log/traits/can_log.rs`)
   with no backend; `runtime`/`run` have no executor integration.
6. **Monolithic macro crate.** Parsers and codegen live inside the proc-macro crate, so fuzzing has to include
   modules by `#[path]` and cannot be split per entrypoint (`docs/IMPROVEMENT_PLAN.md` I-9 "Partial"; `fuzz/README.md`).
   There is no shared intermediate representation: each entrypoint parses into its own AST type under
   `macro_core/types/*` (≈40 directories).
7. **Docs live elsewhere.** The construct reference moved to a separate repository (`sibling-projects.md`); 8 drafted
   pages wait in `docs/kb-pages/` to be copied over (`IMPROVEMENT_PLAN.md` F-5 "Partial"). 56 macro-crate examples
   are still `rust,ignore` in the crate itself (test output).
8. **Fork tax.** A large share of process (UPSTREAM_COMMIT/REVIEWED, sync workflow, merge rules in `CONTRIBUTING.md`)
   exists to stay mergeable with upstream. Any real rewrite ends that, so it must be a deliberate decision.
9. **Process overhead vs. size.** `tests.yml` is 379 lines and 13+ jobs for a 6k-line library; release-plz config
   plus PAT/secret requirements (`release-plz.toml`, `IMPROVEMENT_PLAN.md` F-2).
10. **Hygiene.** `.gitignore` ignores `docs/IMPROVEMENT_PLAN.md` while the file is tracked; `homepage` duplicates
   `repository` (`Cargo.toml`); the compile-time benchmark only measures flat wiring (`scripts/compile-time.sh`), not
   deep handler stacks or wide records.

## 4. Goals of the rewrite

1. **Errors a Rust beginner can read** without an external tool.
2. **Smaller, orthogonal API**: one concept per axis (sync/async, owned/ref/mut, fallible) instead of a product of names.
3. **Robust macros**: shared IR, crate-path override, fuzzable as a normal library.
4. **Wiring is checked by default**, not opt-in.
5. **Batteries for real apps**: tracing logger, async runtime adapter, test mocks, wiring graph export.
6. **Docs in the repo**, every example compiled.
7. Keep: `no_std` core, tiny deps, compile-time cost ≤ today, MSRV ≤ 1.89 for the core.

Non-goals: runtime DI containers, reflection, nightly-only features in the default build.

## 5. Strategic decision first: hard fork

Recommendation: **declare an independent project** (working name `cgp-fork` 1.0, or rename to make the break visible),
stop merging upstream (retire `UPSTREAM_COMMIT`, `UPSTREAM_REVIEWED`, `upstream-sync.*`), and port upstream ideas by
hand when useful. Keep a `compat-0.10` feature that re-exports old names as deprecated aliases for one minor series.
If the user wants to keep tracking upstream instead, phases 1–2 below still apply, but phases 3–4 must be proposed
upstream first.

## 6. New architecture

```
cgp-fork            (facade; features select modules; the only crate users add)
├── cgp-fork-core      no_std: DelegateComponent, IsProviderFor, CanUseComponent, UseContext, UseDelegate,
│                      namespaces/RedirectLookup, abstract types, error traits
├── cgp-fork-data      no_std: Symbol/Chars, HasField(s), records, variants, builders, extractors (one Mode param)
├── cgp-fork-handler   unified Handler + dispatch + compose/pipe; monad layer behind `unstable-monad`
├── cgp-fork-std       alloc/std: error backends (anyhow, eyre, std-error), tracing logger, runtime adapters
├── cgp-fork-macro     thin proc-macro shim (≈ entrypoint fns only)
├── cgp-fork-macro-impl  normal lib: parse → IR → validate → codegen; unit-tested, snapshot-tested, fuzzed
└── cgp-fork-test      dev-only: mock providers, wiring assertions
xtask/                 codegen checks, compile-time bench, migration codemod, docs build
book/                  mdBook: guide + reference, all examples compiled
```

Why: keeps the fork's single-dependency UX (facade) while making internals testable. Splitting `macro-impl` out is
the prerequisite listed in `IMPROVEMENT_PLAN.md` I-9 for per-entrypoint fuzzing.

## 7. Module-by-module redesign

### 7.1 Macro pipeline (`cgp-fork-macro-impl`)
- One IR: `ComponentDef`, `ProviderImpl`, `WiringTable`, `Namespace`, `Getter`, `DataShape`. Every entrypoint in
  `lib.rs` becomes `parse → ir → validate → emit`.
- `CrateRoot` resolved once: default `::cgp_fork`, overridable with `#[cgp(crate = path)]` / `crate = …` key; fixes
  §3.2. Re-export a hidden `__private` module instead of `macro_prelude` names.
- Validation pass emits all errors at once (`syn::Error::combine`) with spans on the user's tokens.
- Emit a hidden `const` with JSON wiring metadata per `delegate_components!` (feature `inspect`) for tooling (7.7).
- Keep the existing rejection tests (`macro_tests/parser_rejections/`) as the spec; port them to the IR.

### 7.2 Core (`cgp-fork-core`)
- Add `#[diagnostic::on_unimplemented]` to `DelegateComponent`, `IsProviderFor`, `CanUseComponent`, `HasField`, and
  every generated consumer/provider trait, e.g. "`{Self}` has no provider wired for `{Component}`; add it to
  `delegate_components!`".
- Macros also emit a readable alias per symbol (`type __field_name = Symbol!("name")`) in docs/notes where possible.
- Keep `DelegateComponent<Key>` as the single primitive; keep namespaces (`cgp_namespace!`, `#[prefix]`) — they are
  well tested (46 tests).

### 7.3 Data (`cgp-fork-data`)
- Merge `HasExtractor` / `HasExtractorRef` / `HasExtractorMut` into `HasExtractor<M: Mode>` with `Owned`, `Ref<'a>`,
  `Mut<'a>`; same for `HasFields`/`HasFieldsRef`, `ToFields`/`ToFieldsRef`, `MapType`/`MapTypeRef`.
- Keep `Symbol<LEN, Chars<…>>` (const `&str` generics are not stable); add an optional nightly
  `cfg(cgp_unstable_adt_const)` experiment for `Symbol<"name">`.
- New: `serde` feature to (de)serialize any `HasFields` record; `Default`-filling builder kept from `extra/field`.

### 7.4 Handlers and dispatch (`cgp-fork-handler`)
- Replace 9 components with 2: `Handler<Code, Input>` (sync, `type Output`) and `AsyncHandler<Code, Input>`
  (`fn handle(..) -> impl Future<Output = Self::Output>`). Ref-ness is in the `Input` type (`&'a T`), exactly as
  `match_with_handlers_ref.rs` already does; fallibility is `Output = Result<_, _>`; producers are `Input = ()`.
- One `Promote<P>` adapter family replaces `promote*.rs`.
- Dispatch: `MatchWith<Handlers, Strategy = All|First>` generic over the extractor `Mode` → 7 files become 2.
- Monad layer moves behind `unstable-monad` until it has real docs and ≥ 20 tests.
- Compatibility: `compat-0.10` provides `ComputerRef`, `TryComputer`, `MatchFirstWithValueHandlersRef`, … as aliases
  or thin wrappers with `#[deprecated]`.

### 7.5 Async, run, runtime
- Rename `#[async_trait]` → `#[cgp_async]`; accept `#[cgp_async(Send)]` to add `+ Send` to every desugared future.
  Drop the `CanRun`/`CanSendRun` duplication (`extra/run/mod.rs`) in favor of one component plus the flag.
- `cgp-fork-std` features `tokio`, `smol`: ready-made `HasRuntime` providers (spawn, sleep, timeout).

### 7.6 Errors and logging
- Keep the 3 backends; add `#[cgp_auto_error]` derived error enums with `Display` and source chains.
- `CanLog` gets a `tracing` provider (`log-tracing` feature) and a `defmt` one for embedded `no_std` users.

### 7.7 New capabilities (thinking big)
1. **Checked by default.** `delegate_components!` on a non-generic context also emits the `check_components!` trait;
   `#[unchecked]` opts out. Behavior change; folds in `delegate_and_check_components!`.
2. **Wiring graph.** `cargo xtask wiring` (later a `cargo cgp graph` contribution) reads the IR metadata and prints a
   Mermaid/DOT graph of context → component → provider → dependencies.
3. **Test mocks.** `#[cgp_component(Greeter, mock)]` generates `MockGreeter` that records calls and returns
   configured values via a field on a test context (`cgp-fork-test`).
4. **Presets 2.0.** Versioned, overridable bundles (`cgp_preset!` exists with 2 tests) with an explicit
   "override" syntax and a check that every override names an existing component.
5. **Profiles.** Swap a whole wiring set with a cfg/feature (`#[cfg_wiring(test)]`), generalising the
   `swap_context` example.
6. **Docs site in repo.** `book/` with guide, reference, error catalog (one page per `on_unimplemented` message).

## 8. Behavior changes (breaking, 1.0)

| Area | Today | After |
|---|---|---|
| Crate path | hard-coded `::cgp_fork` | `crate = …` override, renamed deps work |
| Wiring checks | opt-in macro | on by default for concrete contexts |
| Handler names | 9 components | `Handler`, `AsyncHandler` (+ deprecated aliases) |
| Dispatch | 7 providers | `MatchWith<_, All/First>` over a mode |
| Async attr | `#[async_trait]`, never Send | `#[cgp_async]`, `#[cgp_async(Send)]` |
| Run | `CanRun` + `CanSendRun` | one `CanRun` |
| Monad | stable API | `unstable-monad` feature |
| Errors | type-level strings | custom messages naming component/field |

## 9. Phased roadmap

| Phase | Weeks (est.) | Deliverable | Exit criteria |
|---|---|---|---|
| 0 Decide & baseline (**done**, decision pending) | 1 | hard-fork decision, ADRs, baseline metrics | 824 tests green; compile bench baseline recorded |
| 1 Internals split (**done**, IR moved to 1b) | 3 | `macro-impl` crate, IR, crate-path override | all existing tests + snapshots unchanged; renamed-dep test passes; per-entrypoint fuzz targets |
| 1b Shared IR (**done**) | 3 | one IR for every entrypoint | snapshots unchanged after each ported entrypoint |
| 2 Diagnostics (**done**) | 2 | `on_unimplemented` everywhere, new UI tests | every `tests/ui` case reads without type-level strings in the first line |
| 3 Handler/data unification | 4 | `Handler`/`AsyncHandler`, mode-generic extractors/dispatch, compat aliases | old tests pass via `compat-0.10`; new tests ported |
| 4 Async/run/runtime | 2 | `#[cgp_async]`, tokio/smol adapters | async tests on 3 OS |
| 5 New capabilities | 4 | checked-by-default, mocks, wiring graph, tracing | each feature has guide page + example + tests |
| 6 Docs & 1.0 | 2 | `book/`, migration guide, codemod, 1.0-rc | semver-checks report reviewed; examples compiled |

## 10. Testing and CI

- Keep: trybuild UI tests (expand to one per diagnostic), insta snapshots (move to IR + expansion), nextest, MSRV,
  no_std, cargo-hack, cargo-deny, coverage ≥ 90%, semver-checks.
- Add: IR unit tests and property tests (`proptest`) in `macro-impl`; one fuzz target per entrypoint; compile-time
  bench with three shapes (flat 200, 10-deep handler pipeline, 64-field record), fail on > 25% regression.
- Simplify `tests.yml` into a reusable `xtask ci` so local and CI run the same steps.
- Snapshots that depend on rustc output stay pinned to the toolchain (as today, `CONTRIBUTING.md`).

## 11. Migration

- 0.11: deprecations only (new names added, old names `#[deprecated]`), `crate =` override, diagnostics.
- 1.0: old names only under `compat-0.10`; codemod `cargo xtask migrate` rewrites imports and handler names with `syn`.
- Migration guide maps every removed name to its replacement (table generated from the compat module).

## 12. Risks

| Risk | Mitigation |
|---|---|
| Losing upstream fixes after hard fork | monthly manual review of upstream changelog; port by hand |
| Unified generic handlers slow the trait solver or hit recursion limits | compile bench gates each phase; keep specialised providers if a shape regresses |
| `on_unimplemented` can't render type-level strings | message names component and context; field names via macro-emitted notes |
| Breaking downstream users | compat feature, codemod, 0.11 deprecation release first |
| Scope creep (mocks, graph, runtimes) | phase 5 items independent; ship any subset |
| New trait solver changes behavior | canary job on beta/nightly already exists; keep it |

## 13. Quick wins before the rewrite (≤ 1 day each)

1. ~~Add `#[diagnostic::on_unimplemented]` to `CanUseComponent` and `DelegateComponent`.~~ Done in phase 2.
2. ~~Add a crate-path override to `export_constructs!` (`macro_core/macros/export.rs`).~~ Done in phase 1.
3. Remove `docs/IMPROVEMENT_PLAN.md` from `.gitignore` (it is tracked).
4. Drop `homepage` (duplicates `repository`).
5. ~~Add the renamed-dependency case to the UI/integration tests.~~ Done in phase 1 (`crates/tests/renamed-dep`).
