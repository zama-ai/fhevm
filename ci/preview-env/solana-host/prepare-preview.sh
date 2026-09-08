#!/usr/bin/env bash
set -euo pipefail
if [[ "${SOLANA_ACTION:-off}" == off ]]; then
  if kubectl get configmap preview-env-bootstrap-state -n "$NAMESPACE" >/dev/null 2>&1; then
    echo '::error::This is a persistent Solana preview; use a Solana rollout or the explicit destroy workflow.'; exit 1
  fi
  if kubectl get namespace "$NAMESPACE" >/dev/null 2>&1; then
    helm list -n "$NAMESPACE" -q | xargs -r helm uninstall -n "$NAMESPACE" --wait
    kubectl delete namespace "$NAMESPACE" --timeout=180s
  fi
  kubectl create namespace "$NAMESPACE"
  echo PREVIEW_BOOTSTRAP=true >> "$GITHUB_ENV"
  exit 0
fi
[[ -n "${SOLANA_SECRETS_NAMESPACE:-}" ]] || { echo '::error::Set solana_secrets_namespace to the preview vault sync namespace'; exit 1; }
for name in solana-rpc solana-deployer solana-proof-api solana-deployment-lock; do
  kubectl get secret "$name" -n "$SOLANA_SECRETS_NAMESPACE" >/dev/null
done
# Bind retained state to the actual chain, including providers that reuse an RPC URL.
# Read credentials on stdin and report only a generic error if the request fails.
genesis=$(kubectl get secret solana-rpc -n "$SOLANA_SECRETS_NAMESPACE" -o json | node -e '
  let input = "";
  process.stdin.on("data", chunk => input += chunk);
  process.stdin.on("end", async () => {
    try {
      const url = Buffer.from(JSON.parse(input).data["rpc-url"], "base64").toString();
      const response = await fetch(url, {
        method: "POST", headers: {"content-type": "application/json"},
        body: JSON.stringify({jsonrpc: "2.0", id: 1, method: "getGenesisHash"}),
        signal: AbortSignal.timeout(10000)
      });
      const body = await response.json();
      if (!response.ok || !/^[1-9A-HJ-NP-Za-km-z]{32,44}$/.test(body.result)) throw new Error();
      process.stdout.write(body.result);
    } catch {
      console.error("::error::Cannot verify the Solana RPC genesis hash");
      process.exitCode = 1;
    }
  });
')
# Image revisions can change; topology and the KMS identities cannot be silently replaced.
fingerprint=$({
  printf '%s\n' "$NB_COPROCESSOR" "$NB_KMS_CORE" "$KMS_REPO_REF" "$KMS_CORE_TAG" "$SOLANA_SECRETS_NAMESPACE" "$genesis"
  kubectl get secret solana-rpc solana-deployer solana-proof-api solana-deployment-lock -n "$SOLANA_SECRETS_NAMESPACE" -o json | jq -Sc '[.items[] | {name:.metadata.name,data:(if .metadata.name == "solana-deployer" then {"deployer.json":.data["deployer.json"]} else .data end)}] | sort_by(.name)'
} | sha256sum | cut -d' ' -f1)
echo "SOLANA_PREVIEW_FINGERPRINT=$fingerprint" >> "$GITHUB_ENV"
if ! kubectl get namespace "$NAMESPACE" >/dev/null 2>&1; then
  [[ "$SOLANA_ACTION" == deploy ]] || { echo '::error::Upgrade requires an existing preview'; exit 1; }
  kubectl create namespace "$NAMESPACE"
  echo PREVIEW_BOOTSTRAP=true >> "$GITHUB_ENV"
else
  recorded=$(kubectl get configmap preview-env-bootstrap-state -n "$NAMESPACE" -o jsonpath='{.data.fingerprint}' --ignore-not-found)
  [[ "$recorded" == "$fingerprint" ]] || {
    echo '::error::Preview has no completed persistent bootstrap or its topology differs. Inspect it and explicitly destroy/reset it before a fresh experiment.'; exit 1;
  }
  echo PREVIEW_BOOTSTRAP=false >> "$GITHUB_ENV"
fi

# Copy only the explicitly selected preview credentials. Do not print Secret data.
for name in solana-rpc solana-deployer solana-proof-api solana-deployment-lock; do
  kubectl get secret "$name" -n "$SOLANA_SECRETS_NAMESPACE" -o json |
    jq --arg ns "$NAMESPACE" '{apiVersion,kind,type,data,metadata:{name:.metadata.name,namespace:$ns}}' |
    kubectl apply -f - >/dev/null
done
