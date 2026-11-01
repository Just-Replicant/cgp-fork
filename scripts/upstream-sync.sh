#!/usr/bin/env bash
# Report (and optionally merge) upstream contextgeneric/cgp changes.
#
#   bash scripts/upstream-sync.sh            # fetch upstream, list commits not yet merged
#   bash scripts/upstream-sync.sh --merge    # also merge upstream/main into a new sync branch
#   bash scripts/upstream-sync.sh --check    # exit 1 when a commit is neither merged nor reviewed
#
# The fork records the last merged upstream commit in `UPSTREAM_COMMIT` at the
# repository root (and the README's "includes upstream through" line). After a
# merge, update both, run the full verification in CONTRIBUTING.md, and open a PR.
#
# `UPSTREAM_REVIEWED` lists newer upstream commits that were reviewed and
# deliberately skipped (for example docs that only concern upstream). They are
# reported as reviewed, not pending, so a weekly check only flags new work.
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

reviewed=""
if [[ -f UPSTREAM_REVIEWED ]]; then
  # `|| true`: a file with only comments is fine, and grep exits 1 on no match.
  reviewed="$( (grep -vE '^[[:space:]]*(#|$)' UPSTREAM_REVIEWED || true) | awk '{print $1}')"
fi

echo "Last merged upstream commit: $synced"
newer="$(git rev-list --count "$synced..upstream/$branch")"
unreviewed=0
while IFS= read -r sha; do
  [[ -z "$sha" ]] && continue
  line="$(git log -1 --format='%h %s' "$sha")"
  if grep -qx "$sha" <<<"$reviewed"; then
    echo "  reviewed, not merged: $line"
  else
    echo "  pending:              $line"
    unreviewed=$((unreviewed + 1))
  fi
done < <(git rev-list --reverse "$synced..upstream/$branch")
echo "Upstream commits not yet merged: $newer ($unreviewed not reviewed)"
pending="$newer"

if [[ "${1:-}" == "--check" ]]; then
  [[ "$unreviewed" -eq 0 ]]
  exit
fi

if [[ "${1:-}" == "--merge" && "$pending" -gt 0 ]]; then
  target="$(git rev-parse --short "upstream/$branch")"
  git switch -c "upstream-sync-$target"
  git merge --no-ff "upstream/$branch" -m "chore(upstream): merge contextgeneric/cgp through $target"
  git rev-parse "upstream/$branch" > UPSTREAM_COMMIT
  echo "Merged. Resolve the crate-layout differences (upstream is multi-crate), update the"
  echo "README line, commit UPSTREAM_COMMIT, and run the checks in CONTRIBUTING.md."
fi
