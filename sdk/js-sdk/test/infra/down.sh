#!/usr/bin/env bash
set -uo pipefail

# Stop the whole test infra: both anvils, the gateway, and any leftover Next dev
# server. Force-kills whatever holds the ports, so it also cleans up reused
# anvils that up.sh's Ctrl-C leaves running (stopAnvils only stops what it
# spawned). Ports mirror test/infra/topology.ts + browser-next (Next on 3334).

PORTS=(8544 8546 8590 3334)

for p in "${PORTS[@]}"; do
  pids="$(lsof -ti "tcp:${p}" 2>/dev/null || true)"
  if [[ -n "${pids}" ]]; then
    echo "stopping port ${p} (pids: ${pids//$'\n'/ })"
    # shellcheck disable=SC2086
    kill ${pids} 2>/dev/null || true
  fi
done

# Anvil is wrapped by fhevm-anvil.sh; kill any stragglers too.
pkill -f "fhevm-anvil.sh" 2>/dev/null || true

sleep 1

# Force-kill any survivors still holding the ports.
for p in "${PORTS[@]}"; do
  pids="$(lsof -ti "tcp:${p}" 2>/dev/null || true)"
  if [[ -n "${pids}" ]]; then
    # shellcheck disable=SC2086
    kill -9 ${pids} 2>/dev/null || true
  fi
done

echo "infra stopped."
