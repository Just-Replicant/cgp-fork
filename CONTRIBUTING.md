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
`cargo semver-checks`, `cargo package --workspace`, coverage, a deep fuzzing pass, and the
compile-time budget.

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
- Merging that PR publishes both crates with crates.io trusted publishing and tags `v<version>`.
- Breaking changes need a `!` and a `BREAKING CHANGE:` footer, and a minor bump while the version
  is `0.x`.

Edit `CHANGELOG.md` by hand only under `## Unreleased`, and only for context a commit subject
cannot carry. release-plz moves those notes into the release section; do not add a second
`### Added` (or other) heading to a released section.

## Keeping up with upstream

The fork tracks [contextgeneric/cgp](https://github.com/contextgeneric/cgp). `UPSTREAM_COMMIT`
records the last upstream commit merged here, and the README repeats it.

```sh
bash scripts/upstream-sync.sh          # fetch upstream and list the commits not merged yet
bash scripts/upstream-sync.sh --merge  # merge them into a new upstream-sync-<sha> branch
```

Upstream is split into many crates, while the fork has two, so a merge usually needs its paths
mapped by hand: upstream `crates/*/cgp-*/src` corresponds to a module of `cgp-fork`, and upstream's
macro crates to `crates/macros/cgp-fork-macro/src/{macro_core,macro_lib,…}`. After merging, update
`UPSTREAM_COMMIT` and the README line, then run the full check list above.

## Other scripts

- `bash scripts/check-kb-refs.sh [kb-checkout]` checks that every `cgp-knowledge-base-fork/<path>`
  reference in comments and docs names a file that exists in the knowledge base.
- `bash scripts/compile-time.sh` times a generated crate with `N` (default 200) components against the
  budget in `scripts/compile-budget.txt`. Invoke it with `bash` so a checkout that dropped the
  executable bit still runs.
