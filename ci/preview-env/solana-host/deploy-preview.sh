#!/usr/bin/env bash
# Extend an already bootstrapped preview through its existing Helm releases.
set -euo pipefail
umask 077
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=ci/preview-env/scripts/lib.sh
source "${script_dir}/../scripts/lib.sh"
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
values=ci/preview-env/solana-host
tag=$(jq -er .solana_programs <<< "$TAGS_JSON")
# The canonical EVM host chain (per-namespace Anvil) whose key material the Solana chain shares.
key_source_chain_id=12345

# Credentials are provisioned by the existing secret sync outside the throwaway namespace.
for name in solana-rpc solana-deployer solana-proof-api; do
  kubectl get secret "$name" -n "$SOLANA_SECRETS_NAMESPACE" -o json |
    jq --arg ns "$NAMESPACE" '{apiVersion,kind,type,data,metadata:{name:.metadata.name,namespace:$ns}}' |
    kubectl apply -f - >/dev/null
done

# Keygen completion precedes asynchronous key download into each coprocessor DB.
for i in $(seq 1 "$NB_COPROCESSOR"); do
  ready=false
  for ((attempt=0; attempt<120; attempt++)); do
    if [[ $(kubectl exec -n "$NAMESPACE" "postgres-coprocessor-$i-0" -- psql -U zama -d fhevm_e2e -Atc "SELECT EXISTS(SELECT 1 FROM keys WHERE chain_id=$key_source_chain_id)") == t ]]; then
      ready=true; break
    fi
    sleep 5
  done
  [[ "$ready" == true ]] || { echo "::error::Canonical keys missing in coprocessor $i"; exit 1; }
done

# HostConfig thresholds follow the same formulas as the Gateway registration (lib.sh).
cp "$values/values-solana-programs-e2e.yaml" "$work/host.yaml"
KMS_T=$(kms_t "$NB_KMS_CORE") COPRO_T=$(coproc_threshold "$NB_COPROCESSOR") yq -i '.scDeploy.env += [
  {"name": "KMS_THRESHOLD", "value": strenv(KMS_T)},
  {"name": "COPROCESSOR_THRESHOLD", "value": strenv(COPRO_T)}]' "$work/host.yaml"
helm upgrade --install solana-host "$CONTRACTS_CHART" -n "$NAMESPACE" -f "$work/host.yaml" \
  --set-string "scDeploy.image.tag=$tag" \
  --wait --wait-for-jobs --timeout=20m
helm upgrade --install gateway-add-host-chains-solana "$CONTRACTS_CHART" -n "$NAMESPACE" \
  -f "$values/values-gateway-add-host-chains-solana-e2e.yaml" \
  --set-string "scDeploy.image.tag=$(jq -r .gateway_contracts <<< "$TAGS_JSON")" --wait --wait-for-jobs --timeout=10m

for i in $(seq 1 "$NB_COPROCESSOR"); do
  cp "$values/values-solana-register-coprocessor-e2e.yaml" "$work/register.yaml"
  DATABASE_URL="postgresql://zama:zama@postgres-coprocessor-$i:5432/fhevm_e2e" yq -i '.scDeploy.env += [
    {"name": "DATABASE_URL", "value": strenv(DATABASE_URL)}]' "$work/register.yaml"
  helm upgrade --install "solana-register-coprocessor-$i" "$CONTRACTS_CHART" -n "$NAMESPACE" \
    -f "$work/register.yaml" \
    --set-string "scDeploy.configmap.name=solana-coprocessor-registration-$i" \
    --set-string "scDeploy.image.tag=$tag" --wait --wait-for-jobs --timeout=10m
  # Carry forward the original party-specific DB, wallets, S3 and tracing settings.
  helm get values "coprocessor-$i" -n "$NAMESPACE" -o yaml > "$work/coprocessor.yaml"
  helm upgrade "coprocessor-$i" "$COPROCESSOR_CHART" -n "$NAMESPACE" \
    -f "$work/coprocessor.yaml" -f "$values/values-solana-coprocessor-e2e.yaml" \
    --set-string "solanaHostListener.image.tag=$(jq -r .coprocessor_host_listener <<< "$TAGS_JSON")" \
    --set-string "solanaHostListener.serviceAccountName=coprocessor-$i" --wait --wait-for-jobs --timeout=10m
  # zkproof loads host_chains only at startup.
  kubectl rollout restart "deployment/coprocessor-$i-zkproof-worker" -n "$NAMESPACE"
  kubectl rollout status "deployment/coprocessor-$i-zkproof-worker" -n "$NAMESPACE" --timeout=10m
