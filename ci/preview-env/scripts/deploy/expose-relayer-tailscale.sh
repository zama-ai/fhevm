#!/usr/bin/env bash
# Publish the relayer HTTP API on the zws-dev tailnet (Tailscale IngressClass).
# Only port 3000 is a backend: metrics 9898 and enable_admin_endpoint stay
# ClusterIP-only. The hostname is relayer-<namespace> so two previews cannot
# collide. Deleting the namespace deletes the Ingress and the operator drops
# the MagicDNS name.
# Env: NAMESPACE. Optional TAILSCALE_HOSTNAME (used only as a fallback suffix).
set -euo pipefail

: "${NAMESPACE:?}"

host="relayer-${NAMESPACE}"
# tailscale-operator-zws-dev.diplodocus-boa.ts.net -> diplodocus-boa.ts.net
tailnet="${TAILSCALE_HOSTNAME#*.}"
tailnet="${tailnet:-diplodocus-boa.ts.net}"

kubectl apply -n "${NAMESPACE}" -f - <<EOF
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: relayer
  labels:
    app.kubernetes.io/name: relayer
    app.kubernetes.io/component: tailscale-ingress
  annotations:
    # Same tag gitops uses for other zws-dev Tailscale Services, so existing
    # tailnet ACLs cover preview relayers without a new grant.
    tailscale.com/tags: "tag:k8s-zws-dev"
spec:
  ingressClassName: tailscale
  tls:
    - hosts:
        - ${host}
  rules:
    - host: ${host}
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: relayer
                port:
                  number: 3000
EOF

echo "Waiting for Tailscale Ingress ${host} to get a MagicDNS hostname..."
reported=""
for _ in $(seq 1 36); do
  reported=$(kubectl get ingress relayer -n "${NAMESPACE}" \
    -o jsonpath='{.status.loadBalancer.ingress[0].hostname}' 2>/dev/null || true)
  if [[ -n "${reported}" ]]; then
    break
  fi
  sleep 5
done

if [[ -z "${reported}" ]]; then
  echo "::warning::Ingress/relayer has no hostname after 3m; publishing the expected MagicDNS name anyway"
  reported="${host}.${tailnet}"
fi

# Operator status is sometimes the short hostname, sometimes the FQDN.
if [[ "${reported}" != *.* ]]; then
  reported="${reported}.${tailnet}"
fi
url="https://${reported}"
echo "Relayer Tailscale URL: ${url}"
if [[ -n "${GITHUB_ENV:-}" ]]; then
  echo "RELAYER_TS_URL=${url}" >> "${GITHUB_ENV}"
fi
