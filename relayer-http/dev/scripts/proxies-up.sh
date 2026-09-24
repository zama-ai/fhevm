#!/usr/bin/env bash
# Starts, on the compose network of the local stack, one KMS connector proxy per party (the stack does not ship
# them) and one socat forwarder per proxy, published on 127.0.0.1:8081..808N, for a relayer-http run on the host.
# The forwarder terminates the proxy's self-signed TLS (a host process on macOS cannot trust it); the relayer
# speaks plain http to loopback while the proxy still checks the API key over TLS. Idempotent.
# Usage: proxies-up.sh N [TAG]   (TAG defaults to the running endpoint's image tag: proxy and endpoint must match)
set -euo pipefail
N="${1:?number of KMS connectors}"
REPO="$(cd "$(dirname "$0")/../../.." && pwd)"
NETWORK=fhevm_default
API_KEY="${API_KEY:-fhevm-e2e-kms-connector-api-key}"
# sha256 of API_KEY (the e2e test key, see test-suite/fhevm/templates/env/.env.kms-connector)
API_KEY_DIGEST=0x75e65d843ac0b8656e1d75b2efc1cc7879e4b6c1854651cb28664686625a6e23
CERTS="$REPO/test-suite/fhevm/static/config/kms-connector-proxy"

prefix() { if [ "$1" = 1 ]; then echo kms-connector; else echo "kms-connector-$1"; fi; }

TAG="${2:-$(docker inspect kms-connector-endpoint --format '{{.Config.Image}}' 2>/dev/null | sed 's/.*://')}"
[ -n "$TAG" ] || { echo "no kms-connector-endpoint container: start the stack first (./fhevm-cli up)"; exit 1; }
PROXY_IMAGE="ghcr.io/zama-ai/fhevm/kms-connector/proxy:$TAG"
echo "parties=$N, proxy image $PROXY_IMAGE"

"$(dirname "$0")/proxies-down.sh" >/dev/null 2>&1 || true
for i in $(seq 1 "$N"); do
  p="$(prefix "$i")"
  docker run -d --name "${p}-proxy" --network "$NETWORK" --platform linux/amd64 \
    -e KMS_CONNECTOR_API_KEY_DIGEST="$API_KEY_DIGEST" \
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
