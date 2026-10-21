#!/usr/bin/env bash
# Same-context epoch rotation on an already deployed preview namespace.
# Broadcasts defineNewEpochForCurrentKmsContext with the live host-contracts
# network, RPC, and deployer, then waits on that host chain until every signer
# has emitted EpochActivationConfirmation, ActivateEpoch is in, and the epoch
# id has moved. The context id must stay the same. Core log lines such as
# "Still waiting to receive from party" are not a signal.
#
# After the chain moves, runs the preview test-suite's user-input decrypt
# (user decrypt and public decrypt) inside the idle test-suite pod.
# The Helm release stays installed with its Job Complete. The next run
# uninstalls it before broadcasting again.
#
# Env: NAMESPACE (required). CONTRACTS_CHART (default charts/contracts).
#      EPOCH_TIMEOUT (helm --timeout, default 20m).
#      EPOCH_WATCH_TIMEOUT (chain watch, default 45m). EPOCH_POLL_SECS (default 30).
#      DECRYPT_CMD (optional; default is kubectl exec ./run-tests.sh).
# Run from a checkout of this repo. Not part of preview-env-deploy.
set -euo pipefail

: "${NAMESPACE:?NAMESPACE is required}"

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
root=$(cd "${script_dir}/../../../.." && pwd)
cd "${root}"

chart="${CONTRACTS_CHART:-charts/contracts}"
timeout="${EPOCH_TIMEOUT:-20m}"
watch_timeout="${EPOCH_WATCH_TIMEOUT:-45m}"
poll_secs="${EPOCH_POLL_SECS:-30}"
overlay=ci/preview-env/host-chain/values-host-define-new-epoch-e2e.yaml
release=host-define-new-epoch
local_port="${EPOCH_LOCAL_PORT:-18545}"

topic_confirm="0x7eda6f85e23b7b91c019b0570d02b663606ef9d74594f7e01fcfbdb0f4e954d5"
topic_activate="0x1a547b42e72cd3dda04e6adccd2200276cfef01fe2138d07f3a7440f416d38bc"

