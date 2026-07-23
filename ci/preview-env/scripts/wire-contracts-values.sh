#!/usr/bin/env bash
# Append the real per-party KMS + coprocessor on-chain wiring to .scDeploy.env
# of a contracts values file and emit its path as the `values_file` output.
#
# One script for both targets, but the emitted env-var names stay
# target-specific ON PURPOSE - the gateway and host charts read different names
# (KMS_NODE_IP_ADDRESS vs KMS_NODE_IP, KMS_GENERATION_THRESHOLD vs
# KMS_GEN_THRESHOLD) and host takes extra node fields + only the coprocessor
# SIGNER (no tx-sender/S3 URL). Do NOT unify them.
#
# Env: TARGET (gateway|host), NB_KMS_CORE, NB_COPROCESSOR, NAMESPACE,
# SIGNERS_JSON, WALLETS_JSON, COPROC_WALLETS_JSON, S3_ENDPOINT.
set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
# shellcheck source-path=SCRIPTDIR
# shellcheck source=lib.sh
source "${script_dir}/lib.sh"

: "${TARGET:?TARGET is required (gateway|host)}"
case "${TARGET}" in
  gateway)
    src=ci/preview-env/gateway-chain/values-gateway-contracts-e2e.yaml
    generated=/tmp/values-gateway-contracts-e2e.generated.yaml
    kms_gen_threshold_key=KMS_GENERATION_THRESHOLD
    ;;
  host)
    src=ci/preview-env/host-chain/values-host-contracts-e2e.yaml
    generated=/tmp/values-host-contracts-e2e.generated.yaml
    kms_gen_threshold_key=KMS_GEN_THRESHOLD
    ;;
  *)
    echo "::error::unknown TARGET '${TARGET}' (expected gateway|host)"; exit 1;;
esac

n="${NB_KMS_CORE}"
t=$(kms_t "${n}")
reconstruct=$(kms_reconstruct "${n}")

cp "${src}" "${generated}"

