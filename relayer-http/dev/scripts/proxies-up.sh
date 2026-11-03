#!/usr/bin/env bash
# Starts, on the compose network of the local stack, one KMS connector proxy per party (the stack does not ship
# them) and one socat forwarder per proxy, published on 127.0.0.1:8081..808N, for a relayer-http run on the host.
# The forwarder terminates the proxy's TLS, issued by a test CA (a host process on macOS cannot trust it); the relayer
# speaks plain http to loopback while the proxy still checks the FhevmSig over TLS. Each proxy takes its party's runtime
# env from the stack (chain id, ProtocolConfig, its tx-sender as audience) and allows SIGNER_ADDRESS, the address of the
# relayer's `auth.signer` (defaults to the e2e test key, KMS_CONNECTOR_ALLOWED_SENDER_PRIVATE_KEY in .env.test-suite).
# Idempotent.
# Usage: proxies-up.sh N [TAG]   (TAG defaults to the running endpoint's image tag: proxy and endpoint must match)
set -euo pipefail
N="${1:?number of KMS connectors}"
REPO="$(cd "$(dirname "$0")/../../.." && pwd)"
NETWORK=fhevm_default
SIGNER_ADDRESS="${SIGNER_ADDRESS:-0xAE808c0968fF9557C8E0EAbcf57747c75B490Aa0}"
ENV_DIR="$REPO/.fhevm/runtime/env"
CERTS="$REPO/test-suite/fhevm/static/config/kms-connector-proxy"

prefix() { if [ "$1" = 1 ]; then echo kms-connector; else echo "kms-connector-$1"; fi; }
env_file() { if [ "$1" = 1 ]; then echo "$ENV_DIR/kms-connector.env"; else echo "$ENV_DIR/kms-connector.$1.env"; fi; }

TAG="${2:-$(docker inspect kms-connector-endpoint --format '{{.Config.Image}}' 2>/dev/null | sed 's/.*://')}"
[ -n "$TAG" ] || { echo "no kms-connector-endpoint container: start the stack first (./fhevm-cli up)"; exit 1; }
PROXY_IMAGE="ghcr.io/zama-ai/fhevm/kms-connector/proxy:$TAG"
echo "parties=$N, proxy image $PROXY_IMAGE"

"$(dirname "$0")/proxies-down.sh" >/dev/null 2>&1 || true
for i in $(seq 1 "$N"); do
  p="$(prefix "$i")"
  [ -f "$(env_file "$i")" ] || { echo "no $(env_file "$i"): start the stack first (./fhevm-cli up)"; exit 1; }
  docker run -d --name "${p}-proxy" --network "$NETWORK" --platform linux/amd64 \
    --env-file "$(env_file "$i")" \
    -e KMS_CONNECTOR_ALLOWED_SENDERS="$SIGNER_ADDRESS" \
    -e KMS_CONNECTOR_ENDPOINT_ADDRESSES="${p}-endpoint:8080" \
    -e KMS_CONNECTOR_TLS_CONFIG__CERT_PATH=/etc/kms-connector/proxy/tls.crt \
    -e KMS_CONNECTOR_TLS_CONFIG__KEY_PATH=/etc/kms-connector/proxy/tls.key \
    -e OTEL_EXPORTER_OTLP_ENDPOINT=http://jaeger:4317 \
    -v "$CERTS:/etc/kms-connector/proxy:ro" \
    "$PROXY_IMAGE" >/dev/null
  docker run -d --name "relayer-fwd-$i" --network "$NETWORK" -p "808$i:8443" \
    alpine/socat TCP-LISTEN:8443,fork,reuseaddr "OPENSSL:${p}-proxy:8443,verify=0" >/dev/null
  echo "  party $i: 127.0.0.1:808$i -> ${p}-proxy:8443 -> ${p}-endpoint:8080"
done
sleep 3
exec "$(dirname "$0")/proxies-status.sh" "$N"
