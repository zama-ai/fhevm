"""Assert the three-operator, two-chain E2E stack has no legacy host ingestion."""
import json
import os
import subprocess

ids = subprocess.check_output(["docker", "ps", "-q"], text=True).split()
assert ids, "No running containers"
containers = json.loads(subprocess.check_output(["docker", "inspect", *ids], text=True))
consumers = []
publishers = []
for container in containers:
    name = container["Name"].lstrip("/")
    command = container["Config"].get("Cmd") or []
    executable = os.path.basename(command[0]) if command else ""
    assert executable not in ("host_listener", "host_listener_poller"), f"Legacy ingestion running: {name}"
    if executable == "host_listener_consumer":
        consumers.append(name)
    if name.startswith("listener-publisher-"):
        publishers.append(name)
assert len(consumers) == 6, f"Expected 6 host consumers, found: {consumers}"
assert len(publishers) == 2, f"Expected 2 listener-core producers, found: {publishers}"
print(f"Consumer-only stack: {sorted(consumers)}; producers: {sorted(publishers)}")
