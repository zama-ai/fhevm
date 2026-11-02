#!/usr/bin/env bash
# Smoke through forwarder -> proxy: an unsigned request must come back as `401 auth_missing` from every proxy (a
# signed one needs the relayer). Connection refused = forwarder down, `000` (empty reply) = proxy down. Usage: proxies-status.sh N
N="${1:?number of KMS connectors}"
ok=0
for i in $(seq 1 "$N"); do
  for _ in $(seq 1 10); do
    code=$(curl -s -o /tmp/relayer-http-dev-smoke.json -w '%{http_code}' -X POST "http://127.0.0.1:808$i/v1/public-decrypt" \
      -H 'content-type: application/json' -d '{}' || true)
    [ "$code" = 401 ] && break
    sleep 1
  done
  [ "$code" = 401 ] && ok=$((ok + 1))
  echo "  127.0.0.1:808$i -> $code $(head -c 120 /tmp/relayer-http-dev-smoke.json 2>/dev/null)"
done
echo "chain OK for $ok/$N parties"
[ "$ok" = "$N" ]
