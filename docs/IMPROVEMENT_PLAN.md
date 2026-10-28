# cgp-fork: Improvement plan

Fixes and improvements found in the analysis of `main` @ `0567fad2` (v0.10.0), in the order to do
them. The earlier analysis of v0.9.3 is kept in [`docs/history/`](history/2026-10-27-improvement-plan.md).

## Status

| Item | Status | Commit |
|---|---|---|
| F-1 | Done | `90320d35` |
| F-2 | Done (repository settings are documented for the maintainer) | `3e5077df` |
| F-3 | Done | `939ded18` |
| F-4 | Done | `26d316be` |
| F-5 | Done in this repository; 8 drafted pages wait to be copied into the knowledge base | `d53299a5` |
| I-1 | Done; the `.mailmap` line is a placeholder for the maintainer to fill in | this commit |
| I-2 … I-9 | Open | |

## Checklist

1. [ ] F-1 Make the published crates testable and complete (macro tests, LICENSE, package CI check)
2. [ ] F-2 Gate publishing on a green CI run
3. [ ] F-3 Bump the toolchain to 1.99.1 and add a canary job
4. [ ] F-4 Stop README doctests from being dropped silently
5. [ ] F-5 Fix the 11 dangling knowledge-base references
6. [ ] I-1 Repository hygiene (`.mailmap`, comments, stale plan doc)
7. [ ] I-2 Track upstream reviews and automate the sync check
8. [ ] I-3 Enforce coverage and test the weak spots
9. [ ] I-4 Compile the 11 ignored facade doc examples
10. [ ] I-5 Bump or drop `itertools`
11. [ ] I-6 Drop syn `extra-traits` from normal builds
12. [ ] I-7 Track compile time over time
13. [ ] I-8 Give macro-crate docs compiled examples
14. [ ] I-9 Coverage-guided fuzzing with `cargo-fuzz`

Do F-1 to F-2 before the next release. Ship F-1 as **0.10.1**, the first release that waits for CI.

---

## Fix

### F-1 Make the published crates testable and complete
**Priority:** High · **Severity:** Medium
- **Problem:**
  - `cargo test` on the published `cgp-fork-macro` 0.10.0 fails with `couldn't find file src/macro_tests/…`. The `#[cfg(test)] #[path = "macro_tests/…"]` modules point into a directory the package leaves out.
  - Neither crate ships a `LICENSE` file, which Apache-2.0 §4(a) requires.
  - The CI `package` job doesn't catch either problem.
- **Fix:**
  1. Remove `src/macro_tests/` from `exclude`, so the tests ship with the crate.
  2. Add a `LICENSE` copy (or symlink) in each crate directory.
  3. Extend the `package` job: after `cargo package --workspace`, unpack both `.crate` files, run `cargo test --all-features` in each, and assert that `LICENSE` is listed.
- **Files:**
  - `crates/macros/cgp-fork-macro/Cargo.toml:14`
  - `crates/macros/cgp-fork-macro/src/lib.rs:19-33`
  - `crates/main/cgp-fork/LICENSE` and `crates/macros/cgp-fork-macro/LICENSE` (new)
  - `.github/workflows/tests.yml:230-246`
- **Effort:** ~1 h
- **Verify:**
  - Before the fix, the extended `package` job fails on current `main`; after it, the job passes.
  - `cargo package --list -p cgp-fork-macro | grep -E 'LICENSE|macro_tests'` shows both.

### F-2 Gate publishing on a green CI run
**Priority:** High · **Severity:** Medium
- **Problem:** release-plz publishes on every push to `main` in parallel with the Tests workflow (`release_always = true`). Its own check runs only `cargo test`, so a commit that breaks MSRV, `no_std`, packaging, macOS or Windows can still reach crates.io. 0.10.0 was published 6 minutes before Tests finished, and PR #14 was merged while its own CI run was still in progress (it later ended in failure).
- **Fix:**
  1. Start `release-plz-release` from `on: workflow_run: workflows: [Tests], types: [completed], branches: [main]`, with `if: github.event.workflow_run.conclusion == 'success'`, and check out `github.event.workflow_run.head_sha`.
  2. Keep the `push` trigger only for `release-plz-pr`.
  3. Add `concurrency: { group: release-plz-release, cancel-in-progress: false }`.
  4. Maintainer, in repository settings: protect `main` with the Tests jobs as required checks, and set the `RELEASE_PLZ_TOKEN` secret. Then turn the "PAT missing" warning into an error.
