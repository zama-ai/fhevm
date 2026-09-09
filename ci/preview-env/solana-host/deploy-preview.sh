#!/usr/bin/env bash
# Extend an already bootstrapped preview through its existing Helm releases.
set -euo pipefail
umask 077
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
values=ci/preview-env/solana-host
tag=$(jq -er .solana_programs <<< "$TAGS_JSON")

# A retained address file is not evidence that the chain survived a restart.
for pair in 'anvil-gateway:gw-sc-addresses:8546' 'anvil-host:host-sc-addresses:8545'; do
  IFS=: read -r release config port <<< "$pair"
  kubectl rollout status "statefulset/${release}-anvil-node" -n "$NAMESPACE" --timeout=5m
  addresses=$(kubectl get configmap "$config" -n "$NAMESPACE" -o json | jq -r '.data[] | select(test("^0x[0-9a-fA-F]{40}$"))' | sort -u)
  [[ -n "$addresses" ]] || { echo "::error::No recorded contracts in $config"; exit 1; }
  while read -r address; do
    code=$(kubectl exec -n "$NAMESPACE" "${release}-anvil-node-0" -- cast code "$address" --rpc-url "http://127.0.0.1:$port")
    [[ "$code" != 0x ]] || { echo "::error::Missing contract $address on $release; stop this experiment and restore or reset its entire state"; exit 1; }
  done <<< "$addresses"
done

# Keygen completion precedes asynchronous key download into each coprocessor DB.
for i in $(seq 1 "$NB_COPROCESSOR"); do
  ready=false
  for ((attempt=0; attempt<120; attempt++)); do
    if [[ $(kubectl exec -n "$NAMESPACE" "postgres-coprocessor-$i-0" -- psql -U zama -d fhevm_e2e -Atc 'SELECT EXISTS(SELECT 1 FROM keys WHERE chain_id=12345)') == t ]]; then
      ready=true; break
    fi
    sleep 5
  done
  [[ "$ready" == true ]] || { echo "::error::Canonical keys missing in coprocessor $i"; exit 1; }
done
# Record completion before any public-Solana transaction. Failed later Jobs can resume.
if [[ "$PREVIEW_BOOTSTRAP" == true ]]; then
  kubectl create configmap preview-env-bootstrap-state -n "$NAMESPACE" \
    --from-literal="fingerprint=$SOLANA_PREVIEW_FINGERPRINT" \
    --dry-run=client -o yaml | kubectl apply -f -
fi

cp "$values/values-solana-programs-e2e.yaml" "$work/host.yaml"
KMS_T=$(( (NB_KMS_CORE - 1) / 3 )) COPRO_T=1 yq -i '
  (.scDeploy.env[] | select(.name == "KMS_THRESHOLD").value) = strenv(KMS_T) |
  (.scDeploy.env[] | select(.name == "COPROCESSOR_THRESHOLD").value) = strenv(COPRO_T)' "$work/host.yaml"
helm upgrade --install solana-host "$CONTRACTS_CHART" -n "$NAMESPACE" -f "$work/host.yaml" \
  --set-string "scDeploy.image.tag=$tag" --set-string "scDeploy.deployCommands[0]=node /app/cli.mjs host $SOLANA_ACTION" \
  --wait --wait-for-jobs --timeout=20m
