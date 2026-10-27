#!/usr/bin/env bash
# Compile-time budget for CGP wiring.
#
# Generates a throwaway crate with N components, N providers, and one context
# wiring all of them (plus a `check_components!` over every entry), then times a
# clean `cargo build` of just that crate (dependencies are built first and not
# counted). Fails when the build exceeds the budget.
#
#   scripts/compile-time.sh                 # N=200, budget from scripts/compile-budget.txt
#   N=500 BUDGET_SECS=60 scripts/compile-time.sh
#
# The budget is deliberately loose (CI machines vary); the goal is to catch a
# macro-expansion blow-up, not a few percent of noise.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
n="${N:-200}"
budget="${BUDGET_SECS:-$(tr -d '[:space:]' < "$root/scripts/compile-budget.txt")}"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

mkdir -p "$work/src"
cat > "$work/Cargo.toml" <<TOML
[package]
name = "cgp-compile-time"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
cgp-fork = { path = "$root/crates/main/cgp-fork" }

[workspace]
TOML
cp "$root/rust-toolchain.toml" "$work/"

{
  echo 'use cgp_fork::prelude::*;'
  echo
  for i in $(seq 1 "$n"); do
    cat <<RS
#[cgp_component(Op${i}Provider)]
pub trait CanOp${i} {
    fn op_${i}(&self, input: u64) -> u64;
}

#[cgp_impl(new Op${i}Impl)]
impl Op${i}Provider
where
    Self: HasBase,
{
    fn op_${i}(&self, input: u64) -> u64 {
        input.wrapping_mul(${i}).wrapping_add(*self.base())
    }
}

RS
  done
  cat <<'RS'
#[cgp_auto_getter]
pub trait HasBase {
    fn base(&self) -> &u64;
}

#[derive(HasField)]
pub struct App {
    pub base: u64,
}

delegate_components! {
    App {
RS
  for i in $(seq 1 "$n"); do echo "        Op${i}ProviderComponent: Op${i}Impl,"; done
  echo '    }'
  echo '}'
  echo
  echo 'check_components! {'
  echo '    App {'
  for i in $(seq 1 "$n"); do echo "        Op${i}ProviderComponent,"; done
  echo '    }'
  echo '}'
  echo
  echo 'fn main() {'
  echo '    let app = App { base: 1 };'
  echo "    println!(\"{}\", app.op_${n}(2));"
  echo '}'
} > "$work/src/main.rs"

cd "$work"
# Build dependencies first so only the generated crate is timed.
cargo build --quiet --offline 2>/dev/null || cargo build --quiet
touch src/main.rs
start=$(date +%s.%N)
cargo build --quiet
end=$(date +%s.%N)
elapsed=$(awk -v s="$start" -v e="$end" 'BEGIN { printf "%.2f", e - s }')

echo "compile-time: ${n} components wired and checked in ${elapsed}s (budget ${budget}s)"
if awk -v e="$elapsed" -v b="$budget" 'BEGIN { exit !(e > b) }'; then
  echo "compile-time: over budget" >&2
  exit 1
fi
