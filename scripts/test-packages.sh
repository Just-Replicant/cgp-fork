#!/usr/bin/env bash
# Package both crates, unpack the `.crate` files, and run their tests from the
# unpacked sources, the way a distro packager or crater run sees them.
#
#   bash scripts/test-packages.sh
#
# The facade is tested against the unpacked macro crate (a `[patch.crates-io]`
# override), because the version being released is not on crates.io yet.
# Also checks that each package ships its LICENSE.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
version="$(cargo metadata --no-deps --format-version 1 \
  | python3 -c 'import json,sys; print(next(p["version"] for p in json.load(sys.stdin)["packages"] if p["name"] == "cgp-fork"))')"

# `--allow-dirty` so the check also runs on uncommitted local changes.
# `cgp-fork-doctests` (publish = false) reads files outside its own directory,
# so it cannot be verified from a package; it is never published anyway.
cargo package --workspace --exclude cgp-fork-doctests --locked --allow-dirty

for crate in cgp-fork cgp-fork-macro; do
  if ! cargo package -p "$crate" --list --allow-dirty | grep -qx LICENSE; then
    echo "test-packages: $crate does not ship LICENSE" >&2
    exit 1
  fi
done

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
for crate in cgp-fork-macro cgp-fork; do
  tar -xzf "target/package/$crate-$version.crate" -C "$work"
done

macro_dir="$work/cgp-fork-macro-$version"
export CARGO_TARGET_DIR="$work/target"

echo "test-packages: cargo test in cgp-fork-macro-$version"
(cd "$macro_dir" && cargo test --all-features)

echo "test-packages: cargo test in cgp-fork-$version"
(cd "$work/cgp-fork-$version" && cargo test --all-features \
  --config "patch.crates-io.cgp-fork-macro.path=\"$macro_dir\"")

echo "test-packages: both packaged crates pass their tests"
