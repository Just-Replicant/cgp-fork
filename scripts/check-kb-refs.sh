#!/usr/bin/env bash
# Check that every `cgp-knowledge-base-fork/<path>` reference in this repository
# names a file that exists in a local knowledge-base checkout.
#
#   scripts/check-kb-refs.sh [path-to-cgp-knowledge-base-fork]
#
# Defaults to the sibling checkout `../cgp-knowledge-base-fork` (see
# sibling-projects.md). Anchors (`#…`) are ignored. Exits non-zero when a
# reference is dangling.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
kb="${1:-$root/../cgp-knowledge-base-fork}"

if [[ ! -d "$kb" ]]; then
  echo "knowledge base checkout not found at $kb" >&2
  exit 2
fi

missing=0
total=0
while IFS= read -r ref; do
  total=$((total + 1))
  path="${ref#cgp-knowledge-base-fork/}"
  path="${path%%#*}"
  if [[ ! -e "$kb/$path" ]]; then
    echo "dangling: $ref"
    missing=$((missing + 1))
  fi
done < <(cd "$root" && grep -rhoE 'cgp-knowledge-base-fork/[A-Za-z0-9_./#-]+[A-Za-z0-9_/#-]' \
  --include='*.rs' --include='*.md' --exclude-dir=target --exclude-dir=.git . | sort -u)

echo "checked $total distinct references, $missing dangling"
[[ "$missing" -eq 0 ]]
