"""Assert the emergency fallback uses legacy ingestion on all operators/chains."""
import json
import os
import subprocess

ids = subprocess.check_output(["docker", "ps", "-q"], text=True).split()
assert ids, "No running containers"
containers = json.loads(subprocess.check_output(["docker", "inspect", *ids], text=True))
listeners = []
pollers = []
for container in containers:
    name = container["Name"].lstrip("/")
    command = container["Config"].get("Cmd") or []
    executable = os.path.basename(command[0]) if command else ""
    assert executable != "host_listener_consumer", f"Consumer masking legacy ingestion: {name}"
    if executable == "host_listener":
        listeners.append(name)
    if executable == "host_listener_poller":
        pollers.append(name)
expected = {
    f"{operator}-host-listener{chain}"
    for operator in ("coprocessor", "coprocessor1", "coprocessor2")
    for chain in ("", "-chain-b")
}
assert set(listeners) == expected, f"Unexpected legacy listeners: {listeners}"
assert set(pollers) == {name.replace("host-listener", "host-listener-poller") for name in expected}, f"Unexpected legacy pollers: {pollers}"
print(f"Legacy-only stack: {sorted(listeners)}; pollers: {sorted(pollers)}")