# Gateway registration is bootstrap-only, using the existing additive task.
gateway_config=$(kubectl get configmap gw-sc-addresses -n "$NAMESPACE" -o jsonpath='{.data.gateway_config\.address}')
registered=$(kubectl exec -n "$NAMESPACE" anvil-gateway-anvil-node-0 -- cast call "$gateway_config" 'isHostChainRegistered(uint256)(bool)' 9223372036854788153 --rpc-url http://127.0.0.1:8546)
if [[ "$registered" == false ]]; then
  helm upgrade --install gateway-add-host-chains-solana "$CONTRACTS_CHART" -n "$NAMESPACE" \
    -f "$values/values-gateway-add-host-chains-solana-e2e.yaml" \
    --set-string "scDeploy.image.tag=$(jq -r .gateway_contracts <<< "$TAGS_JSON")" --wait --wait-for-jobs --timeout=10m
fi

for i in $(seq 1 "$NB_COPROCESSOR"); do
  helm upgrade --install "solana-register-coprocessor-$i" "$CONTRACTS_CHART" -n "$NAMESPACE" \
    -f "$values/values-solana-register-coprocessor-e2e.yaml" \
    --set-string "scDeploy.configmap.name=solana-coprocessor-registration-$i" \
    --set-string "scDeploy.env[0].value=postgresql://zama:zama@postgres-coprocessor-$i:5432/fhevm_e2e" \
    --set-string "scDeploy.image.tag=$tag" --wait --wait-for-jobs --timeout=10m
  # Carry forward the original party-specific DB, wallets, S3 and tracing settings.
  helm get values "coprocessor-$i" -n "$NAMESPACE" -o yaml > "$work/coprocessor.yaml"
  images=()
  for mapping in dbMigration:db_migration gwListener:gw_listener hostListenerShared:host_listener hostListenerPollerShared:host_listener hostListenerCatchupOnlyShared:host_listener hostListenerConsumerShared:host_listener snsWorker:sns_worker tfheWorker:tfhe_worker txSender:tx_sender zkProofWorker:zkproof_worker; do
    IFS=: read -r field component <<< "$mapping"
    images+=(--set-string "$field.image.tag=$(jq -er ".coprocessor_$component" <<< "$TAGS_JSON")")
  done
  helm upgrade "coprocessor-$i" "$COPROCESSOR_CHART" -n "$NAMESPACE" \
    -f "$work/coprocessor.yaml" -f "$values/values-solana-coprocessor-e2e.yaml" "${images[@]}" \
    --set-string "solanaHostListener.image.tag=$(jq -r .coprocessor_host_listener <<< "$TAGS_JSON")" \
    --set-string "solanaHostListener.serviceAccountName=coprocessor-$i" --wait --wait-for-jobs --timeout=10m
  # zkproof loads host_chains only at startup.
  kubectl rollout restart "deployment/coprocessor-$i-zkproof-worker" -n "$NAMESPACE"
  kubectl rollout status "deployment/coprocessor-$i-zkproof-worker" -n "$NAMESPACE" --timeout=10m
  kubectl rollout status "deployment/coprocessor-$i-solana-host-listener" -n "$NAMESPACE" --timeout=10m
done

cp "$values/values-solana-connector-e2e.yaml" "$work/connector-solana.yaml"
endpoints=$(seq 1 "$NB_COPROCESSOR" | jq -Rsc 'split("\n")[:-1] | map("http://coprocessor-" + . + "-solana-host-listener:8080")')
ENDPOINTS="$endpoints" yq -i '.kmsConnectorKmsWorker.config.hostChains[1].solanaProofEndpoints = (strenv(ENDPOINTS) | from_json)' "$work/connector-solana.yaml"
for i in $(seq 1 "$NB_KMS_CORE"); do
  helm get values "kms-connector-$i" -n "$NAMESPACE" -o yaml > "$work/connector.yaml"
  images=()
  for mapping in kmsConnectorDbMigration:db_migration kmsConnectorGwListener:gw_listener kmsConnectorKmsWorker:kms_worker kmsConnectorTxSender:tx_sender; do
    IFS=: read -r field component <<< "$mapping"
    images+=(--set-string "$field.image.tag=$(jq -er ".kms_connector_$component" <<< "$TAGS_JSON")")
  done
  helm upgrade "kms-connector-$i" "$KMS_CONNECTOR_CHART" -n "$NAMESPACE" \
    -f "$work/connector.yaml" -f "$work/connector-solana.yaml" "${images[@]}" \
    --wait --wait-for-jobs --timeout=10m
done
# The initial bootstrap already migrated the relayer; subsequent images may add migrations.
if [[ "$PREVIEW_BOOTSTRAP" == false ]]; then
  helm upgrade relayer-migrate "$COMMON_CHART" --version "$COMMON_CHART_VERSION" -n "$NAMESPACE" \
    -f ci/preview-env/relayer/values-relayer-migrate-e2e.yaml \
    --set-string "fullnameOverride=relayer-migrate-$GITHUB_RUN_ID-$GITHUB_RUN_ATTEMPT" \
    --set-string "image.tag=$(jq -er .relayer_migrate <<< "$TAGS_JSON")" --wait --wait-for-jobs --timeout=5m
fi
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
if [[ "$SOLANA_DEMOS" == true ]]; then
  helm upgrade --install solana-demos "$CONTRACTS_CHART" -n "$NAMESPACE" \
    -f "$values/values-solana-demos-e2e.yaml" --set-string "scDeploy.image.tag=$tag" \
    --set-string "scDeploy.deployCommands[0]=node /app/cli.mjs demos $SOLANA_ACTION" \
    --wait --wait-for-jobs --timeout=20m
fi
{
  echo '### Solana rollout'
  echo "Programs: \`$tag\`; action: \`$SOLANA_ACTION\`; retained namespace: \`$NAMESPACE\`."
  echo 'Listeners are ready. Run the confidential-operation acceptance scenario before sending experiment traffic.'
} >> "$GITHUB_STEP_SUMMARY"
