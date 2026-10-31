#!/usr/bin/env bash
# Package every published crate, unpack the `.crate` files, and run their tests
# from the unpacked sources, the way a distro packager or crater run sees them.
#
#   bash scripts/test-packages.sh
#
# Each crate is tested against the unpacked crates it depends on (a
# `[patch.crates-io]` override), because the version being released is not on
# crates.io yet.
# Also checks that each package ships its LICENSE.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
version="$(cargo metadata --no-deps --format-version 1 \
  | python3 -c 'import json,sys; print(next(p["version"] for p in json.load(sys.stdin)["packages"] if p["name"] == "cgp-fork"))')"

# `--allow-dirty` so the check also runs on uncommitted local changes.
# `cgp-fork-doctests`, `cgp-fork-renamed-dep` and `cgp-fork-preset-consumer`
# (publish = false) are workspace-only checks that read files outside their own
# directory or depend on another workspace crate by path; they are never published.
cargo package --workspace --exclude cgp-fork-doctests --exclude cgp-fork-renamed-dep \
  --exclude cgp-fork-preset-consumer --locked --allow-dirty

for crate in cgp-fork cgp-fork-macro cgp-fork-macro-impl; do
  if ! cargo package -p "$crate" --list --allow-dirty | grep -qx LICENSE; then
    echo "test-packages: $crate does not ship LICENSE" >&2
    exit 1
  fi
done

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
for crate in cgp-fork-macro-impl cgp-fork-macro cgp-fork; do
  tar -xzf "target/package/$crate-$version.crate" -C "$work"
done

impl_dir="$work/cgp-fork-macro-impl-$version"
macro_dir="$work/cgp-fork-macro-$version"
export CARGO_TARGET_DIR="$work/target"
patch_impl="patch.crates-io.cgp-fork-macro-impl.path=\"$impl_dir\""
patch_macro="patch.crates-io.cgp-fork-macro.path=\"$macro_dir\""

echo "test-packages: cargo test in cgp-fork-macro-impl-$version"
(cd "$impl_dir" && cargo test --all-features)

echo "test-packages: cargo test in cgp-fork-macro-$version"
(cd "$macro_dir" && cargo test --all-features --config "$patch_impl")

echo "test-packages: cargo test in cgp-fork-$version"
(cd "$work/cgp-fork-$version" && cargo test --all-features \
  --config "$patch_impl" --config "$patch_macro")

echo "test-packages: every packaged crate passes its tests"
