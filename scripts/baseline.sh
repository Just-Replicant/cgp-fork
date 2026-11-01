#!/usr/bin/env bash
# Record a baseline of the repository's health as a Markdown report.
#
#   bash scripts/baseline.sh                      # print the report
#   bash scripts/baseline.sh > docs/baseline/$(date +%F).md
#
# Measures, on the current checkout: toolchain, commit, test counts
# (`cargo test --workspace --all-features`), clippy status, the compile-time
# benchmark (`scripts/compile-time.sh`), and source size. Every rewrite phase
# compares against the committed baseline (see docs/rewrite-plan.md, phase 0).
#
# Dates are UTC. It only reads the tree and builds into `target/`; it never
# commits.
set -euo pipefail
export LC_ALL=C

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
log="$(mktemp)"
trap 'rm -f "$log"' EXIT

count_lines() {
  find "$@" -name '*.rs' -print0 2>/dev/null | xargs -0 cat 2>/dev/null | wc -l | tr -d ' '
}
count_files() {
  find "$@" -name '*.rs' 2>/dev/null | wc -l | tr -d ' '
}

cargo test --workspace --all-features >"$log" 2>&1 || tests_status="failing"
read -r passed failed ignored < <(awk '/^test result:/ {p+=$4; f+=$6; i+=$8} END {print p+0, f+0, i+0}' "$log")

if cargo clippy --workspace --all-features --all-targets --quiet -- -D warnings >/dev/null 2>&1; then
  clippy="clean"
else
  clippy="warnings or errors"
fi

bench_file="$(mktemp)"
RESULT_FILE="$bench_file" bash scripts/compile-time.sh >/dev/null 2>&1 || true
bench="$(tr -d '[:space:]' < "$bench_file" 2>/dev/null || true)"
rm -f "$bench_file"

cat <<REPORT
# Baseline: $(git rev-parse --short HEAD) ($(TZ=UTC git log -1 --date=format-local:%Y-%m-%d --format=%cd))

| Measure | Value |
|---|---|
| Commit | \`$(git rev-parse HEAD)\` |
| Toolchain | \`$(rustc --version)\` |
| Tests passed | ${passed} |
| Tests failed | ${failed}${tests_status:+ (${tests_status})} |
| Tests ignored | ${ignored} |
| Clippy (\`-D warnings\`, all targets, all features) | ${clippy} |
| Compile-time bench (200 components, wired and checked) | ${bench:-n/a} s |
| Facade \`src\` | $(count_files crates/main/cgp-fork/src) files, $(count_lines crates/main/cgp-fork/src) lines |
| Macro crates \`src\` | $(count_files crates/macros) files, $(count_lines crates/macros) lines |
| Facade integration tests | $(count_files crates/main/cgp-fork/tests) files, $(count_lines crates/main/cgp-fork/tests) lines |
REPORT