done

# hostChains is a list, so append the Solana entry to each connector's existing entries.
endpoints=$(seq 1 "$NB_COPROCESSOR" | jq -Rsc 'split("\n")[:-1] | map("http://coprocessor-" + . + "-solana-host-listener:8080")')
ENDPOINTS="$endpoints" yq '.[0].solanaProofEndpoints = (strenv(ENDPOINTS) | from_json)' \
  "$values/connector-host-chain.yaml" > "$work/connector-host-chain.yaml"
for i in $(seq 1 "$NB_KMS_CORE"); do
  helm get values "kms-connector-$i" -n "$NAMESPACE" -o yaml > "$work/connector.yaml"
  SOLANA_CHAIN="$work/connector-host-chain.yaml" yq -i '.kmsConnectorKmsWorker.config.hostChains = ((.kmsConnectorKmsWorker.config.hostChains // [])
    | map(select(.chainKind != "solana"))) + load(strenv(SOLANA_CHAIN))' "$work/connector.yaml"
  helm upgrade "kms-connector-$i" "$KMS_CONNECTOR_CHART" -n "$NAMESPACE" \
    -f "$work/connector.yaml" -f "$values/values-solana-connector-e2e.yaml" \
    --wait --wait-for-jobs --timeout=10m
done
# Relayer host dispatch also needs the Solana RPC/program identity; preserve its EVM entry.
helm get values relayer -n "$NAMESPACE" -o yaml > "$work/relayer.yaml"
yq -i '.env = ((.env // []) | map(select(.name != "APP_HOST_CHAINS__1__CHAIN_ID" and .name != "APP_HOST_CHAINS__1__URL" and .name != "APP_HOST_CHAINS__1__ACL_ADDRESS"))) + [
  {"name":"APP_HOST_CHAINS__1__CHAIN_ID","value":"9223372036854788153"},
  {"name":"APP_HOST_CHAINS__1__URL","valueFrom":{"secretKeyRef":{"name":"solana-rpc","key":"rpc-url"}}},
  {"name":"APP_HOST_CHAINS__1__ACL_ADDRESS","valueFrom":{"configMapKeyRef":{"name":"solana-host-addresses","key":"zama_host.address"}}}
]' "$work/relayer.yaml"
helm upgrade relayer "$COMMON_CHART" --version "$COMMON_CHART_VERSION" -n "$NAMESPACE" -f "$work/relayer.yaml" \
  --set-string "image.tag=$(jq -r .relayer <<< "$TAGS_JSON")" --wait --wait-for-jobs --timeout=10m

# HTTP readiness alone does not prove that Yellowstone is delivering sealed blocks.
for i in $(seq 1 "$NB_COPROCESSOR"); do
  initial=$(kubectl exec -n "$NAMESPACE" "postgres-coprocessor-$i-0" -- psql -U zama -d fhevm_e2e -Atc 'SELECT COALESCE(MAX(slot),0) FROM solana_listener_checkpoint')
  progressed=false
  for ((attempt=0; attempt<60; attempt++)); do
    current=$(kubectl exec -n "$NAMESPACE" "postgres-coprocessor-$i-0" -- psql -U zama -d fhevm_e2e -Atc 'SELECT COALESCE(MAX(slot),0) FROM solana_listener_checkpoint')
    if (( current > initial )); then progressed=true; break; fi
    sleep 5
  done
  [[ "$progressed" == true ]] || { echo "::error::Solana listener $i is ready but its sealed-block checkpoint is not advancing; check Yellowstone endpoint, credentials, connectivity and provider block delivery"; exit 1; }
done
if [[ "$SOLANA_DEPLOY_EXAMPLE_PROGRAMS" == true ]]; then
  helm upgrade --install solana-demos "$CONTRACTS_CHART" -n "$NAMESPACE" \
    -f "$values/values-solana-demos-e2e.yaml" --set-string "scDeploy.image.tag=$tag" \
    --wait --wait-for-jobs --timeout=20m
fi
{
  echo '### Solana rollout'
  echo "Programs: \`$tag\`; namespace: \`$NAMESPACE\`."
  echo 'Listeners are ready. Run the confidential-operation acceptance scenario before sending experiment traffic.'
} >> "$GITHUB_STEP_SUMMARY"
