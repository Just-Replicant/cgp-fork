# Contributing to cgp-fork

## Layout

| Path | What |
|---|---|
| `crates/main/cgp-fork` | The facade library users depend on (`no_std`). `src/core`, `src/extra`, and the error backends `src/{anyhow,eyre,std_error}` behind features. |
| `crates/main/cgp-fork/tests` | The behaviour suite, one integration target per CGP concept, plus `tests/ui` (trybuild diagnostics). |
| `crates/main/cgp-fork/examples` | Runnable programs, built in CI. |
| `crates/macros/cgp-fork-macro` | Every procedural macro. `src/macro_core` (AST types, parsers, codegen), `src/macro_lib` and `src/extra_macro_lib` (entrypoints), `src/test_util_lib` (the `snapshot_*` macros, feature `snapshot`). |
| `crates/macros/cgp-fork-macro/src/macro_tests` | Parser unit tests, rejection cases, and the panic fuzzer. |
| `crates/tests/README.md` | The map of the test suite and where a new test belongs. |
| `scripts/` | Maintenance scripts (below). |

Deeper documentation (construct reference, macro internals, error catalog) lives in the
[knowledge base](https://github.com/Just-Replicant/cgp-knowledge-base-fork); see
[sibling-projects.md](sibling-projects.md) for the related repositories.

## Toolchain

`rust-toolchain.toml` pins the toolchain CI uses, and `rustup` installs it on first use. The
supported minimum is `rust-version` in `Cargo.toml` (1.89), checked in CI. Formatting uses nightly
rustfmt because `.rustfmt.toml` groups and merges imports.

The `tests/ui/*.stderr` snapshots are rustc's exact output, so they belong to the pinned
toolchain: a new rustc often prints a type path or a note differently. CI's non-blocking
`Toolchain canary` job runs clippy and the tests on the newest stable and beta to show that drift
early. To move to a new toolchain, change `channel` in `rust-toolchain.toml` and regenerate the
snapshots in the same commit:

```sh
TRYBUILD=overwrite cargo test -p cgp-fork --all-features --test ui_tests
```

## Before you push

```sh
cargo +nightly fmt --all
cargo clippy --workspace --all-features --all-targets -- -D warnings
cargo clippy --workspace --no-default-features --all-targets -- -D warnings
cargo test --workspace --all-features            # unit, integration, ui, doctests
cargo test --workspace --no-default-features
RUSTDOCFLAGS="-D warnings --cfg docsrs" cargo doc --workspace --all-features --no-deps
cargo check -p cgp-fork --target thumbv7m-none-eabi --no-default-features
```

CI also runs the MSRV check (`cargo +1.89 check --workspace --all-features --all-targets`),
`cargo hack check --feature-powerset`, `cargo deny check`, `cargo audit`,
`cargo semver-checks`, `cargo package --workspace --exclude cgp-fork-doctests`, coverage, a deep fuzzing pass, and the
compile-time budget.

Coverage has a floor: the job fails below 90% line coverage
(`cargo llvm-cov report --summary-only --fail-under-lines 90`). Run it locally with
`cargo llvm-cov --workspace --all-features --summary-only`. Macro code is measured only
through the in-crate `macro_tests` (expansion inside rustc is not instrumented), so a new
macro rule needs a `macro_tests` case, usually a row in a `parser_rejections` table test.

### Snapshots

Many tests pin a macro's exact expansion as an inline `insta` snapshot. After an intended change:

```sh
cargo insta test -p cgp-fork --all-features --review
```

### Diagnostics (`tests/ui`)

Each case must fail to compile with exactly its `.stderr`. Regenerate after an intended change, and
review the diff:

```sh
TRYBUILD=overwrite cargo test -p cgp-fork --all-features --test ui_tests
```

### Fuzzing

`macro_tests/robustness_tests.rs` feeds every macro entrypoint random and mutated input and fails
if one panics. Run a deeper pass, or reproduce a CI failure with its printed seed:

```sh
CGP_FUZZ_ITERS=20000 CGP_FUZZ_SEED=<seed> cargo test --release -p cgp-fork-macro robustness
```

## Commits, changelog, and releases

Commit subjects follow [Conventional Commits](https://www.conventionalcommits.org/). release-plz
reads them on every push to `main`:

- `feat`, `fix`, and `perf` commits (or any type with `!`) open or update a release PR that bumps
  the shared workspace version and writes `CHANGELOG.md`.
- Merging that PR publishes both crates with crates.io trusted publishing and tags `v<version>`,
  but only after the Tests workflow has passed on that merge commit: the release job runs on
  Tests' `workflow_run` completion and checks out the exact commit Tests passed. A failed or
  cancelled Tests run publishes nothing; the next green run on `main` does.
- Breaking changes need a `!` and a `BREAKING CHANGE:` footer, and a minor bump while the version
  is `0.x`.

Edit `CHANGELOG.md` by hand only under `## Unreleased`, and only for context a commit subject
cannot carry. release-plz moves those notes into the release section; do not add a second
`### Added` (or other) heading to a released section.

## Maintainer setup

These are repository settings, so no file in this repository can enforce them. Configure them once:

1. **Branch protection for `main`** (Settings → Branches, or a ruleset):
   - Require a pull request before merging.
   - Require status checks to pass, and require branches to be up to date before merging.
     Mark these Tests jobs as required: `Rustfmt (nightly)`, `Clippy`,
     `Test (nextest, --all-features)`, `Test (nextest, --no-default-features)`,
     `Test (macos-latest)`, `Test (windows-latest)`, `Rustdoc (-D warnings)`, `MSRV (1.89)`,
     `no_std target`, `Feature powerset (cargo-hack)`,
     `cargo-deny (advisories, licenses, bans, sources)`, `cargo-audit (RustSec)`,
     `Semver (cargo-semver-checks)`, `Package (publish dry run)`, `Coverage (cargo-llvm-cov)`,
     `Macro fuzzing (deep)`, and `Compile-time budget`. Leave `Toolchain canary (stable)`,
     `Toolchain canary (beta)`, `Knowledge-base references`, and
     `Coverage-guided fuzzing (cargo-fuzz)` optional (the last runs only on the schedule).
   - Block force pushes and deletion of `main`.
2. **Release token:** create a fine-grained `RELEASE_PLZ_TOKEN` (Contents and Pull requests:
   read/write) as an Actions secret, so release PRs run Tests. Then set the Actions variable
   `REQUIRE_RELEASE_PLZ_TOKEN` to `true`; the release-pr job then fails instead of warning when
   the token is missing.
3. **Actions permissions:** Workflow permissions "read and write", and allow GitHub Actions to
   create pull requests (release-plz). The `Upstream sync check` workflow needs `issues: write`,
   which it requests itself.
4. **crates.io trusted publishers** for both crates: repository `Just-Replicant/cgp-fork`, workflow
   `release-plz.yml`.

## Keeping up with upstream

The fork tracks [contextgeneric/cgp](https://github.com/contextgeneric/cgp). `UPSTREAM_COMMIT`
records the last upstream commit merged here, and the README repeats it.

```sh
bash scripts/upstream-sync.sh          # fetch upstream and list the commits not merged yet
bash scripts/upstream-sync.sh --merge  # merge them into a new upstream-sync-<sha> branch
bash scripts/upstream-sync.sh --check  # exit 1 when a commit is neither merged nor reviewed
```

An upstream commit that does not apply to the fork (for example docs about upstream's own release)
goes into `UPSTREAM_REVIEWED` with a one-line reason, so it is reported as reviewed rather than
pending. The weekly `Upstream sync check` workflow runs `--check` and opens or updates one issue
when upstream has commits that are neither merged nor reviewed.

Upstream is split into many crates, while the fork has two, so a merge usually needs its paths
mapped by hand: upstream `crates/*/cgp-*/src` corresponds to a module of `cgp-fork`, and upstream's
macro crates to `crates/macros/cgp-fork-macro/src/{macro_core,macro_lib,…}`. After merging, update
`UPSTREAM_COMMIT` and the README line, then run the full check list above.

## Other scripts

- `bash scripts/check-kb-refs.sh [kb-checkout]` checks that every `cgp-knowledge-base-fork/<path>`
  reference in comments and docs names a file that exists in the knowledge base. A page that is
  drafted in `docs/kb-pages/` but not yet in the knowledge base is reported as pending
  (`KB_REFS_STRICT=1` fails on those too). CI runs it, non-blocking, as `Knowledge-base references`.
- `bash scripts/compile-time.sh` times a generated crate with `N` (default 200) components against the
  budget in `scripts/compile-budget.txt`. Invoke it with `bash` so a checkout that dropped the
  executable bit still runs.
- `fuzz/` is a cargo-fuzz crate for coverage-guided fuzzing of every macro entrypoint
  (`cargo +nightly fuzz run entrypoints`). It includes the macro modules by path, so a new
  module under `crates/macros/cgp-fork-macro/src/` needs no change there, but a new top-level
  module must also be added to `fuzz/src/lib.rs`. See `fuzz/README.md`.
- `python3 scripts/compile-time-trend.py --history FILE --value SECS [--record --sha SHA]` compares
  a measurement with the median of the last 10 recorded runs and fails when it is more than 1.5×
  that median and more than 2 s above it. In CI the `Compile-time budget` job keeps the series in
  the Actions cache (`compile-time-history-<os>-*`): pushes to `main` append to it, pull requests
  are only compared with it. Until three runs are recorded the trend check passes and only the
  fixed budget applies. To reset the series (for example after a runner-image change makes every
  build slower), delete those cache entries under *Actions → Caches*.