- **Files:**
  - `.github/workflows/release-plz.yml:31-34`, `:68-73`, and the warning step
  - `release-plz.toml:12`
  - repository settings
- **Effort:** 2–3 h, plus maintainer settings
- **Verify:**
  - `actionlint` passes.
  - On a test fork, a push that fails clippy publishes nothing.
  - A green push publishes only after Tests completes.
  - The next release PR shows Tests checks.

### F-3 Bump the toolchain to 1.99.1 and add a canary job
**Priority:** High · **Severity:** Medium
- **Problem:** Stable is 1.99.1, but the repo pins 1.98.1. Under 1.99.1, the UI test `missing_dependency` fails because rustc prints shorter type paths in the `help:` lines. Every toolchain bump will turn `main` red, and nothing warns ahead of time.
- **Fix:**
  1. Set `channel = "1.99.1"`.
  2. Regenerate the snapshot with `TRYBUILD=overwrite cargo test -p cgp-fork --all-features --test ui_tests`, in the same commit.
  3. Add a `canary` job (stable and beta, `continue-on-error: true`, on PRs and the weekly cron) that runs clippy and the tests.
  4. Note in CONTRIBUTING that the `.stderr` snapshots depend on the toolchain.
- **Files:**
  - `rust-toolchain.toml:2`
  - `crates/main/cgp-fork/tests/ui/missing_dependency.stderr`
  - `.github/workflows/tests.yml`
  - `CONTRIBUTING.md`
- **Effort:** 1–2 h
- **Verify:**
  - `cargo test --workspace --all-features` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` pass on 1.99.1.
  - The MSRV job is unchanged.

### F-4 Stop README doctests from being dropped silently
**Priority:** Medium · **Severity:** Low
- **Problem:** The root-README doctests run only when `.cargo/config.toml` sets `--cfg cgp_fork_repository_doctests`. Any `RUSTDOCFLAGS` in the environment replaces that flag, and the README examples are skipped without a warning (23 doctests become 21).
- **Fix:**
  1. Move the README doctest into an unpublished workspace member, e.g. `crates/tests/readme-doctests` with `publish = false` and `#[doc = include_str!("../../../README.md")] pub struct Readme;`.
  2. Delete `.cargo/config.toml`, the cfg gate, and the `check-cfg` entry.
- **Files:**
  - `crates/main/cgp-fork/src/lib.rs:40-48`
  - `.cargo/config.toml`
  - `Cargo.toml:26-27`
  - new crate under `crates/tests/`
- **Effort:** 1–2 h
- **Verify:**
  - `RUSTDOCFLAGS="-D warnings" cargo test --workspace --doc` still runs the README examples.
  - The `cargo package --list` output is unchanged.

### F-5 Fix the 11 dangling knowledge-base references
**Priority:** Medium · **Severity:** Low–Medium
- **Problem:** 11 of 113 `cgp-knowledge-base-fork/…` references point at pages that don't exist. They cover the `struct!`/`enum!` shape macros, `cgp_auto_error`/`cgp_auto_impl`/`cgp_auto_log`, and the per-backend `testing.md` pages (the knowledge base names those directories `cgp-error-*`).
- **Fix:**
  1. Repoint references to pages that already exist (e.g. `projects/error/cgp-error-*/README.md`, `cgp/implementation/entrypoints/derive_cgp_data.md`), or add the missing pages to the knowledge base.
  2. Add a CI job that shallow-clones the public knowledge base and runs `bash scripts/check-kb-refs.sh "$KB"`. Make it non-blocking at first.
