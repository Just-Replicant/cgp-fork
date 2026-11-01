#!/usr/bin/env bash
# Check that every `cgp-knowledge-base-fork/<path>` reference in this repository
# names a file that exists in a local knowledge-base checkout.
#
#   bash scripts/check-kb-refs.sh [path-to-cgp-knowledge-base-fork]
#
# Defaults to the sibling checkout `../cgp-knowledge-base-fork` (see
# sibling-projects.md). Anchors (`#…`) are ignored.
#
# A reference whose page is missing from the knowledge base but drafted under
# `docs/kb-pages/` (same relative path) is reported as pending, not dangling:
# the draft is waiting to be copied into the knowledge base. Exits non-zero
# when a reference is dangling, or, with KB_REFS_STRICT=1, also when one is
# pending.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
kb="${1:-$root/../cgp-knowledge-base-fork}"
drafts="$root/docs/kb-pages"

if [[ ! -d "$kb" ]]; then
  echo "knowledge base checkout not found at $kb" >&2
  exit 2
fi

missing=0
pending=0
total=0
while IFS= read -r ref; do
  total=$((total + 1))
  path="${ref#cgp-knowledge-base-fork/}"
  path="${path%%#*}"
  if [[ -e "$kb/$path" ]]; then
    continue
  elif [[ -e "$drafts/$path" ]]; then
    echo "pending: $ref (drafted in docs/kb-pages/$path)"
    pending=$((pending + 1))
  else
    echo "dangling: $ref"
    missing=$((missing + 1))
  fi
done < <(cd "$root" && grep -rhoE 'cgp-knowledge-base-fork/[A-Za-z0-9_./#-]+[A-Za-z0-9_/#-]' \
  --include='*.rs' --include='*.md' --exclude-dir=target --exclude-dir=.git . | sort -u)

echo "checked $total distinct references, $missing dangling, $pending pending in docs/kb-pages"
[[ "$missing" -eq 0 ]]
if [[ "${KB_REFS_STRICT:-0}" == "1" ]]; then
  [[ "$pending" -eq 0 ]]
fi
