#!/usr/bin/env bash
# Compile-time budget for CGP wiring.
#
# Generates a throwaway crate with N components, N providers, and one context
# wiring all of them (plus a `check_components!` over every entry), then times a
# clean `cargo build` of just that crate (dependencies are built first and not
# counted). Fails when the build exceeds the budget.
#
#   bash scripts/compile-time.sh                 # N=200, budget from scripts/compile-budget.txt
#   N=500 BUDGET_SECS=60 bash scripts/compile-time.sh
#   RESULT_FILE=secs.txt bash scripts/compile-time.sh   # also write the seconds alone
#   KEEP_WORK=1 bash scripts/compile-time.sh             # keep the generated crate
#
# The trend over time (rolling median of earlier `main` runs) is checked
# separately by `scripts/compile-time-trend.py`, fed from RESULT_FILE.
#
# The budget is deliberately loose (CI machines vary); the goal is to catch a
# macro-expansion blow-up, not a few percent of noise.
set -euo pipefail
# Decimal points and `date` must not depend on the locale (`awk` would treat
# `1.5` as zero under a comma decimal and the budget check would never fire).
export LC_ALL=C
# The timed step rebuilds an unchanged file; with incremental compilation on it
# can reuse the cache and under-measure. CI already sets this; local runs need it too.
export CARGO_INCREMENTAL=0

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
n="${N:-200}"
budget="${BUDGET_SECS:-$(tr -d '[:space:]' < "$root/scripts/compile-budget.txt")}"
work="$(mktemp -d)"
if [[ -n "${KEEP_WORK:-}" ]]; then
  echo "compile-time: keeping $work" >&2
else
  trap 'rm -rf "$work"' EXIT
fi

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
case "$start" in
  *N*)
    echo "compile-time: this date(1) has no %N (nanoseconds); need GNU date" >&2
    exit 1
    ;;
esac
cargo build --quiet
end=$(date +%s.%N)
elapsed=$(awk -v s="$start" -v e="$end" 'BEGIN { printf "%.2f", e - s }')

echo "compile-time: ${n} components wired and checked in ${elapsed}s (budget ${budget}s)"
if [[ -n "${RESULT_FILE:-}" ]]; then
  echo "$elapsed" > "$RESULT_FILE"
fi
if awk -v e="$elapsed" -v b="$budget" 'BEGIN { exit !(e > b) }'; then
  echo "compile-time: over budget" >&2
  exit 1
fi
