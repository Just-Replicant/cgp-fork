## What and why

<!-- One or two sentences. Link the issue if there is one. -->

## Checklist

- [ ] The commit subjects follow Conventional Commits (`feat:`, `fix:`, `docs:`, … and `!` for breaking changes); release-plz builds the changelog from them
- [ ] Tests cover the change (behaviour test, snapshot, rejection case, or `tests/ui` case — see `crates/tests/README.md`)
- [ ] `cargo +nightly fmt --all`, `cargo clippy --workspace --all-features --all-targets -- -D warnings`
- [ ] `cargo test --workspace --all-features` and `--no-default-features`
- [ ] Docs updated (rustdoc, README, CHANGELOG `Unreleased` for anything user-visible that the commit log does not explain)
