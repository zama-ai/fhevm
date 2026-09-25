#!/usr/bin/env bash
# Extend an already bootstrapped preview through its existing Helm releases.
set +x
set -euo pipefail
umask 077
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=ci/preview-env/scripts/lib.sh
source "${script_dir}/../scripts/lib.sh"
# shellcheck source=ci/preview-env/solana-host/ownership.sh
source "$script_dir/ownership.sh"
solana_acquire
work=$(mktemp -d)
trap 'rm -rf "$work"; solana_release_operation' EXIT
values=ci/preview-env/solana-host
tag=$(jq -er .solana_programs <<< "$TAGS_JSON")
# The canonical EVM host chain (per-namespace Anvil) whose key material the Solana chain shares.
key_source_chain_id=12345

# RPC credentials and keypairs come from AWS Secrets Manager into this namespace, the same
# way chain_mode=testnets gets its RPC URLs; they disappear with the namespace.
for name in solana-rpc solana-deployer; do
  helm upgrade --install "$name" "$SYNC_SECRETS_CHART" --version "$SYNC_SECRETS_CHART_VERSION" \
    -n "$NAMESPACE" -f "$values/values-$name.yaml"
  wait_external_secret "$name"
done
# The leaf-proof bearer token is only ever read inside this namespace, by the listeners and
# the connectors, so each preview mints its own.
openssl rand -base64 32 | tr -d '\n' > "$work/proof-api-key"
kubectl create secret generic solana-proof-api -n "$NAMESPACE" \
  --from-file=api-key="$work/proof-api-key" --dry-run=client -o yaml | kubectl apply -f - >/dev/null

# Old public actors are imported only for recovery, never for new funding.
if [[ -z $(kubectl get secret solana-recovery -n "$NAMESPACE" --ignore-not-found -o name) ]]; then
  kubectl create secret generic solana-recovery -n "$NAMESPACE" \
    --from-file=demo-legacy-keeper.json=solana/scripts/demo/demo-keypairs/keeper.json \
    --from-file=demo-legacy-alice.json=solana/scripts/demo/demo-keypairs/alice.json \
    --from-file=demo-legacy-bob.json=solana/scripts/demo/demo-keypairs/bob.json \
    --from-file=demo-legacy-mintAuthority.json=solana/scripts/demo/demo-keypairs/mint-authority.json >/dev/null
fi
SOLANA_RECOVERY_IMAGE="hub.zama.org/ghcr/zama-ai/fhevm/solana-programs:$tag" \
  bash "$script_dir/recover.sh" reset


# Keygen completion precedes asynchronous key download into each coprocessor DB.
for i in $(seq 1 "$NB_COPROCESSOR"); do
  ready=false
  for ((attempt=0; attempt<120; attempt++)); do
    if [[ $(psql_party "$i" "SELECT EXISTS(SELECT 1 FROM keys WHERE chain_id=$key_source_chain_id)") == t ]]; then
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

endpoints=$(seq 1 "$NB_COPROCESSOR" | jq -Rsc 'split("\n")[:-1] | map("http://coprocessor-" + . + "-solana-host-listener:8080")')
for i in $(seq 1 "$NB_KMS_CORE"); do
  helm get values "kms-connector-$i" -n "$NAMESPACE" -o yaml > "$work/connector.yaml"
  helm upgrade "kms-connector-$i" "$KMS_CONNECTOR_CHART" -n "$NAMESPACE" \
    -f "$work/connector.yaml" -f "$values/values-solana-connector-e2e.yaml" \
    --set-json "commonConfig.hostChains.solana.solanaProofEndpoints=$endpoints" \
    --wait --wait-for-jobs --timeout=10m
done
# Relayer host dispatch also needs the Solana RPC/program identity; preserve its EVM entry.
helm get values relayer -n "$NAMESPACE" -o yaml > "$work/relayer.yaml"
yq -i '.env = ((.env // []) | map(select(.name != "APP_HOST_CHAINS__1__CHAIN_ID" and .name != "APP_HOST_CHAINS__1__URL" and .name != "APP_HOST_CHAINS__1__ACL_ADDRESS"))) + [
  {"name":"APP_HOST_CHAINS__1__CHAIN_ID","value":"130140237723663404"},
  {"name":"APP_HOST_CHAINS__1__URL","valueFrom":{"secretKeyRef":{"name":"solana-rpc","key":"rpc-url"}}},
  {"name":"APP_HOST_CHAINS__1__ACL_ADDRESS","valueFrom":{"configMapKeyRef":{"name":"solana-host-addresses","key":"zama_host.address"}}}
]' "$work/relayer.yaml"
helm upgrade relayer "$COMMON_CHART" --version "$COMMON_CHART_VERSION" -n "$NAMESPACE" -f "$work/relayer.yaml" \
  --set-string "image.tag=$(jq -r .relayer <<< "$TAGS_JSON")" --wait --wait-for-jobs --timeout=10m

# HTTP readiness alone does not prove that Yellowstone is delivering sealed blocks.
for i in $(seq 1 "$NB_COPROCESSOR"); do
  initial=$(psql_party "$i" 'SELECT COALESCE(MAX(slot),0) FROM solana_listener_checkpoint')
  progressed=false
  for ((attempt=0; attempt<60; attempt++)); do
    current=$(psql_party "$i" 'SELECT COALESCE(MAX(slot),0) FROM solana_listener_checkpoint')
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
