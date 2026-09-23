#!/usr/bin/env bash

# Print one host-resource line per interval until killed.
#
# Meant to run in the background of a long CI step. A runner that loses its host
# — out of memory, out of disk, or a reclaimed instance — reports as "The
# operation was canceled." with no failing command, and no later step runs to
# collect evidence. Only output already streamed to the live log survives, so
# the last line printed here is the account of what the host looked like as it
# died.

set -uo pipefail

INTERVAL="${1:-30}"

while true; do
  # Tolerate a probe failing rather than ending the sampler.
  DISK=$(df -h --output=avail / 2>/dev/null | tail -1 | tr -d ' ' || echo "?")
  MEM_AVAIL=$(free -m 2>/dev/null | awk '/^Mem:/ {print $7}' || echo "?")
  SWAP_USED=$(free -m 2>/dev/null | awk '/^Swap:/ {print $3}' || echo "?")
  LOAD=$(cut -d ' ' -f1-3 /proc/loadavg 2>/dev/null || echo "?")
  printf '[host] %s disk_avail=%s mem_avail=%sMi swap_used=%sMi load=%s\n' \
    "$(date -u +%H:%M:%S)" "${DISK}" "${MEM_AVAIL}" "${SWAP_USED}" "${LOAD}"
  sleep "${INTERVAL}"
done