- **Files** (all paths relative to `crates/`):
  - `macros/cgp-fork-macro/src/macro_tests/shape_macro_parsing/{enum_variants,struct_form}.rs:5-6`
  - `macros/cgp-fork-macro/src/macro_tests/parser_rejections/{enum_macro.rs:8,struct_macro.rs:7,cgp_auto_error.rs:4,cgp_auto_impl.rs:4,cgp_auto_log.rs:4}`
  - `main/cgp-fork/tests/shape_macros_tests.rs:9-10`, `tests/shape_macros/*`, `tests/error_definition*`, `tests/blanket_traits/auto_impl_*`, `tests/logging/log_*`, `tests/error_backends/*`
  - `.github/workflows/tests.yml`
- **Effort:** 2–4 h
- **Verify:** `bash scripts/check-kb-refs.sh <kb-checkout>` reports `0 dangling`.

---

## Improve

### I-1 Repository hygiene
**Priority:** Medium · **Severity:** Low
- **Problem:**
  - 18 commits carry the placeholder author `cgp-fork maintainer <maintainer@localhost>`.
  - The clippy comment in the root `Cargo.toml` sits above the wrong lints table.
  - The usage lines in the script headers still call the scripts directly, but they're no longer executable.
  - `docs/IMPROVEMENT_PLAN.md` is out of date: it marks compile-time tracking "Done" and mentions analysis scratch paths.
- **Fix:**
  1. Add a `.mailmap` line mapping that author to `Just-Replicant <335413437+Just-Replicant@users.noreply.github.com>`.
  2. Move the comment above `[workspace.lints.clippy]`.
  3. Change the header usage lines to `bash scripts/…`.
  4. Move the old plan to `docs/history/`, remove the scratch paths, and mark F-6 as partial.
- **Files:**
  - `.mailmap` (new)
  - `Cargo.toml:25`
  - `scripts/check-kb-refs.sh:5`, `scripts/upstream-sync.sh:4-5`
  - `docs/IMPROVEMENT_PLAN.md:17,50-51`
- **Effort:** 30 min
- **Verify:**
  - `git shortlog -se` shows a single identity.
  - `rg '/workspace|/tmp/' docs` finds nothing.

