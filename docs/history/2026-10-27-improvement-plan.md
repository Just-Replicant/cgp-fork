# cgp-fork: Analysis and Improvement Plan (2026-10-27, historical)

> **Historical record.** This is the analysis of v0.9.3 (`58c50861`) and the status of the work
> that became v0.10.0 (PR #14). It is kept for context and is not updated. The current plan, with
> each item's status, is [`docs/IMPROVEMENT_PLAN.md`](../IMPROVEMENT_PLAN.md).

> ## Implementation status (2026-10-27, branch `improvements`, v0.10.0)
>
> Everything was implemented locally on the `improvements` branch (18 commits on top of `58c50861`, the last adding this file),
> with nothing pushed and no GitHub connection. All local verification is green (see the end of this
> section). Paths in the original findings below refer to the pre-change layout: `crates/macros/cgp-fork-macro/shared/…`
> now lives at `crates/macros/cgp-fork-macro/src/…`.
>
> | Item | Status | Commit(s) | Notes |
> |---|---|---|---|
> | **F-1 / Q1** `#[async_trait]` default bodies + ignored args | ✅ Done | `5ee342fb` | Default bodies wrapped in `async move`; arguments rejected (breaking). Integration, macro, and ui tests added. |
> | **F-2 / P2** feature graph | ✅ Done (breaking) | `a6cb437e` | `extern crate alloc` gated on `alloc`; `anyhow`/`eyre`/`std-error` imply `alloc`; eyre's `std` requirement and forced `auto-install` documented. `auto-install` is kept on purpose: without it every `eyre::Report` panics unless the app installs a hook. |
> | **F-3 / P6** invariant `unwrap`/`expect` | ✅ Done | `28b4c12f` | Replaced with `syn::Error`s. Only `PathWithTypeArgs::ident()` keeps an `expect`, on a type invariant the parser now enforces. |
> | **F-4 / P1** test-only `build.rs` | ✅ Done | `eea9fb5d` | Build script and `readme_*.rs` shims removed; the README examples are doctests. |
> | **F-5 / Q6** tests shipped in packages | ✅ Done | `f44b070d` | `exclude` in both manifests; the CI `package` job asserts it. |
> | **F-6 / L4** compile-time cost | ✅ Done (partial) | `d0850211` | `scripts/compile-time.sh` plus a CI job: 200 components wired and checked, with a budget in `scripts/compile-budget.txt` (30 s; 0.63 s locally). Each run uploads `compile-time.txt` as the `compile-time` artifact. **Remaining:** no trend over time; an artifact alone is not tracking (current plan, I-7). |
> | **F-7 / Q4 / P4** stale docs | ✅ Done (partial) | `eea9fb5d`, `4d791365` | Backend READMEs retitled, all defunct crate names replaced, broken link fixed. **Remaining:** the 113 distinct `cgp-knowledge-base-fork/…` references weren't verified at the time, because no knowledge-base checkout was available. (Since checked: see the current plan, F-5.) |
> | **F-8** no compiled doc examples | ✅ Done (partial) | `4d791365`, `6f8159fe` | The facade has 23 compiled doctests (it had 0), including both READMEs. Compiling them found 6 doc bugs, all fixed. The root README is included only when `.cargo/config.toml` sets `cgp_fork_repository_doctests`, so the published crate's `cargo test --doc` does not `include_str!` a file outside the package. **Remaining:** the macro crate's 56 `rust,ignore` examples stay ignored. A proc-macro crate can't run its own macros in doctests without a dev-dependency cycle on `cgp-fork`; they are covered by the facade doctests and the expansion snapshots. |
> | **F-9 / Q4** malformed CHANGELOG | ✅ Done | `8910a0c1` | 0.9.3 entry repaired; 0.10.0 entry with a **Breaking changes** section. |
> | **F-10 / Q5 / L1** blanket `allow(dead_code)`, `#[path]` layout | ✅ Done | `d2107494`, `c8aaa59e` | `shared/` moved under `src/` as normal modules. Dead items removed (`PathHeadOrType`, `derive_is_provider_for`, `is_empty`, `UseProviderAttribute::context_type`). Syntax-only token fields get a field-level `allow(dead_code, reason)`. |
> | **F-11 / Q5** leftover allows | ✅ Done | `c8aaa59e`, `a6cb437e` | `mixed_script_confusables` and the duplicate `extern crate alloc` removed. The `large_enum_variant` allows on parser enums are kept (compile-time only). |
> | **F-12 / L7** pedantic clippy | ✅ Done (subset) | `9326c3cd` | `[workspace.lints.clippy]` enforces 5 pedantic lints; about 120 findings fixed. `needless_pass_by_value` (the entrypoints share a by-value signature) and `redundant_closure_for_method_calls` (its fix breaks privacy) were evaluated and left out. |
> | **F-13 / L2 / L3** test gaps | ✅ Done (partial) | `5ee342fb`, `6e73f0bb`, `d4df9efa`, `10ff9ac1` | Added `#[async_trait]` regressions, a trybuild `tests/ui` suite (6 cases), and a deterministic panic fuzzer over all 38 macro entrypoints (locally 3 seeds × 20k iterations × 38 entrypoints, no panics). CI now runs no-default tests, doctests, and coverage. **Remaining:** coverage runs only in CI (`cargo-llvm-cov` wasn't run locally). The fuzzer is in-tree rather than `cargo-fuzz`: libFuzzer can't link a `proc-macro` crate as a library without splitting the parsers into a separate crate. |
> | **F-14 / Q2 / Q3 / P3** CI and supply chain | ✅ Done (files only) | `d0850211` | Actions SHA-pinned (checkout v7.0.1, cache v6.1.0, upload-artifact v7.0.2). nextest 0.9.150 is SHA-256-verified. New jobs: MSRV 1.89, doctests, rustdoc, macOS and Windows, cargo-hack, deny, audit, semver-checks, package, coverage, fuzz, compile-time. Also `deny.toml` and Dependabot. Validated with actionlint and a YAML parser. **Remaining:** none of it has been run on GitHub Actions. |
> | **F-15 / P5** release safety | ✅ Done | `d0850211` | The release job runs the test suite before publishing, and the release-pr job warns when `RELEASE_PLZ_TOKEN` is unset. release-plz no longer releases on `refactor`. Making the PAT mandatory is left to the maintainer, because it needs a repository secret. |
> | **Q7** `cargo update` | ✅ Done | `d0b61e69` | |
> | **L5** upstream sync | ✅ Done (not executed) | `d0850211` | `UPSTREAM_COMMIT`, `scripts/upstream-sync.sh`, and CONTRIBUTING docs. The script fetches from GitHub, so it wasn't run here. Upstream was one docs commit ahead (`eca0c5f`) at analysis time. |
> | **L6** examples and onboarding | ✅ Done | `6f8159fe`, `d0850211` | 4 examples (`hello`, `swap_context`, `error_backends`, `async_component`). README example, feature table, and upstream differences. CONTRIBUTING, SECURITY, and issue/PR templates. |
> | §5 diagnostics | ✅ Done (one) | `10ff9ac1` | `#[cgp_provider(Greeter)]` now explains that it needs `GreeterComponent`, instead of rustc's "add `dyn`". |
> | hygiene | ✅ Done | `b0e18b55` | 3 accidentally committed `.pending-snap` files untracked, and `*.pending-snap` ignored. |
>
> **Breaking changes (0.9.3 → 0.10.0):** `#[async_trait]` rejects arguments. `alloc` is no longer linked unless the `alloc` feature (or a backend) is on. The backends imply `alloc`.
>
> **Local verification at HEAD (all PASS):**
> - **Format, build, lint:** nightly rustfmt; build with all features and with none; clippy `-D warnings` (all features, no default).
> - **Tests:** nextest with all features (712 passed) and with none (699 passed); `cargo test` (including doctests) in both modes; facade doctests 23 passed.
> - **Docs and examples:** examples build; rustdoc `-D warnings --cfg docsrs`.
> - **no_std (`thumbv7m-none-eabi`):** default, core-only, and `anyhow,std-error`.
> - **MSRV 1.89:** `check --all-targets` and no-default.
> - **Feature matrix:** `cargo hack --feature-powerset` (host, and thumbv7m without eyre).
> - **Dependency scanning:** `cargo deny check`; `cargo audit --deny warnings`.
> - **Semver:** `cargo semver-checks` against 0.9.3 (minor bump in 0.x, so the checks are skipped as expected).
> - **Packaging:** `cargo package --workspace`; `cargo publish -p cgp-fork-macro --dry-run`.
> - **Compile-time budget and workflow files:** compile-time budget; actionlint; YAML parse.
> - **Per-commit:** each of the 16 code commits builds its test targets.

---

**Repository:** https://github.com/Just-Replicant/cgp-fork (HEAD `58c50861`, v0.9.3, last commit 2026-10-10 17:33 UTC)
**Analysed:** 2026-10-27. The project code was not changed. Experiments ran in throwaway copies of the repository.

---

## 1. Executive summary

`cgp-fork` is a fork of **Context-Generic Programming (CGP)** ([contextgeneric/cgp](https://github.com/contextgeneric/cgp)) by Soares Chen. CGP is a Rust "language extension" built from proc macros and traits. One trait (a *component*) can have many interchangeable implementations (*providers*), and each concrete type (a *context*) picks its providers through a wiring table (`delegate_components!`). The compiler resolves the table statically, so there is no runtime cost.

The fork's main structural change is **consolidation**. Upstream ships many crates (`crates/core`, `extra`, `standalone`, ...; v0.8.0). The fork ships two: the facade **`cgp-fork`** (`no_std` library, with optional `anyhow`, `eyre`, and `std-error` error backends) and **`cgp-fork-macro`** (all proc macros, about 386 source files pulled in with `#[path]` modules). Releases are automated with release-plz and crates.io trusted publishing. The fork includes upstream through `69d2854` (PR #278). Upstream is only one docs commit ahead (`eca0c5f`, #280), so it is in sync.

**Overall health is good.** It builds cleanly, all 706 tests pass, clippy is clean with `-D warnings` in both feature modes, nightly rustfmt is clean, rustdoc builds with `-D warnings`, the `no_std` target check passes, the declared MSRV 1.89 compiles, and `cargo audit` finds no advisories. Most of what remains is correctness edge cases in the macros, packaging and feature hygiene, documentation left stale by the crate consolidation, and CI coverage gaps. There are no critical security issues.

### Top issues
| # | Severity | Issue |
|---|---|---|
| 1 | **High** (bug) | `#[async_trait]` breaks any trait whose `async fn` has a **default body**. The compile error is `` `{integer}` is not a future ``. It also silently ignores any attribute arguments. |
| 2 | **Medium** | The library ships a **`build.rs` that only serves the test suite** (it turns README examples into tests). Every downstream user compiles and runs it. |
| 3 | **Medium** | Feature semantics are inconsistent. `extern crate alloc` is unconditional, so `alloc` gates almost nothing. `anyhow` and `std-error` don't imply `alloc`. `eyre` silently breaks `no_std` and forces `auto-install` on all users. |
| 4 | **Medium** | Stale post-consolidation docs. The user-facing module docs (on docs.rs) are titled `cgp-fork-error-anyhow` / `-eyre` / `-std`. About 19 files still name crates that no longer exist. All 86 doc examples are `ignore`, so no documentation example is compile-checked. |
| 5 | **Medium** (CI/supply chain) | CI never checks the MSRV (1.89) that README and Cargo.toml promise. It never runs doctests (nextest skips them). It installs `cargo-nextest` from an unpinned, unverified `latest` URL, while release-plz is pinned and SHA-256-verified. There is no dependency or advisory scanning and no semver check for a published library. |

### Top recommended actions
1. Fix `#[async_trait]`: wrap default bodies in `async move { … }`, reject unknown attribute arguments, and add regression tests.
2. Remove the build script from the published crate. Turn the three README examples into real doctests (or move the generator into a dev-only test crate), and add `include`/`exclude` to the manifests so 340 test files and the macro tests stop shipping to crates.io.
3. Make the features honest: `anyhow`/`std-error` imply `alloc`, gate `extern crate alloc` on `alloc`, document that `eyre` needs `std`, and add a CI feature matrix with `cargo hack`.
4. Do one documentation sweep: retitle the backend READMEs, replace stale crate names, fix the broken relative link, clean the 0.9.3 CHANGELOG, and convert `rust,ignore` examples to compiled doctests where possible.
5. Harden CI: add an MSRV job, `cargo test --doc`, a pinned and checksummed nextest, SHA-pinned `actions/checkout`, `cargo deny`/`cargo audit`, `cargo semver-checks`, `cargo package` verification, and Dependabot.

---

## 2. Project overview

| Aspect | Details |
|---|---|
| Purpose | Compile-time dependency injection / pluggable trait implementations for Rust (CGP) |
| Language / edition | Rust, edition 2024, resolver 3 |
| Toolchain | Pinned `1.98.1` (`rust-toolchain.toml:2`). Current stable is 1.99.1. Declared MSRV `1.89` (`Cargo.toml:12`). |
| Workspace | `crates/main/cgp-fork` (facade lib, `no_std`, `#![deny(missing_docs)]`) and `crates/macros/cgp-fork-macro` (proc-macro) |
| Size | 913 `.rs` files, ~63.5k lines. Library: 186 src files. Macro crate: 386 files (`shared/macro_core`, `macro_lib`, `extra_macro_core`, `extra_macro_lib`, `test_util_lib`). 340 integration-test files. |
| Runtime deps | `syn 3`, `quote`, `proc-macro2`, `itertools 0.14`, optional `prettyplease 0.3` (macro); optional `anyhow 1`, `eyre 0.6` (lib). 42 crates in the lockfile in total. |
| Dev deps | `insta` (inline snapshots of macro expansions), `futures` |
| Features | `default=["full"]`, `full=["alloc"]`, `alloc`, `anyhow`, `eyre`, `std-error`; macro crate `snapshot` |
| Public surface | ~37 proc macros (`cgp_component`, `cgp_impl`, `cgp_provider`, `cgp_fn`, `cgp_getter`, `cgp_type`, `delegate_components!`, `check_components!`, `cgp_preset`, `cgp_namespace!`, `Symbol!`, `Product!`, `Sum!`, derives `HasField`, `HasFields`, `CgpData`…, `cgp_computer`, `cgp_producer`, `cgp_auto_dispatch`, `async_trait`, …) plus core traits (`DelegateComponent`, `IsProviderFor`, `HasField`, `HasErrorType`, `CanRaiseError`, `CanWrapError`, …), and in `extra`: handlers, dispatch, monads, runtime, logging. |
| Tests | 21 concept-organized integration targets (`tests/<concept>_tests.rs` + `tests/<concept>/`), macro unit and rejection tests in `src/macro_tests`. Many tests are inline `insta` snapshots of the generated code. "Accepts, but the expansion fails to compile" UI tests live in the sibling repo `cargo-cgp`. |
| CI | `.github/workflows/tests.yml` (nightly fmt, clippy ×2, nextest, `thumbv7m` no_std check) and `release-plz.yml` (release PR + publish via OIDC) |
| History | 793 commits: 261 upstream (Soares Chen, since 2023-09) and 529 by Just-Replicant (512 of them since 2026-10-01). Tags v0.9.1–v0.9.3. Conventional-commit style. |
| Docs | Root README (short), crate README (rendered as the crate docs), per-backend READMEs (rendered as module docs). Deep docs are kept externally in `cgp-knowledge-base-fork` (`sibling-projects.md`). |

---

## 3. Build / run / test results (local, no credentials)

| Check | Command | Result |
|---|---|---|
| Build | `cargo build --workspace --all-features` | ✅ 8.4 s clean |
| Tests | `cargo test --workspace --all-features --no-fail-fast` | ✅ **706 passed, 0 failed**. Doc-tests: **0 run, 86 ignored** (all `rust,ignore`). |
| Clippy (all features) | `cargo clippy --all-features --all-targets -- -D warnings` | ✅ clean |
| Clippy (no default) | `cargo clippy --no-default-features --all-targets -- -D warnings` | ✅ clean |
| Clippy pedantic (informational) | `-W clippy::pedantic` | ~200 warnings, mostly style (`explicit_iter_loop` 32, `semicolon_if_nothing_returned` 19, `needless_pass_by_value` 13, `used_underscore_binding` 12, `ref_option` 8, `missing_errors_doc` 8 …) |
| Rustfmt | `cargo +nightly fmt --all -- --check` | ✅ clean |
| Rustdoc | `RUSTDOCFLAGS="-D warnings --cfg docsrs" cargo doc --all-features --no-deps` | ✅ clean (not run in CI) |
| no_std | `cargo check -p cgp-fork --target thumbv7m-none-eabi` (default and `--no-default-features`) | ✅. With `--features anyhow` ✅, `std-error` ✅, **`eyre` ❌** (`E0463: can't find crate for std`). |
| Feature matrix (host) | `--no-default-features` + each of `""`, `alloc`, `anyhow`, `eyre`, `std-error`, all | ✅ all compile |
| MSRV | `cargo +1.89 check --workspace --all-features --all-targets` | ✅ compiles (verified manually only; no CI job) |
| Security advisories | `cargo audit` (RustSec DB, 1379 advisories) | ✅ 0 vulnerabilities / 0 warnings across 42 crates |
| Dependency freshness | `cargo update --dry-run` | 12 semver-compatible bumps available (e.g. `syn 3.0.6→3.0.9`, `libc`, `memchr`, `rustix`). Nothing outdated in a major-version sense. |
| Packaging | `cargo package --list`, `cargo publish -p cgp-fork-macro --dry-run` | ✅ verifies. The `cgp-fork` package includes `build.rs` and **340 `tests/` files**. The macro package includes 39 `macro_tests` files. |
| Dead-code scan | Removed the blanket `#[allow(dead_code, unused_imports)]` in a scratch copy, then ran `cargo check -p cgp-fork-macro` | 17 warnings. 4 are genuinely dead items (see F-9). The rest are unread syn token fields, which is normal. |
| Probe: `#[async_trait]` default body | scratch crate | ❌ **compile error** (F-1) |
| Probe: `PanicOnError` panic location | scratch crate | ✅ reports the caller's line (`#[track_caller]` propagates correctly) |

Nothing contacted an external service with credentials. Network access was limited to crates.io, static.rust-lang.org, the RustSec DB, and anonymous read-only git clones.

---

## 4. Findings

Severity: **High** = incorrect behaviour users can hit. **Medium** = real cost, risk, or misleading to users. **Low** = hygiene or polish.

### Correctness / bugs

**F-1 (High). `#[async_trait]` breaks async methods that have default bodies**
- `crates/macros/cgp-fork-macro/src/async_impl.rs:10-25`. The macro rewrites `async fn f() -> T` to `fn f() -> impl Future<Output = T>` and clears `asyncness`, but it leaves `trait_fn.default` (the body) unchanged. A default body therefore returns `T` where `impl Future` is expected.
- Repro (verified):
  ```rust
  #[async_trait]
  pub trait HasDefault { async fn value(&self) -> u32 { 7 } }
  // error[E0277]: `{integer}` is not a future
  ```
- Related: `src/lib.rs:1368` takes `_attr` and ignores it, so `#[async_trait(?Send, whatever = 1)]` compiles silently. Users coming from the `async-trait` crate may assume `?Send` does something.
- `async_impl.rs:30`: when the input is not a trait (`_ => item`), the macro passes it through unchanged and silently. That is harmless on `impl` blocks but undocumented. It may be inherited from upstream (not checked).

**F-2 (Medium). Features don't describe what the code needs**
- `crates/main/cgp-fork/src/lib.rs:6`: `extern crate alloc;` is unconditional, but `Cargo.toml:22` defines an `alloc` feature that only gates `DebugError`/`DisplayError` (`src/extra/error/mod.rs:1-6`). As a result, "no_std without alloc" is not actually supported, although the feature names suggest it is.
- `Cargo.toml:23,25`: `anyhow` and `std-error` use `Box`/`alloc` but don't enable `alloc`. They only work because of the unconditional `extern crate`.
- `Cargo.toml:24,36`: `eyre` requires `std` (fails on `thumbv7m`), but nothing documents this, and the crate is advertised as `no_std` (`categories = ["no-std"]`).
- `Cargo.toml:36`: `eyre` is compiled with `auto-install`. Features are additive, so no downstream user can opt out of the global hook auto-install. This is a deliberate trade-off (see the comment), but it should be documented in the eyre module docs.

**F-3 (Low). Test-only invariants in proc-macro code use `unwrap()`/`expect()`**
- `shared/macro_core/types/empty_struct.rs:87`, `cgp_data/derive_has_field.rs:19`, `ident/path_with_type_args.rs:76,98`, `attributes/use_type/grounding.rs:78`. Each one is guarded by an invariant (the length was checked, or the fields are named), so none is a live bug. Converting them to `syn::Error` or `debug_assert` and documenting the invariant would keep a future refactor from turning one into an ICE-style panic inside a user's build.

### Packaging, build, and performance

**F-4 (Medium). Library `build.rs` exists only for tests**
- `crates/main/cgp-fork/build.rs:1-64` reads `src/{anyhow,eyre,std_error}/README.md` and generates `$OUT_DIR/readme_*.rs`. Only `tests/error_backends/readme_*.rs` consume the output. Even so, every downstream crate that depends on `cgp-fork` compiles and runs this build script. That adds build time, cache churn (`rerun-if-changed`), and blocks some sandboxed or hermetic build setups. It also `unwrap()`s env vars (`build.rs:14,15,36`).
- The stated reason for the indirection is stale: `tests/error_backends/readme_anyhow.rs:1-3` says the README block is `ignore` "because in the crate's own doctests `cgp_fork` names `cgp-fork-core`". After the consolidation `cgp_fork` *is* this crate, so the READMEs can be real doctests.

**F-5 (Medium). Published crates ship the whole test suite**
- `cargo package --list` for `cgp-fork` includes 340 `tests/**` files, and `cgp-fork-macro` includes 39 `src/macro_tests/**` files. Neither manifest has an `include`/`exclude`. This inflates download size and docs.rs build time for every user.

**F-6 (Low). Compile-time cost of the CGP pattern**
- Heavy type-level recursion (`Cons`/`Chars` symbol encoding, `Symbol!("hello") = Symbol<5, Chars<'h', …>>`) and large macro expansions are inherent to CGP. There is no benchmark or tracking of compile-time cost, either for the macros or for downstream wiring. An example: 21 test targets × full `syn 3` `full,extra-traits,visit,visit-mut` features.

### Documentation

**F-7 (Medium). Docs left stale by the crate consolidation**
- User-facing on docs.rs: `src/anyhow/README.md:1,3`, `src/eyre/README.md:1,3`, and `src/std_error/README.md:1,3` are titled and worded as separate crates (`cgp-fork-error-anyhow`, …). They are included as module docs at `src/lib.rs:28-38`.
- Internal comments still name non-existent crates in about 19 files: `cgp-fork-core`, `cgp-fork-component`, `cgp-fork-handler`, `cgp-fork-error`, `cgp-fork-macro-core`, `cgp-fork-macro-lib`, `cgp-fork-macro-test-util-lib`. Examples: `crates/macros/cgp-fork-macro/src/lib.rs:752`, `shared/macro_lib/derive_provider.rs:12`, `shared/macro_core/types/attributes/derive_promote.rs:19`, `tests/error_backends_tests.rs:3-4`, `tests/error_definition_tests.rs:5`.
- Broken relative link: `shared/macro_core/types/cgp_provider/args.rs:14` → `../../../cgp-fork-macro-lib/src/cgp_new_provider.rs`.
- Around 592 references to `cgp-knowledge-base-fork/...` paths across `crates/` are plain-text pointers to a different repository. They can't be checked and will rot quietly.

**F-8 (Medium). No documentation example is compiled**
- 86 doc examples (31 in `cgp_fork`, 55 in `cgp_fork_macro`) are all `rust,ignore`. Proc-macro doctests can't easily use the macros from inside `cgp-fork-macro`, but they *can* in `cgp-fork`, which re-exports everything. The README examples and most of the macro docs could be compiled doctests on the facade (`#[doc = include_str!]` or `cgp_fork::prelude` re-exports).
- Example quality: `#[cgp_component]`'s rustdoc (`src/lib.rs:44-95`) shows the input but not the generated provider trait or blanket impls that the text promises.

**F-9 (Low). CHANGELOG 0.9.3 is malformed**
- `CHANGELOG.md:7` and `:27` both have `### Added`. The section at `:31` (`### Fixed`) lists documentation changes ("drop the crate version", "name HasErrorType…") as fixes. Hand edits and the release-plz output were merged. The README links to the CHANGELOG as the release notes.
- Versioning: the fork is 0.9.x while upstream is 0.8.0. That's fine for a fork, but no compatibility table explains which upstream version each fork version corresponds to.

### Code quality

**F-10 (Low). A blanket lint suppression hides dead code**
- `crates/macros/cgp-fork-macro/src/lib.rs:12-26` applies `#[allow(dead_code, unused_imports)]` to the entire `macro_core`, `macro_lib`, `extra_macro_core`, `extra_macro_lib`, and `test_util_lib` trees. With the suppression removed, the genuinely dead items are:
  - `shared/macro_core/types/path/path_head_or_type.rs:10`: `enum PathHeadOrType` is never used.
  - `shared/macro_core/types/path/mod.rs:12`: unused import `path_head_or_type::*`.
  - `shared/macro_core/types/provider_impl.rs:13`: `fn derive_is_provider_for` is never used.
  - `shared/macro_core/types/ident/type_generic_param.rs:153`: `fn is_empty` is never used.
  - The other 13 warnings are syn tokens that are parsed but never read (`arrow`, `colon`, `semi`, …). Those are normal; prefix them with `_` or use a narrow per-struct `allow`.
- The `#[path = "../shared/…"]` layout is a leftover of upstream's multi-crate split. It works, but it confuses rust-analyzer and readers (files sit outside `src/`, and the module tree differs from the directory tree). Moving `shared/*` under `src/` would make it a normal crate.

**F-11 (Low). Leftover lint allowances**
- `crates/main/cgp-fork/src/lib.rs:3`: `#![allow(mixed_script_confusables)]`, although there are no non-ASCII identifiers in the source. It's probably a leftover from upstream's Greek-letter aliases. Remove it.
- `src/extra/error/mod.rs:1-2` repeats `extern crate alloc` under a `cfg` that is already covered by `lib.rs:6`.
- Several `#[allow(clippy::large_enum_variant)]` sit on parser enums (`path_element.rs:12`, `unipath_or_type.rs:10`, `check_components/key.rs:7`, `cgp_provider/provider_impl_args.rs:17`). That's acceptable at compile time but worth a comment, or boxing.

**F-12 (Low). Pedantic clippy debt**
- About 200 pedantic warnings, mostly mechanical. Enabling a curated subset (`needless_pass_by_value`, `ref_option`, `redundant_closure`, `match_wildcard_for_single_variants`, `uninlined_format_args`) would improve the macro code without much churn.

### Tests

**F-13 (Medium). Gaps in the test strategy**
- No test covers `#[async_trait]` with default bodies or with attribute arguments (F-1 slipped through).
- Doctests never run: `cargo nextest` (CI, `tests.yml:61`) skips doctests, and every doctest is `ignore` anyway.
- No coverage measurement. There's no `cargo llvm-cov` job, so it's unknown which macro branches or rejection paths are exercised.
- Compile-fail (UI) tests live in the external `cargo-cgp` repo, so this repo's CI can't catch a regression in error spans or messages. Consider a small `trybuild` suite for the most important rejection diagnostics.
- No `--no-default-features` test run (only clippy). No property-based or fuzz testing of the parsers (`delegate_components!` grammar, `Symbol!`, namespaces) even though they are the most intricate code.

### CI/CD and supply chain

**F-14 (Medium). Tests workflow gaps** (`.github/workflows/tests.yml`)
- `:58`: `curl -LsSf https://get.nexte.st/latest/linux | tar zxf -` is unpinned and has no checksum. This contradicts the workflow's own "minimize supply-chain exposure" comment and the checksummed release-plz install (`release-plz.yml`).
- `:31,40,53,67`: `actions/checkout@v4` is pinned by tag, not by commit SHA.
- No MSRV job, although `rust-version = "1.89"` and the README badge promise 1.89.
- No `cargo doc -D warnings`, `cargo test --doc`, or `cargo package`/`publish --dry-run` job. No feature-matrix job (`cargo hack --feature-powerset`). No macOS or Windows job (the build script has CRLF handling but nothing tests it).
- No `cargo deny`/`cargo audit` job and no Dependabot/Renovate. Today's audit is clean (§3), but nothing keeps it that way.
- No `Swatinem/rust-cache` or equivalent. Avoiding third-party actions is deliberate, so a plain `actions/cache` keyed on `Cargo.lock` would do.
- No `cargo semver-checks`, which matters for a published library that releases automatically on every `feat`/`fix`/`refactor` commit.

**F-15 (Low). Release workflow** (`.github/workflows/release-plz.yml`, `release-plz.toml`)
- It's well designed: pinned, checksummed binary, OIDC trusted publishing, no long-lived crates.io token. Notes:
  - Both jobs run on every push to `main`. `release_always = true` together with `refactor` triggering a patch bump means many small releases (three releases in three days: v0.9.1 to v0.9.3).
  - Without the optional `RELEASE_PLZ_TOKEN`, the release PR doesn't trigger the Tests workflow. In that case a release could be merged without CI. Consider making the PAT mandatory, or adding a required check that `workflow_dispatch` runs.
  - `changelog_update = false` at the workspace level combined with a hand-edited changelog led to F-9.

### Security
- No hardcoded secrets, tokens, or credentials found. CI uses OIDC and `secrets.*` correctly, and the Tests workflow has `permissions: contents: read`.
- No `unsafe` code in the library. The macros only *propagate* `unsafe` from user traits (`derive_promote.rs:89`, `blanket.rs:141`, `blanket_trait.rs:164`) and explicitly reject `unsafe` where it isn't supported (`cgp_auto_error/item.rs:36`, `getter/parse.rs:143`). That looks correct.
- Proc-macro and build-script code executes at compile time in user builds. The build script only reads files inside the crate. Removing it (F-4) shrinks that attack surface further.
- The supply-chain items in F-14 are the only security-relevant findings.

---

## 5. Improvement opportunities (beyond fixes)
- **Compiled examples:** an `examples/` directory (e.g. an app/test context swap, error backends, an async runtime) that CI builds, linked from the README. Today the README has no code example of CGP itself.
- **Better onboarding:** the root README is very short and defers everything to external sites. Add a 30-line "hello CGP" (component → provider → `delegate_components!` → call), a feature table, and a "difference from upstream" section.
- **Upstream sync tooling:** a documented `git remote add upstream` plus a scripted merge/check, and a "synced through" line updated automatically (README currently hard-codes `69d2854`).
- **Diagnostics:** keep improving spans (much is already done). Add `trybuild` snapshots for the top 10 user mistakes, including missing wiring and a wrong provider.
- **Compile-time budget:** a CI job that times `cargo build` of a representative wiring test, to catch macro expansion blow-ups.
- **`#[async_trait]` parity:** optionally support `Send`-bounded futures (`#[async_trait(Send)]`) or document `CanSendRun` as the pattern.
- **Coverage plus fuzzing:** `cargo llvm-cov` reports, and `cargo fuzz` targets for the `delegate_components!`/`Symbol!` parsers (they take arbitrary user tokens).
- **Repo hygiene:** add an `AGENTS.md`/`CONTRIBUTING.md` (upstream has `AGENTS.md`; this fork's `sibling-projects.md` assumes agent workflows), issue and PR templates, and a `SECURITY.md`.

---

## 6. Phased roadmap

Effort: S ≤ 2 h, M ≈ ½–2 days, L > 2 days. Risk is the risk of the change itself.

### Phase 0: Quick wins (≈ 1–2 days total)

| ID | Item | Problem | Proposed fix | Files | Effort | Risk | Verification |
|---|---|---|---|---|---|---|---|
| Q1 | Fix `#[async_trait]` default bodies | F-1 | When `trait_fn.default` is `Some(block)`, replace it with `{ async move #block }`. Emit `syn::Error` for non-empty `attr`, or explicitly support `?Send` as a no-op. Add tests. | `crates/macros/cgp-fork-macro/src/async_impl.rs`, `src/lib.rs:1368`, new test in `crates/main/cgp-fork/tests/async_and_send/` | S | Low | Repro from F-1 compiles and the future yields 7. A rejection test for `#[async_trait(bogus)]`. Snapshot of the expansion. |
| Q2 | Pin and verify nextest, SHA-pin checkout | F-14 | Download a fixed nextest release tarball plus `sha256sum -c` (same as the release-plz step). Pin `actions/checkout` to a commit SHA with a version comment. | `.github/workflows/tests.yml`, `release-plz.yml` | S | Low | CI green. Grep finds no `latest` URLs. |
| Q3 | Add doctest, doc, and MSRV jobs | F-8/F-14 | Jobs: `cargo test --doc --all-features`, `RUSTDOCFLAGS=-Dwarnings cargo doc --all-features --no-deps`, `rustup toolchain install 1.89 && cargo +1.89 check --workspace --all-features --all-targets` (the last already passes locally) | `tests.yml` | S | Low | The new jobs pass on main |
| Q4 | Clean the CHANGELOG and stale titles | F-7, F-9 | Merge the duplicate `### Added`, move doc items out of `Fixed`, retitle the three backend READMEs to "`anyhow` feature", "`eyre` feature", "`std-error` feature", and fix the broken link in `cgp_provider/args.rs:14` | `CHANGELOG.md`, `src/{anyhow,eyre,std_error}/README.md`, `args.rs` | S | Very low | docs.rs preview (`cargo doc --open`) and a link check |
| Q5 | Remove dead code and leftover allows | F-10, F-11 | Delete `PathHeadOrType`, `derive_is_provider_for`, `is_empty` (or `#[cfg(test)]` it), the unused import, and `#![allow(mixed_script_confusables)]` | files listed in F-10/F-11 | S | Low | clippy `-D warnings` is clean, and tests pass |
| Q6 | Package `include`/`exclude` | F-5 | `exclude = ["tests/**", "src/macro_tests/**"]` (or an explicit `include`) | both crate `Cargo.toml` | S | Low (check that the build script and READMEs are still included until P1) | `cargo package --list` shows no tests. `cargo publish --dry-run` verifies. |
| Q7 | `cargo update` | §3 | Take the 12 compatible bumps | `Cargo.lock` | S | Very low | Full test suite |

### Phase 1: Short term (≈ 1–2 weeks)

| ID | Item | Problem | Proposed fix | Files | Effort | Risk | Verification |
|---|---|---|---|---|---|---|---|
| P1 | Remove the library build script | F-4 | Convert the three README `rust,ignore` blocks into real doctests (hide `fn main` boilerplate with `#`), gated by `#[cfg_attr(feature = "...", doc = include_str!(...))]`. Delete `build.rs` and `tests/error_backends/readme_*.rs`. Alternatively, move the generator to a dev-only `xtask`. | `crates/main/cgp-fork/build.rs`, `src/*/README.md`, `tests/error_backends/{mod.rs,readme_*.rs}` | M | Low–Med (the doctests must run with `--all-features`) | `cargo test --doc --all-features` runs the three examples. `cargo package --list` has no `build.rs`. A downstream sample crate builds without a build script. |
| P2 | Honest feature graph | F-2 | `#[cfg(feature = "alloc")] extern crate alloc;`. Then `anyhow = ["alloc", "dep:anyhow"]`, `std-error = ["alloc"]`, `eyre = ["alloc", "dep:eyre"]`, documented as std-only. Gate alloc-only items. Document the eyre `auto-install` trade-off in the module docs. Decide whether pure-`core` is supported; if not, drop the `alloc` feature (a breaking change, so batch it into 0.10). | `src/lib.rs`, `Cargo.toml`, `src/extra/error/**`, any module using `alloc::` | M | Medium (feature changes can break users) | `cargo hack check --feature-powerset --no-dev-deps` on host and on `thumbv7m` (excluding eyre). semver-checks. |
| P3 | Feature-matrix and supply-chain CI | F-14 | Add `cargo hack` powerset, `cargo deny check` (advisories, licenses, bans, sources) with `deny.toml`, `cargo semver-checks` against the last crates.io release, a `cargo publish --dry-run` job, and Dependabot (cargo + github-actions). Add `actions/cache` for `~/.cargo` and `target`. | `.github/workflows/tests.yml`, new `deny.toml`, `.github/dependabot.yml` | M | Low | All jobs green, and a deliberate API break is flagged by semver-checks |
| P4 | Documentation sweep | F-7, F-8 | Replace every stale crate name (≈19 files). Turn macro rustdoc examples into compiled doctests on the facade where feasible. Show the generated code in `#[cgp_component]` docs. Expand the root README with a minimal example and a feature table. | `crates/**`, `README.md`, `crates/main/cgp-fork/README.md` | M | Very low | `rg 'cgp-fork-(core|error|component|handler|macro-core|macro-lib)'` finds nothing. The doctest count is above 0. |
| P5 | Release safety | F-15 | Require CI on release PRs (make `RELEASE_PLZ_TOKEN` mandatory or add a `workflow_dispatch` gate). Consider dropping `refactor` from `release_commits`, or batching releases. | `release-plz.yml`, `release-plz.toml` | S | Low | The release PR shows Tests checks |
| P6 | Proc-macro robustness | F-3 | Replace invariant `unwrap`/`expect` calls with `syn::Error` or with comments plus `debug_assert`. | the files in F-3 | S | Low | Tests plus the rejection tests |

### Phase 2: Long term (≈ 1–2 months, opportunistic)

| ID | Item | Problem | Proposed fix | Files | Effort | Risk | Verification |
|---|---|---|---|---|---|---|---|
| L1 | Normalize the macro crate layout | F-10 | Move `shared/*` under `src/`, drop the `#[path]` modules, and replace the blanket `allow(dead_code)` with targeted allows. Keep `test_util_lib` behind `cfg(any(test, feature="snapshot"))`. | `crates/macros/cgp-fork-macro/**` | L (mechanical but huge diff) | Medium (upstream merges get harder, so coordinate with upstream sync) | Build, tests, and identical snapshot output |
| L2 | In-repo diagnostics tests | F-13 | A small `trybuild` suite for the top user mistakes, complementing `cargo-cgp` | new `crates/main/cgp-fork/tests/ui/` | M | Low (the toolchain pin keeps output stable) | `cargo test` |
| L3 | Coverage and fuzzing | F-13 | A `cargo llvm-cov` CI report, and `cargo fuzz` targets for the `delegate_components!`, `Symbol!`, and namespace parsers (they must never panic) | new `fuzz/`, workflow | M | Low | Coverage badge, and the fuzzers run 10 min without a panic |
| L4 | Compile-time budget | F-6 | A benchmark crate with N components or wiring entries, timed in CI (`cargo build --timings`), with alerts on regressions above X% | new `benches/compile/`, workflow | M | Low | A baseline is recorded |
| L5 | Upstream sync process | — | Document an `upstream` remote, a sync script, and an automatically updated "synced through" commit in the README. Optionally a scheduled job that opens an issue when upstream moves. | `CONTRIBUTING.md`, `scripts/`, workflow | S–M | Low | A dry-run sync reports `eca0c5f` as pending |
| L6 | Examples and onboarding | §5 | An `examples/` directory with 3–4 real-world programs, compiled in CI, plus `CONTRIBUTING.md`, `SECURITY.md`, and issue templates | new files | M | Very low | `cargo build --examples` |
| L7 | Clippy pedantic subset | F-12 | Enable a curated `[workspace.lints]` list and fix the findings | `Cargo.toml`, macro sources | M | Low | clippy clean |

---

## 7. Suggested execution order

1. **Q1** (the `#[async_trait]` bug) first. It's the only user-visible defect, and the fix is small and test-backed. Ship it as `fix(async-trait): …` so release-plz cuts a patch.
2. **Q2 + Q3**: harden CI before more changes land, so later PRs are verified for MSRV, docs, and doctests.
3. **Q4, Q5, Q6, Q7**: cheap hygiene, batched into one or two PRs.
4. **P3**: feature matrix, `cargo deny`, semver-checks, and Dependabot. Required before touching features.
5. **P1**: remove `build.rs` by converting the README examples to doctests. Q3's doctest job now verifies it.
6. **P2**: fix the feature graph, guarded by P3's feature matrix and semver-checks. If it's breaking, hold it for a 0.10 release together with any other breaking changes.
7. **P4, P5, P6**: documentation sweep, release safety, proc-macro robustness.
8. **Phase 2** as capacity allows. Do **L5** (the upstream sync process) before **L1** (the big layout move), so the restructure doesn't make future upstream merges painful. L2 and L3 can run in parallel with anything else.

---

## Appendix: reproduction commands

```sh
git clone https://github.com/Just-Replicant/cgp-fork && cd cgp-fork
cargo build --workspace --all-features
cargo test --workspace --all-features --no-fail-fast          # 706 pass, 86 doctests ignored
cargo clippy --all-features --all-targets -- -D warnings
cargo clippy --no-default-features --all-targets -- -D warnings
cargo +nightly fmt --all -- --check
RUSTDOCFLAGS="-D warnings --cfg docsrs" cargo doc --all-features --no-deps
rustup target add thumbv7m-none-eabi
cargo check -p cgp-fork --target thumbv7m-none-eabi
cargo check -p cgp-fork --features eyre --target thumbv7m-none-eabi   # fails: needs std
cargo +1.89 check --workspace --all-features --all-targets            # needs rust-toolchain.toml removed or overridden
cargo audit
cargo package -p cgp-fork --list | grep -c '^tests/'                  # 340
```

`#[async_trait]` repro (in a scratch crate depending on `cgp-fork`):
```rust
use cgp_fork::prelude::*;
#[async_trait]
pub trait HasDefault { async fn value(&self) -> u32 { 7 } }
// error[E0277]: `{integer}` is not a future
```
