#!/usr/bin/env bash
# Sends k6 traffic (dev/load/relayer-http.js) to a running relayer-http: `make load PROFILE=normal|load`.
#   TARGET=fhevm-cli (default)  the stack's relayer, on RELAYER_HTTP_PORT
#   TARGET=host                 `make run`, on RELAYER_HTTP_RUN_PORT
# Valid handles: HANDLES (comma-separated), or those of the stack relayer's last successful public decrypt, from its
# summary line (`make public-decrypt` deploys a fixture and decrypts it). Without any, only the invalid requests run.
# k6 runs in its pinned image on the host network, to reach the relayer on 127.0.0.1.
set -euo pipefail
DIR="$(cd "$(dirname "$0")/.." && pwd)"
case "${TARGET:-fhevm-cli}" in
  fhevm-cli) PORT="${RELAYER_HTTP_PORT:?run through make}" ;;
  host) PORT="${RELAYER_HTTP_RUN_PORT:?run through make}" ;;
  *) echo "TARGET must be fhevm-cli or host"; exit 1 ;;
esac
URL="${RELAYER_HTTP_URL:-http://127.0.0.1:$PORT}"

if [ -z "${HANDLES:-}" ]; then
  HANDLES="$(docker logs fhevm-relayer-http 2>&1 \
    | grep '"message":"request completed"' | grep '"flow":"public_decrypt"' | grep '"outcome":"succeeded"' \
    | tail -1 | sed -nE 's/.*"handles":"([^"]+)".*/\1/p')" || true
fi
if [ -z "$HANDLES" ]; then
  echo "no valid handles (no successful public decrypt in fhevm-relayer-http's logs): run make public-decrypt first;"
  echo "sending the invalid requests only"
else
  echo "handles: $HANDLES"
  # The connectors accept only the extraData of the current KMS context: read on-chain by the e2e container.
  if [ -z "${EXTRA_DATA:-}" ]; then
    EXTRA_DATA="$("$DIR/scripts/e2e.sh" extra-data.ts | sed -nE 's/^extraData=(0x[0-9a-fA-F]+)$/\1/p')"
  fi
  [ -n "$EXTRA_DATA" ] || { echo "no extraData from the e2e container (dev/e2e/extra-data.ts)"; exit 1; }
  echo "extraData: $EXTRA_DATA"
fi
echo "profile ${PROFILE:-normal} -> $URL"

docker run --rm -i --network host \
  -e RELAYER_HTTP_URL="$URL" -e HANDLES="$HANDLES" -e EXTRA_DATA="${EXTRA_DATA:-}" -e PROFILE="${PROFILE:-normal}" \
  -e RATE="${RATE:-}" -e DURATION="${DURATION:-}" \
  grafana/k6:0.57.0 run --quiet - < "$DIR/load/relayer-http.js"