### I-2 Track upstream reviews and automate the sync check
**Priority:** Low
- **Problem:** Upstream commit `eca0c5f` (#280, docs-only, not relevant to the fork) is unmerged and not recorded as reviewed, so `upstream-sync.sh` will keep reporting it. Sync checks are also manual.
- **Fix:**
  1. Record a "reviewed through" commit (e.g. an `UPSTREAM_REVIEWED` file) that the script honours.
  2. Add a weekly workflow that runs `bash scripts/upstream-sync.sh` and opens or updates an issue when upstream moves.
- **Files:**
  - `UPSTREAM_COMMIT`
  - `scripts/upstream-sync.sh`
  - `CONTRIBUTING.md`
  - new workflow
- **Effort:** ½ day
- **Verify:** The script lists no pending commits; the scheduled job runs green.

### I-3 Enforce coverage and test the weak spots
**Priority:** Low
- **Problem:** Line coverage is 91.9%, but nothing enforces a threshold. These files fall below 80%:

  | File | Line coverage |
  |---|---|
  | `extra/handler/providers/promote_ref.rs` | 55.8% |
  | `attributes/derive_promote.rs` | 57.9% |
  | `cgp_auto_error/item.rs` | 65.2% |
  | `cgp_auto_impl/blanket.rs` | 68.9% |
  | `delegate_and_check_components/to_keys_with_check_params.rs` | 72.9% |
  | `cgp_component/item.rs` | 73.8% |
  | `cgp_auto_log/item.rs` | 75.2% |
  | `functions/getter/parse.rs` | 78.4% |

- **Fix:**
  1. Add `--fail-under-lines 90` to the coverage job.
  2. Add behaviour and rejection tests for the files above.
- **Files:**
  - `.github/workflows/tests.yml:262-264`
  - new tests under `crates/main/cgp-fork/tests/` and `crates/macros/cgp-fork-macro/src/macro_tests/`
- **Effort:** 1–2 days
- **Verify:** `cargo llvm-cov --workspace --all-features --summary-only` shows ≥ 90% overall and ≥ 80% for each listed file.

### I-4 Compile the 11 ignored facade doc examples
**Priority:** Low
- **Problem:** 11 `rust,ignore` examples in the facade aren't compiled, so they can drift from the code.
- **Fix:** Turn each into a compiled doctest with hidden setup lines (`# …`), or mark it `text` where it shows generated code.
- **Files** (all under `crates/main/cgp-fork/src/core/`):
  - `types/impls/use_type.rs:43`
  - `component/traits/delegate_component.rs:51`
  - `component/providers/use_delegate.rs:33,44`
  - `component/traits/is_provider.rs:55,89,141`
  - `field/types/index.rs:29`
  - `component/providers/use_context.rs:54`
  - `field/impls/use_field.rs:61`
  - `field/traits/has_field.rs:34`
- **Effort:** ½ day
- **Verify:** `cargo test -p cgp-fork --all-features --doc` reports 0 ignored.

### I-5 Bump or drop `itertools`
**Priority:** Low
- **Problem:** `itertools` 0.14 is a build-time dependency for every user, and 0.15 is out.
- **Fix:** Replace its few uses with `std` iterator methods, or bump to 0.15.
- **Files:** `crates/macros/cgp-fork-macro/Cargo.toml:33` and its call sites.
- **Effort:** 2 h
- **Verify:** Tests and snapshots are unchanged; `cargo tree -p cgp-fork` no longer lists `itertools` (if removed).

### I-6 Drop syn `extra-traits` from normal builds
**Priority:** Low
- **Problem:** syn's `extra-traits` feature is needed only because the AST types derive `Debug`, and it adds compile time for every downstream user.
- **Fix:** Gate those `Debug` derives on `cfg(any(test, feature = "snapshot"))`, then drop `extra-traits` from the normal dependency. Measure first.
- **Files:**
  - `crates/macros/cgp-fork-macro/Cargo.toml:30`
  - the AST type derives under `src/macro_core/types/`
- **Effort:** 1 day
- **Verify:**
  - All tests pass.
  - `cargo build --timings` on a downstream crate shows a shorter syn build.

### I-7 Track compile time over time
**Priority:** Low
- **Problem:** The compile-time job only checks a fixed 30 s budget and uploads a log, so gradual regressions go unnoticed.
- **Fix:** Store each `main` result as a series (e.g. a `gh-pages` JSON file, or a benchmark action pinned by SHA). Fail when a run exceeds the rolling median by more than 50%.
- **Files:**
  - `.github/workflows/tests.yml:287-302`
  - `scripts/compile-time.sh`
- **Effort:** 1 day
- **Verify:** The trend series updates on each push; a deliberate slowdown fails the job.

### I-8 Give macro-crate docs compiled examples
**Priority:** Low
- **Problem:** 56 examples in the macro crate are `rust,ignore`. A proc-macro crate can't dev-depend on its own facade, because that creates a dependency cycle.
- **Fix:** Replace the ignored blocks with prose that links to the facade's compiled examples, or generate them from the facade doctests.
- **Files:** `crates/macros/cgp-fork-macro/src/lib.rs`
- **Effort:** 2–3 days
- **Verify:** The macro-crate doctests report 0 ignored, or every macro doc links to a compiled example.

### I-9 Coverage-guided fuzzing with `cargo-fuzz`
**Priority:** Low
- **Problem:** The in-tree fuzzer generates random tokens without coverage guidance. libFuzzer can't link a proc-macro crate.
- **Fix:** Split the parsers into an internal non-proc-macro library crate, then add `cargo-fuzz` targets for each entrypoint. Keep the in-tree fuzzer as a smoke test.
- **Files:** a new internal crate under `crates/macros/`, plus `fuzz/`.
- **Effort:** 1–2 weeks
- **Verify:** `cargo fuzz run <target> -- -max_total_time=3600` runs without crashes for each target.
