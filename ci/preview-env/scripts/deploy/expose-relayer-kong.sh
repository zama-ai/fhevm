#!/usr/bin/env bash
# Serve both relayers of this preview on one tailnet URL, through the shared
# tailnet-only Kong for preview envs (IngressClass kong-preview, installed by
# zws-gitops environments/zws-dev-system/kong/kong-preview.yaml):
#   https://kong-proxy-preview-zws-dev.<tailnet>/<namespace>/v2|v3/... -> relayer:3000
#   https://kong-proxy-preview-zws-dev.<tailnet>/<namespace>/v4/...    -> relayer-http:3002
# Same split as the local stack's relayer-proxy
# (test-suite/fhevm/static/config/relayer-proxy/nginx.conf). Point the SDK
# relayerUrl at the base URL. Deleting the namespace deletes the routes.
# Run after relayer and relayer-http are installed.
# Env: NAMESPACE. Optional TAILSCALE_HOSTNAME (only its tailnet suffix is used).
set -euo pipefail

: "${NAMESPACE:?}"
# tailscale-operator-zws-dev.diplodocus-boa.ts.net -> diplodocus-boa.ts.net
tailnet="${TAILSCALE_HOSTNAME:-}"
tailnet="${tailnet#*.}"
tailnet="${tailnet:-diplodocus-boa.ts.net}"
url="https://kong-proxy-preview-zws-dev.${tailnet}/${NAMESPACE}"

if ! kubectl get ingressclass kong-preview >/dev/null 2>&1; then
  echo "::warning::IngressClass kong-preview not found (zws-gitops kong-preview app not installed): no unified relayer URL"
  exit 0
fi

# /<ns>/v4/x is stripped to /x, so relayer-http needs a Service that adds /v4
# back. The Helm-managed Service cannot carry the annotation (common chart),
# so this one copies its selector and port. 120s read timeout: /v4 decrypts
# are synchronous (same as the local relayer-proxy).
kubectl get service relayer-http -n "${NAMESPACE}" -o json | jq '{
  apiVersion: "v1",
  kind: "Service",
  metadata: {
    name: "relayer-http-v4",
    labels: {"app.kubernetes.io/component": "kong-preview-route"},
    annotations: {
      "konghq.com/path": "/v4",
      "konghq.com/read-timeout": "120000"
    }
  },
  spec: {
    selector: .spec.selector,
    ports: [.spec.ports[] | select(.port == 3002) | {name, port, protocol, targetPort}]
  }
}' | kubectl apply -n "${NAMESPACE}" -f -

# Longest prefix wins: /<ns>/v4/... goes to relayer-http, the rest of /<ns>/...
# to the relayer. No host: kong-preview has no public entry point.
kubectl apply -n "${NAMESPACE}" -f - <<EOF
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: relayer-kong
  labels:
    app.kubernetes.io/component: kong-preview-route
  annotations:
    konghq.com/strip-path: "true"
spec:
  ingressClassName: kong-preview
  rules:
    - http:
        paths:
          - path: /${NAMESPACE}
            pathType: Prefix
            backend:
              service:
                name: relayer
                port:
                  number: 3000
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: relayer-http-kong
  labels:
    app.kubernetes.io/component: kong-preview-route
  annotations:
    konghq.com/strip-path: "true"
spec:
  ingressClassName: kong-preview
  rules:
    - http:
        paths:
          - path: /${NAMESPACE}/v4
            pathType: Prefix
            backend:
              service:
                name: relayer-http-v4
                port:
                  number: 3002
EOF

# Route proof from the runner: keyurl answers 200, and a GET on the POST-only
# /v4/user-decrypt answers 405 from relayer-http (a Kong miss would be 404).
# Warning only: the runner's tailnet ACL to the preview Kong is not guaranteed,
# and a shared-proxy hiccup must not fail the deploy.
echo "Waiting for ${url} to route..."
keyurl="" v4=""
for _ in $(seq 1 12); do
  keyurl=$(curl -s -o /dev/null -w '%{http_code}' --max-time 5 "${url}/v2/keyurl" || true)
  v4=$(curl -s -o /dev/null -w '%{http_code}' --max-time 5 "${url}/v4/user-decrypt" || true)
  if [[ "${keyurl}" == 200 && "${v4}" == 405 ]]; then
    break
  fi
  sleep 5
done
if [[ "${keyurl}" == 200 && "${v4}" == 405 ]]; then
  echo "Unified relayer URL routes /v2 and /v4"
else
  echo "::warning::${url} not verified from the runner (GET /v2/keyurl: ${keyurl}, GET /v4/user-decrypt: ${v4}; expected 200 and 405)"
fi

echo "Unified relayer URL: ${url}"
if [[ -n "${GITHUB_ENV:-}" ]]; then
  echo "RELAYER_UNIFIED_URL=${url}" >> "${GITHUB_ENV}"
fi
