# After phase 1b (final): 2026-10-31 (branch `phase-1b-2`)

Measured on the box on 2026-10-31 at `05de6023` (every code change of phases 1b and 2, the
shared family IR, the diagnostics follow-up and the fuzz fixes). This file and the fuzz notes are
the only changes after that commit. Toolchain `rustc 1.99.1 (58cf8f9c3 2026-10-13)`; fuzzing on
`rustc 1.101.0-nightly (92b5b8c87 2026-10-29)`.

## Checks

| Check | Result |
|---|---|
| Tests (`cargo test --workspace --all-features`) | 909 passed, 0 failed, 56 ignored (the macro-crate doc examples, compiled in `cgp-fork-doctests`) |
| Tests without default features | pass |
| `cargo +nightly fmt --check` (workspace and `fuzz/`) | clean |
| Clippy `-D warnings`, all targets, all features / no default features / `fuzz/` | clean |
| `cargo doc` with `RUSTDOCFLAGS="-D warnings --cfg docsrs"` | clean |
| MSRV 1.89 `cargo check --locked`, all features and no default features | pass |
| `thumbv7m-none-eabi` no_std check, default and no default features | pass |

## Coverage (`cargo llvm-cov` 0.9.1)

`cargo llvm-cov --workspace --all-features`, then `cargo llvm-cov report --summary-only`:

| Measure | Covered | Total | % |
|---|---|---|---|
| Lines | 11837 | 12459 | **95.01%** |
| Regions | 17598 | 18882 | 93.20% |
| Functions | 1423 | 1505 | 94.55% |

CI fails under 90% lines. An earlier coverage pass on this branch removed the dead
`ItemCgpData::to_items` and added a test for the multi-method `WithProvider` rejection (`1e8753c0`).

## Supply chain

| Tool | Result |
|---|---|
| `cargo audit` 0.22.2 (1395 advisories, database fetched 2026-10-31) | no vulnerabilities or warnings in `Cargo.lock` (64 crates) or `fuzz/Cargo.lock` (16 crates) |
| `cargo deny check` 0.20.2 | `advisories ok, bans ok, licenses ok, sources ok`; `fuzz/` with its own policy (`fuzz/deny.toml`, `f3393bcf`): the same |

## Semver (`cargo semver-checks` 0.51.0)

| Crate | Baseline | Result |
|---|---|---|
| `cgp-fork` | 0.10.0 from crates.io (the latest release; local version 0.10.1) | 202 checks pass, 58 skipped: no semver update required |
| `cgp-fork-macro-impl` | `main` (not published) | 202 checks pass, 58 skipped: no semver update required |
| `cgp-fork-macro` | | proc-macro crate, outside what `cargo semver-checks` checks; its macro names and the expansion snapshots are unchanged |

## Fuzzing (`cargo fuzz` 0.13.2)

Final pass: every target built with `cargo +nightly fuzz build -O`, run from its existing corpus
for `-max_total_time=180` with `-rss_limit_mb=4096`, five targets at a time on 8 cores while the
other checks ran. No crash, leak or out-of-memory in any target. 21,041,334 inputs in total.

| Target | Time | Inputs run |
|---|---|---|
| `async_trait` | 181 s | 679209 |
| `blanket_trait` | 181 s | 465884 |
| `cgp_auto_dispatch` | 181 s | 1029732 |
| `cgp_auto_error` | 181 s | 459780 |
| `cgp_auto_getter` | 181 s | 408413 |
| `cgp_auto_impl` | 182 s | 591076 |
| `cgp_auto_log` | 181 s | 1010949 |
| `cgp_component` | 182 s | 652498 |
| `cgp_computer` | 182 s | 897107 |
| `cgp_fn` | 182 s | 759411 |
| `cgp_for_each` | 182 s | 1269133 |
| `cgp_getter` | 181 s | 192916 |
| `cgp_impl` | 181 s | 385018 |
| `cgp_namespace` | 181 s | 391195 |
| `cgp_new_provider` | 181 s | 500632 |
| `cgp_preset` | 182 s | 168654 |
| `cgp_producer` | 181 s | 1114832 |
| `cgp_provider` | 181 s | 1046791 |
| `cgp_type` | 181 s | 748929 |
| `check_components` | 182 s | 858121 |
| `delegate_and_check_components` | 182 s | 203647 |
| `delegate_components` | 182 s | 451736 |
| `derive_build_field` | 182 s | 152664 |
| `derive_cgp_data` | 181 s | 52403 |
| `derive_cgp_record` | 181 s | 102387 |
| `derive_cgp_variant` | 181 s | 57319 |
| `derive_extract_field` | 181 s | 53058 |
| `derive_from_variant` | 182 s | 135973 |
| `derive_has_field` | 182 s | 155103 |
| `derive_has_fields` | 182 s | 164785 |
| `derive_provider` | 181 s | 1947382 |
| `entrypoints` | 182 s | 200841 |
| `product` | 181 s | 730345 |
| `type_enum` | 181 s | 241354 |
| `type_path` | 182 s | 636308 |
| `type_product` | 182 s | 720186 |
| `type_struct` | 181 s | 387136 |
| `type_sum` | 181 s | 848287 |
| `type_symbol` | 181 s | 170140 |

