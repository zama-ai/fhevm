#!/usr/bin/env bash
# Removes every proxy and forwarder started by proxies-up.sh.
names=$(docker ps -a --format '{{.Names}}' | grep -E '^(kms-connector(-[0-9]+)?-proxy|relayer-fwd-[0-9]+)$' || true)
[ -n "$names" ] && docker rm -f $names >/dev/null && echo "removed: $(echo $names | tr '\n' ' ')" || echo "nothing to remove"
