#!/usr/bin/env bash
# Report (and optionally merge) upstream contextgeneric/cgp changes.
#
#   bash scripts/upstream-sync.sh            # fetch upstream, list commits not yet merged
#   bash scripts/upstream-sync.sh --merge    # also merge upstream/main into a new sync branch
#
# The fork records the last merged upstream commit in `UPSTREAM_COMMIT` at the
# repository root (and the README's "includes upstream through" line). After a
# merge, update both, run the full verification in CONTRIBUTING.md, and open a PR.
#
# This script only fetches over anonymous HTTPS; it never pushes.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

url="${UPSTREAM_URL:-https://github.com/contextgeneric/cgp}"
branch="${UPSTREAM_BRANCH:-main}"
synced="$(tr -d '[:space:]' < UPSTREAM_COMMIT)"

if ! git remote get-url upstream >/dev/null 2>&1; then
  git remote add upstream "$url"
fi
GIT_TERMINAL_PROMPT=0 git fetch --no-tags upstream "$branch"

echo "Last merged upstream commit: $synced"
pending="$(git rev-list --count "$synced..upstream/$branch")"
echo "Upstream commits not yet merged: $pending"
git log --oneline "$synced..upstream/$branch"

if [[ "${1:-}" == "--merge" && "$pending" -gt 0 ]]; then
  target="$(git rev-parse --short "upstream/$branch")"
  git switch -c "upstream-sync-$target"
  git merge --no-ff "upstream/$branch" -m "chore(upstream): merge contextgeneric/cgp through $target"
  git rev-parse "upstream/$branch" > UPSTREAM_COMMIT
  echo "Merged. Resolve the crate-layout differences (upstream is multi-crate), update the"
  echo "README line, commit UPSTREAM_COMMIT, and run the checks in CONTRIBUTING.md."
fi
