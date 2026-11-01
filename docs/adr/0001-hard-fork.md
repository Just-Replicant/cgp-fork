# ADR 0001: Become an independent project (hard fork)

- Status: **Proposed**, waiting for the maintainer's decision
- Date: 2026-10-30
- Plan: [rewrite plan §5](../rewrite-plan.md#5-strategic-decision-first-hard-fork)

## Context

cgp-fork tracks upstream `contextgeneric/cgp`. It records the last merged upstream commit in
`UPSTREAM_COMMIT`, skipped commits in `UPSTREAM_REVIEWED`, and checks for new upstream work weekly
(`scripts/upstream-sync.sh`, `.github/workflows/upstream-sync.yml`). `CONTRIBUTING.md` describes how
upstream changes are merged.

The rewrite plan changes behavior and public names (handlers, dispatch, async, wiring checks).
After phase 3, upstream patches would no longer apply, so continuing to merge them stops being
practical.

## Options

1. **Hard fork.** Stop merging upstream; port useful upstream changes by hand; versions and API
   evolve independently from 1.0.
2. **Stay mergeable.** Keep merging upstream; propose phases 3 and 4 upstream first and land them
   here only after upstream does.
3. **Hybrid.** Stay mergeable through phases 0 to 2 (internal and diagnostic changes), and decide
   again before phase 3.

## Decision (proposed)

Option 3 now, leading to option 1 before phase 3. Phases 0 and 1 change only internals and
tooling, so they do not force the choice yet.

## Consequences

- Nothing upstream-related is removed in phases 0 and 1: `UPSTREAM_COMMIT`, `UPSTREAM_REVIEWED`,
  the sync script and workflow stay and keep working. This keeps the decision reversible.
- Phase 1 moves the macro implementation to `crates/macros/cgp-fork-macro-impl` (ADR 0002), so an
  upstream merge must map upstream's macro paths to that crate. `CONTRIBUTING.md` says where.
- If the maintainer accepts option 1: delete the upstream-sync workflow, script and files, change
  this record to Accepted, and replace the "includes upstream through" line in the README with a
  note on the fork point.
- If the maintainer rejects it: change this record to Rejected and re-plan phases 3 to 5 as
  upstream proposals.
