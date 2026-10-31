#!/usr/bin/env python3
"""Compare a compile-time measurement with the rolling median of earlier runs.

The history is a JSON Lines file, one object per recorded run:

    {"secs": 11.84, "sha": "0567fad2...", "date": "2026-10-27T09:12:03Z"}

CI restores it from the Actions cache, checks the new measurement, and (on
pushes to `main` only) appends the run and saves the cache again, so the
series follows `main` and a pull request is compared with it without
changing it.

The check fails when the new value is more than `--max-ratio` times the
median of the last `--window` entries AND more than `--floor-secs` above it.
The absolute floor keeps a fast build from failing on a few tenths of a second
of runner noise. With fewer than `--min-history` entries there is no trend
yet; the check reports that and passes (the fixed budget in
`scripts/compile-time.sh` still applies).

    python3 scripts/compile-time-trend.py --history h.jsonl --value 12.3
    python3 scripts/compile-time-trend.py --history h.jsonl --value 12.3 --record --sha abc
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import statistics
import sys
from pathlib import Path


def load(path: Path) -> list[dict]:
    if not path.exists():
        return []
    entries = []
    for number, line in enumerate(path.read_text().splitlines(), start=1):
        line = line.strip()
        if not line:
            continue
        try:
            entry = json.loads(line)
            float(entry["secs"])
        except (ValueError, KeyError, TypeError) as error:
            # A damaged line must not wedge the series; skip it and say so.
            print(f"compile-time-trend: skipping line {number}: {error}", file=sys.stderr)
            continue
        entries.append(entry)
    return entries


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--history", type=Path, required=True)
    parser.add_argument("--value", type=float, required=True, help="seconds measured now")
    parser.add_argument("--window", type=int, default=10)
    parser.add_argument("--min-history", type=int, default=3)
    parser.add_argument("--max-ratio", type=float, default=1.5)
    parser.add_argument("--floor-secs", type=float, default=2.0)
    parser.add_argument("--keep", type=int, default=200, help="entries kept when recording")
    parser.add_argument("--record", action="store_true", help="append this run to the history")
    parser.add_argument("--sha", default="")
    args = parser.parse_args(argv)

    history = load(args.history)
    recent = [float(entry["secs"]) for entry in history[-args.window :]]

    failed = False
    if len(recent) < args.min_history:
        print(
            f"compile-time-trend: {len(recent)} earlier run(s), need {args.min_history}; "
            f"no trend check yet (now {args.value:.2f}s)"
        )
    else:
        median = statistics.median(recent)
        limit = max(median * args.max_ratio, median + args.floor_secs)
        print(
            f"compile-time-trend: now {args.value:.2f}s, median of last {len(recent)} "
            f"{median:.2f}s, limit {limit:.2f}s "
            f"(x{args.max_ratio} and at least +{args.floor_secs}s)"
        )
        if args.value > limit:
            print("compile-time-trend: slower than the trend allows", file=sys.stderr)
            failed = True

    # A regression is recorded too: the series should show it, and a later fix
    # brings the median back down.
    if args.record:
        history.append(
            {
                "secs": round(args.value, 2),
                "sha": args.sha,
                "date": dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            }
        )
        history = history[-args.keep :]
        args.history.parent.mkdir(parents=True, exist_ok=True)
        args.history.write_text("".join(json.dumps(entry) + "\n" for entry in history))
        print(f"compile-time-trend: recorded; history has {len(history)} run(s)")

    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
