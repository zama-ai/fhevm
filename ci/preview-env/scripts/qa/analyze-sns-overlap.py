#!/usr/bin/env python3
"""Measure conversion-call overlap from actual timestamped SNS worker logs."""
from collections import defaultdict
from datetime import datetime
import json
from pathlib import Path
import re
import sys

folder = Path(sys.argv[1]).resolve()
events = []
matched = {}
for worker in range(1, 5):
    name = f"sns-load-{worker}"
    pending = {}
    count = 0
    for line in (folder / f"{name}.log").read_text().splitlines():
        if "Converting ciphertext" not in line and "Ciphertext converted" not in line:
            continue
        timestamp, _, payload = line.partition(" ")
        timestamp = datetime.fromisoformat(timestamp.replace("Z", "+00:00")).timestamp()
        if payload.startswith("{"):
            fields = json.loads(payload).get("fields", {})
            message = fields.get("message", "")
            handle = str(fields.get("handle", ""))
        else:
            payload = re.sub(r"\x1b\[[0-9;]*m", "", payload)
            message = payload
            match = re.search(r'handle[=:]\s*"?((?:0x)?[0-9a-fA-F]{64})', payload)
            handle = match.group(1) if match else ""
        if not handle:
            raise ValueError(f"Cannot identify a conversion handle in {name}")
        if "Converting ciphertext" in message:
            pending[handle] = timestamp
        elif "Ciphertext converted" in message:
            start = pending.pop(handle, None)
            if start is None:
                raise ValueError(f"Conversion end without a start in {name}: {handle}")
            assert timestamp >= start
            events.extend(((start, worker, 1), (timestamp, worker, -1)))
            count += 1
    matched[name] = {"completed_conversion_intervals": count, "unfinished_intervals": len(pending)}

assert events, "No conversion intervals found"
active = defaultdict(int)
durations = defaultdict(float)
previous = min(t for t, _, _ in events)
for timestamp, worker, delta in sorted(events):
    concurrent = sum(value > 0 for value in active.values())
    durations[concurrent] += timestamp - previous
    active[worker] += delta
    assert active[worker] >= 0
    previous = timestamp
total = sum(durations.values())
result = {
    "workers": matched,
    "seconds_by_concurrent_workers": {str(k): round(v, 6) for k, v in sorted(durations.items())},
    "four_worker_overlap_percent": round(100 * durations[4] / total, 3),
    "conversion_wall_seconds": round(total, 6),
    "scope": "Synchronous squash_noise_and_serialize calls in flight, from start/end logs; not DB write overlap or a linear speedup estimate",
}
(folder / "conversion-overlap.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result, indent=2))
