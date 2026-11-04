#!/usr/bin/env bash
# Runs relayer-http on the host with cargo, on RELAYER_HTTP_RUN_PORT (set by the Makefile), against the running stack
# (./fhevm-cli up). Logs in this terminal; Ctrl-C stops the relayer. The forwarders stay up for the next run
# (reused when they still match) until `make down` removes them.
#
# Config: the stack's own relayer-http config (rendered by fhevm-cli), with each KMS proxy URL replaced by a local
# forwarder. The proxies are reachable only on the stack network, so one socat container per party listens on
# 127.0.0.1:RELAYER_HTTP_RUN_PORT+i and opens the TLS connection to kms-connector[-i]-proxy:8443; the relayer
# speaks plain http to loopback. Signing key and log level: the stack's relayer-http.env.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
STATE="${FHEVM_STATE_DIR:-$(cd "$ROOT/.." && pwd)/.fhevm}"
STACK_CONFIG="$STATE/runtime/config/relayer-http.yaml"
STACK_ENV="$STATE/runtime/env/relayer-http.env"
CONFIG="$ROOT/target/dev-config.yaml"
PORT="${RELAYER_HTTP_RUN_PORT:?run through make (dev/Makefile sets it)}"
[ -f "$STACK_CONFIG" ] && [ -f "$STACK_ENV" ] || { echo "no relayer-http config in $STATE/runtime: start the stack first (./fhevm-cli up)"; exit 1; }

PROXIES=()
while IFS= read -r proxy; do PROXIES+=("$proxy"); done < <(sed -nE 's#^ *url: "?https://([^"]+)"?$#\1#p' "$STACK_CONFIG")
[ "${#PROXIES[@]}" -gt 0 ] || { echo "no KMS endpoint in $STACK_CONFIG"; exit 1; }

# One forwarder per party, labelled with its host port and proxy: a running one with the same label is reused,
# anything else under that name is replaced.
for i in "${!PROXIES[@]}"; do
  name="relayer-http-fwd-$((i + 1))"
  route="127.0.0.1:$((PORT + i + 1)) -> ${PROXIES[$i]}"
  if [ "$(docker inspect -f '{{.State.Running}} {{index .Config.Labels "route"}}' "$name" 2>/dev/null)" = "true $route" ]; then
    echo "forwarder $route (reused)"
    continue
  fi
  docker rm -f "$name" >/dev/null 2>&1 || true
  # verify=0: the stack's test proxies; the relayer-to-proxy TLS path is exercised by the in-stack relayer.
  docker run -d --name "$name" --label "route=$route" --network fhevm_default -p "127.0.0.1:$((PORT + i + 1)):8443" \
    alpine/socat TCP-LISTEN:8443,fork,reuseaddr "OPENSSL:${PROXIES[$i]},verify=0" >/dev/null
  echo "forwarder $route"
done

mkdir -p "$(dirname "$CONFIG")"
awk -v port="$PORT" '
  /^ *endpoint: / { sub(/0\.0\.0\.0:[0-9]+/, "0.0.0.0:" port) }
  /^ *url: "?https:\/\// { n++; sub(/"?https:\/\/[^"]+"?$/, "http://127.0.0.1:" port + n) }
  { print }
' "$STACK_CONFIG" > "$CONFIG"
echo "relayer-http on :$PORT with $CONFIG"

set -a
# shellcheck disable=SC1090 # the stack's generated env file
. "$STACK_ENV"
set +a
cd "$ROOT" && cargo run -p relayer-http -- "$CONFIG"