pf_pid=""
cleanup() {
  if [[ -n "${pf_pid}" ]]; then
    kill "${pf_pid}" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

require_release() {
  if ! helm status "$1" -n "${NAMESPACE}" >/dev/null 2>&1; then
    echo "::error::Helm release $1 is not installed in ${NAMESPACE}" >&2
    exit 1
  fi
}

# Overlay keeps the defineNewEpoch command. Live host-contracts wins on a
# shared env name, including a secretKeyRef RPC.
stage_values() {
  local live="$1" out="$2"
  cp "${overlay}" "${out}"
  LIVE="${live}" yq -i '
    .scDeploy.image.tag = (load(strenv(LIVE)).scDeploy.image.tag // .scDeploy.image.tag) |
    .scDeploy.env = (
      .scDeploy.env as $base |
      (load(strenv(LIVE)).scDeploy.env // []) as $live |
      $base | map(
        .name as $n |
        (($live | map(select(.name == $n)))[0] // .)
      )
    )
  ' "${out}"
}

env_value() {
  yq -r ".scDeploy.env[] | select(.name == \"${2}\") | .value" "$1"
}

# Point cast at the preview host. An in-cluster URL is port-forwarded. A
# public URL is used as-is and not printed.
open_host_rpc() {
  local live="$1" url secret key host port svc ns
  url=$(yq -r '.scDeploy.env[] | select(.name == "RPC_URL") | .value // ""' "${live}")
  if [[ -z "${url}" || "${url}" == "null" ]]; then
    secret=$(yq -r '.scDeploy.env[] | select(.name == "RPC_URL") | .valueFrom.secretKeyRef.name' "${live}")
    key=$(yq -r '.scDeploy.env[] | select(.name == "RPC_URL") | .valueFrom.secretKeyRef.key' "${live}")
    if [[ -z "${secret}" || "${secret}" == "null" || -z "${key}" || "${key}" == "null" ]]; then
      echo "::error::host-contracts has no RPC_URL" >&2
      exit 1
    fi
    url=$(kubectl get secret "${secret}" -n "${NAMESPACE}" -o jsonpath="{.data.${key}}" | base64 -d)
  fi
  case "${url}" in
    http://*)
      host=${url#http://}
      host=${host%%/*}
      port=${host##*:}
      host=${host%:*}
      if [[ "${host}" == *.* ]]; then
        svc=${host%%.*}
        ns=${host#*.}
        ns=${ns%%.*}
      else
        svc=${host}
        ns=${NAMESPACE}
      fi
      kubectl port-forward -n "${ns}" "svc/${svc}" "${local_port}:${port}" >/dev/null 2>&1 &
      pf_pid=$!
      CAST_RPC="http://127.0.0.1:${local_port}"
      local i
      for ((i = 0; i < 20; i++)); do
        if cast block-number --rpc-url "${CAST_RPC}" >/dev/null 2>&1; then
          echo "host RPC via port-forward svc/${svc} in ${ns}"
          return 0
        fi
        sleep 1
      done
      echo "::error::port-forward to svc/${svc} in ${ns} did not answer" >&2
      exit 1
      ;;
    https://*)
      CAST_RPC="${url}"
      echo "host RPC is the public URL from the live host-contracts release"
      ;;
    *)
      echo "::error::RPC_URL is not http or https" >&2
      exit 1
      ;;
  esac
}

install_release() {
  local values="$1"
  if helm status "${release}" -n "${NAMESPACE}" >/dev/null 2>&1; then
    helm uninstall "${release}" -n "${NAMESPACE}"
  fi
  helm upgrade --install "${release}" "${chart}" \
    -n "${NAMESPACE}" -f "${values}" \
    --wait --wait-for-jobs --timeout="${timeout}"
}

job_name() {
  kubectl get job -n "${NAMESPACE}" -o jsonpath='{range .items[*]}{.metadata.name}{"\n"}{end}' \
    | grep "^${release}-deploy" | head -1
}

read_epoch() {
  cast call "${protocol_config}" 'getCurrentKmsContextAndEpoch()(uint256,uint256)' --rpc-url "${CAST_RPC}"
}

count_topic() {
  cast logs --from-block "${from_block}" --to-block latest --address "${protocol_config}" \
    --rpc-url "${CAST_RPC}" --json \
    | jq -r --arg t "$1" '[.[] | select(.topics[0] == $t)] | length'
}

to_seconds() {
  python3 -c 'import sys; s=sys.argv[1]; n=int(s[:-1]); u=s[-1]; print(n*{"s":1,"m":60,"h":3600}[u])' "$1"
}

require_release host-contracts

for cmd in helm kubectl yq cast jq python3; do
  command -v "${cmd}" >/dev/null || { echo "::error::missing ${cmd}" >&2; exit 1; }
done

live=$(mktemp)
helm get values host-contracts -n "${NAMESPACE}" -o yaml > "${live}"
parties=$(env_value "${live}" NUM_KMS_NODES)
if [[ ! "${parties}" =~ ^[0-9]+$ || "${parties}" -lt 1 ]]; then
  echo "::error::host-contracts has no NUM_KMS_NODES" >&2
  exit 1
fi

protocol_config=$(kubectl get configmap host-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.protocol_config\.address}')
if [[ ! "${protocol_config}" =~ ^0x[0-9a-fA-F]{40}$ ]]; then
  echo "::error::host-sc-addresses has no protocol_config.address" >&2
  exit 1
fi

open_host_rpc "${live}"
start=$(read_epoch)
start_context=$(awk 'NR==1 { print $1 }' <<<"${start}")
start_epoch=$(awk 'NR==2 { print $1 }' <<<"${start}")
echo "ProtocolConfig ${protocol_config}"
echo "parties ${parties}"
echo "current context ${start_context}"
echo "current epoch   ${start_epoch}"

values=$(mktemp)
stage_values "${live}" "${values}"
rm -f "${live}"
echo "Broadcasting defineNewEpochForCurrentKmsContext in ${NAMESPACE}"
install_release "${values}"
rm -f "${values}"

host_job=$(job_name)
if [[ -z "${host_job}" ]]; then
  echo "::error::no Job found for release ${release}" >&2
  exit 1
fi
logs=$(kubectl logs -n "${NAMESPACE}" "job/${host_job}")
if [[ ! "${logs}" =~ Broadcast\ defineNewEpochForCurrentKmsContext\ on\ (0x[0-9a-fA-F]+)\ \(tx:\ (0x[0-9a-fA-F]+)\)\. ]]; then
  echo "::error::job/${host_job} completed without the defineNewEpoch broadcast line" >&2
  exit 1
fi
tx_target="${BASH_REMATCH[1]}"
tx_hash="${BASH_REMATCH[2]}"
echo "Broadcast defineNewEpochForCurrentKmsContext on ${tx_target} (tx: ${tx_hash}). New epoch is now PENDING."
from_block=$(cast to-dec "$(cast tx "${tx_hash}" --rpc-url "${CAST_RPC}" --json | jq -r '.blockNumber')")
echo "watching logs from block ${from_block}"

deadline=$((SECONDS + $(to_seconds "${watch_timeout}")))
activated=0
while (( SECONDS < deadline )); do
  confirms=$(count_topic "${topic_confirm}")
  activates=$(count_topic "${topic_activate}")
  now=$(read_epoch)
  context_id=$(awk 'NR==1 { print $1 }' <<<"${now}")
  epoch_id=$(awk 'NR==2 { print $1 }' <<<"${now}")
  echo "$(date -u +%H:%M:%S) confirmations=${confirms}/${parties} ActivateEpoch=${activates} context=${context_id} epoch=${epoch_id}"
  if (( confirms >= parties && activates >= 1 )) && [[ "${epoch_id}" != "${start_epoch}" ]]; then
    if [[ "${context_id}" != "${start_context}" ]]; then
      echo "::error::context changed from ${start_context} to ${context_id}" >&2
      exit 1
    fi
    echo "epoch moved from ${start_epoch} to ${epoch_id}"
    activated=1
    break
  fi
  sleep "${poll_secs}"
done
if [[ "${activated}" -ne 1 ]]; then
  echo "::error::epoch did not activate within ${watch_timeout}" >&2
  exit 1
fi

if [[ -n "${DECRYPT_CMD:-}" ]]; then
  echo "running DECRYPT_CMD"
  bash -lc "${DECRYPT_CMD}"
else
  pod=$(kubectl get pods -n "${NAMESPACE}" --field-selector=status.phase=Running -o jsonpath='{range .items[*]}{.metadata.name}{"\n"}{end}' \
    | grep '^test-suite-' | head -1 || true)
  if [[ -z "${pod}" ]]; then
    echo "::error::no running test-suite pod in ${NAMESPACE}; set DECRYPT_CMD or start the idle test-suite" >&2
    exit 1
  fi
  echo "user decrypt and public decrypt via ${pod}"
  kubectl exec -n "${NAMESPACE}" "${pod}" -- ./run-tests.sh --no-hardhat-compile -g "test user input uint64"
fi

echo "Job ${host_job} is Complete. Release ${release} stays installed."