### Found in the final pass (known, not fixed)

- `cgp_new_provider`: one slow input (1.9 kB of `&&..&&..=` range and reference runs inside a
  method body), saved as `fuzz/known/cgp_new_provider-slow-563b4e2c`. It takes about 1.3 s in a
  release build and 28 s under the fuzzer's sanitizers. The time is spent in `syn`'s own
  `ToTokens` for the expression (printing the parsed `ItemImpl` back takes 1.16 s; parsing it
  takes 0.4 ms; the depth check 0.06 ms), so every `syn`-based macro that re-emits such a body
  pays it. No crash; not fixed here. Candidates: report it upstream to `syn`, or extend
  `ir::check_nesting_depth` to bound runs of `..` between prefix operators.

### Fixed earlier on this branch (all found with `cargo fuzz`; pinned in `macro_tests`: `ir_tests`, `raw_ident_tests`, `symbol_tests`)

| Commit | Target(s) | Finding | Fix |
|---|---|---|---|
| `4f62c825` | `cgp_auto_getter`, `cgp_auto_impl`, `blanket_trait`, `cgp_computer`, `cgp_for_each`, `cgp_producer`, `cgp_type` | stack overflow (rustc `SIGSEGV`) on about 400 levels of nested generics / brackets | `ir::run` rejects input nested deeper than 128 levels before parsing |
| `68c045f4` | `cgp_auto_getter`, `cgp_auto_impl`, `cgp_computer` | stack overflow on long reference chains (`&&&&…T`) | runs of `&`, `*`, `!`, `-` count towards the limit |
| `07e782cb` | `cgp_impl` | panic: `impl r#Greeter` built `Ident::new("r#GreeterComponent")` | derived component name uses the unrawed spelling |
| `d0a9bc7c` | `cgp_fn`, `cgp_computer` | stack overflow on `A<&&&&A<&&&&…>>` (each run reset by `<`) | prefix runs carried into the generics they open |
| `994ca2e0` | `cgp_namespace` | panic: raw namespace name built `Ident::new("__r#GreetComponents")` | unrawed spelling, with and without a parent namespace |
| `6c859b94` | `cgp_auto_getter`, `cgp_type` | stack overflow on closure chains (`\|\| a \|\| \|\| b …`) | every closure bar adds a level until `;`, `,` or `=>` |
| `2bc70c27` | `cgp_namespace` | stack overflow on a name of a few hundred characters (`Symbol::to_type` re-parsed the type-level string, nested once per character) | the syntax tree is built in a loop; snapshots unchanged |
| `20d6d7fd` | `cgp_auto_getter`, `cgp_type` | stack overflow on `\|\|\|\|` runs (`\|\|` arrives as two `\|` puncts) | every `\|` counts, two per level |
| `91675cfd` | `cgp_namespace` | stack overflow on a 600-character `@path` name | identifiers over 255 characters are rejected before parsing |

Before this branch, release 0.10.1 fixed the first finding: raw identifiers in derived names
(`check_components! { r#r {} }`).

Older slow inputs from earlier rounds (`slow-unit-*` under `fuzz/artifacts/`, ignored by git) run in
40 to 280 ms each under the fuzzer's sanitizers when run alone and are not findings.

## Not run here

The 3-OS and beta/nightly CI matrices, `cargo hack` feature powerset, `scripts/test-packages.sh`,
the compile-time benchmark and the knowledge-base reference check (all green or unchanged in the
[after phase 2](2026-10-31-after-phase-2.md) baseline; no code changed since except the fuzz
fixes, which the full test run covers). Fuzzing was capped at 3 minutes per target, well below
CI's 15-minute `entrypoints` run.
