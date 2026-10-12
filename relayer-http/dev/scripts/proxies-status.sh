#!/usr/bin/env bash
# Smoke through the whole chain: an empty body must come back as `400 malformed` from every endpoint.
# 401 = API key digest mismatch, 502 = the proxy cannot reach its endpoint (tag mismatch or endpoint down),
# connection refused = forwarder down. Usage: proxies-status.sh N
N="${1:?number of KMS connectors}"
API_KEY="${API_KEY:-fhevm-e2e-kms-connector-api-key}"
ok=0
for i in $(seq 1 "$N"); do
  for _ in $(seq 1 10); do
    code=$(curl -s -o /tmp/relayer-http-dev-smoke.json -w '%{http_code}' -X POST "http://127.0.0.1:808$i/v1/public-decrypt" \
      -H 'content-type: application/json' -H "authorization: Bearer $API_KEY" -d '{}' || true)
    [ "$code" = 400 ] && break
    sleep 1
  done
  [ "$code" = 400 ] && ok=$((ok + 1))
  echo "  127.0.0.1:808$i -> $code $(head -c 120 /tmp/relayer-http-dev-smoke.json 2>/dev/null)"
done
echo "chain OK for $ok/$N parties"
[ "$ok" = "$N" ]