# Register the coprocessors on-chain. Gateway needs each party's tx-sender,
# signer and S3 bucket URL (bucket name from ConfigMap coprocessor-<i>; bare
# root, region eu-west-1, since sns-worker keys ct128 by bare hex digest); host
# needs only the signer (its InputVerifier checks input-proof signatures).
nc="${NB_COPROCESSOR}"
coproc_thr=$(coproc_threshold "${nc}")
yq -i "
  .scDeploy.env += [
    {\"name\": \"NUM_COPROCESSORS\", \"value\": \"${nc}\"},
    {\"name\": \"COPROCESSOR_THRESHOLD\", \"value\": \"${coproc_thr}\"}
  ]
" "${generated}"
for i in $(seq 1 "${nc}"); do
  idx=$((i - 1))
  if [[ "${TARGET}" == gateway ]]; then
    bucket=$(kubectl get configmap "coprocessor-${i}" -n "${NAMESPACE}" -o jsonpath='{.data.S3_BUCKET_NAME}')
    if [[ -z "${bucket}" ]]; then
      echo "::error::coprocessor-${i} ConfigMap has no S3_BUCKET_NAME key - is coprocessor-infra-${i} reconciled? (kubectl get configmap coprocessor-${i} -n ${NAMESPACE})"
      exit 1
    fi
    coproc_addr=$(jq -r --argjson party "${i}" '.[] | select(.party == $party) | .address' <<<"${COPROC_WALLETS_JSON}")
    require_nonempty "${coproc_addr}" "no derived coprocessor wallet for party ${i} in coproc_wallets_json=${COPROC_WALLETS_JSON}"
    coproc_s3_url="https://${bucket}.s3.eu-west-1.amazonaws.com"
    # tx-sender and signer are the same address per party.
    yq -i "
      .scDeploy.env += [
        {\"name\": \"COPROCESSOR_TX_SENDER_ADDRESS_${idx}\", \"value\": \"${coproc_addr}\"},
        {\"name\": \"COPROCESSOR_SIGNER_ADDRESS_${idx}\", \"value\": \"${coproc_addr}\"},
        {\"name\": \"COPROCESSOR_S3_BUCKET_URL_${idx}\", \"value\": \"${coproc_s3_url}\"}
      ]
    " "${generated}"
    echo "Registered coprocessor party ${i}: tx-sender/signer=${coproc_addr}, S3=${coproc_s3_url}"
  else
    coproc_addr=$(jq -r --argjson party "${i}" '.[] | select(.party == $party) | .address' <<<"${COPROC_WALLETS_JSON}")
    require_nonempty "${coproc_addr}" "no derived coprocessor wallet for party ${i} in coproc_wallets_json=${COPROC_WALLETS_JSON}"
    yq -i "
      .scDeploy.env += [
        {\"name\": \"COPROCESSOR_SIGNER_ADDRESS_${idx}\", \"value\": \"${coproc_addr}\"}
      ]
    " "${generated}"
    echo "Registered host-side coprocessor party ${i} signer=${coproc_addr}"
  fi
done

# Key/CRS *generation* consensus threshold: both reference configs
# (gateway-contracts/.env.example, charts/contracts/values.yaml) set it equal to
# the decryption threshold (reconstruct = n-t), NOT the MPC majority (t+1). Using
# t+1 would let key/CRS activation fire after fewer node agreements than
# production and silently weaken keygen e2e coverage.
yq -i "
  .scDeploy.env += [
    {\"name\": \"NUM_KMS_NODES\", \"value\": \"${n}\"},
    {\"name\": \"PUBLIC_DECRYPTION_THRESHOLD\", \"value\": \"${reconstruct}\"},
    {\"name\": \"USER_DECRYPTION_THRESHOLD\", \"value\": \"${reconstruct}\"},
    {\"name\": \"${kms_gen_threshold_key}\", \"value\": \"${reconstruct}\"},
    {\"name\": \"MPC_THRESHOLD\", \"value\": \"${t}\"}
  ]
" "${generated}"

# KMS_NODE_STORAGE_URL_x is the bare bucket root (no "/PUB-p<i>"): host-listener's
# S3 client only handles a bare URL, and all parties share one bucket.
for i in $(seq 1 "${n}"); do
  idx=$((i - 1))
  signer=$(jq -r --argjson party "${i}" '.[] | select(.party == $party) | .signer' <<<"${SIGNERS_JSON}")
  tx_sender=$(jq -r --argjson party "${i}" '.[] | select(.party == $party) | .address' <<<"${WALLETS_JSON}")
  require_nonempty "${signer}" "no discovered signer for party ${i} in signers_json=${SIGNERS_JSON}"
  require_nonempty "${tx_sender}" "no derived tx-sender wallet for party ${i} in wallets_json=${WALLETS_JSON}"
  if [[ "${TARGET}" == gateway ]]; then
    yq -i "
      .scDeploy.env += [
        {\"name\": \"KMS_TX_SENDER_ADDRESS_${idx}\", \"value\": \"${tx_sender}\"},
        {\"name\": \"KMS_SIGNER_ADDRESS_${idx}\", \"value\": \"${signer}\"},
        {\"name\": \"KMS_NODE_IP_ADDRESS_${idx}\", \"value\": \"kms-core-${i}-core-${i}\"},
        {\"name\": \"KMS_NODE_STORAGE_URL_${idx}\", \"value\": \"${S3_ENDPOINT}\"}
      ]
    " "${generated}"
  else
    yq -i "
      .scDeploy.env += [
        {\"name\": \"KMS_TX_SENDER_ADDRESS_${idx}\", \"value\": \"${tx_sender}\"},
        {\"name\": \"KMS_SIGNER_ADDRESS_${idx}\", \"value\": \"${signer}\"},
        {\"name\": \"KMS_NODE_IP_${idx}\", \"value\": \"kms-core-${i}-core-${i}\"},
        {\"name\": \"KMS_NODE_STORAGE_URL_${idx}\", \"value\": \"${S3_ENDPOINT}\"},
        {\"name\": \"KMS_NODE_PARTY_ID_${idx}\", \"value\": \"${i}\"},
        {\"name\": \"KMS_NODE_MPC_IDENTITY_${idx}\", \"value\": \"kms-core-${i}\"},
        {\"name\": \"KMS_NODE_CA_CERT_${idx}\", \"value\": \"0x706c616365686f6c6465722d63612d636572742d6532652d7370696b65\"},
        {\"name\": \"KMS_NODE_STORAGE_PREFIX_${idx}\", \"value\": \"PUB-p${i}\"}
      ]
    " "${generated}"
  fi
done
# Force values to double-quoted strings (a bare hex address is valid YAML int).
yq -i '(.scDeploy.env[] | select(has("value")) | .value) style="double"' "${generated}"
echo "values_file=${generated}" >> "$GITHUB_OUTPUT"
