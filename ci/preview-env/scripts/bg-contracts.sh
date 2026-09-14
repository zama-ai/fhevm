#!/usr/bin/env bash
# Blue/Green QA, production path step 4: upgrade the host and gateway contracts of a preview env
# from the deployed release (e.g. v0.14.1-1) to the Green release (the branch's image tag), in
# place on the existing proxies, while Blue keeps serving.
#
#   bg-contracts.sh status    per chain and contract: deployed tag, target tag, reinitializer version
#                             deployed -> target, whether an upgrade will run
#   bg-contracts.sh upgrade   one contracts-chart release per chain in upgrade mode: the old image's
#                             sources are copied in, the new image runs task:upgrade<Contract> against
#                             every proxy whose reinitializer version the target bumps, with the owner
#                             key and env the deploy release already carries; waits for the Job,
#                             checks the ConfigMap version and the implementation slots
#
# Contracts whose REINITIALIZER_VERSION did not change are skipped: the upgrade tasks always call
# reinitializeV<N> and a proxy already initialised at N would revert (fresh deploys initialise at N).
# A release without code change does not bump N, so skipping is the correct outcome.
#
# Usage: NAMESPACE=<ns> bash ci/preview-env/scripts/bg-contracts.sh status|upgrade
# Env: NAMESPACE (required); TARGET_TAG (default: the env's test-suite image tag, i.e. the deployed
#      branch SHA - every contracts image carries it); CONTRACTS_CHART (charts/contracts of this
#      checkout); GATEWAY_RPC_URL (optional, implementation checks on the gateway);
#      FROM_TAG (default: contracts.version of the address ConfigMap; override to preview a plan);
#      DRY_RUN (false: with true, `upgrade` renders the release and prints the Job script instead).
set -euo pipefail
DRY_RUN="${DRY_RUN:-false}"

verb="${1:?usage: bg-contracts.sh status|upgrade}"
: "${NAMESPACE:?}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
CONTRACTS_CHART="${CONTRACTS_CHART:-${root}/charts/contracts}"
TARGET_TAG="${TARGET_TAG:-$(kubectl get job -n "${NAMESPACE}" test-suite -o jsonpath='{.spec.template.spec.containers[0].image}' | sed 's/.*://')}"
IMPL_SLOT=0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
work=$(mktemp -d)
trap 'rm -rf "${work}"' EXIT
fail() { echo "::error::$*" >&2; exit 1; }

# release -> "<contracts dir> <address ConfigMap> <chain label>"; contract -> "<ConfigMap key> <env var the task reads>"
release_info() {
  case "$1" in
    host-contracts)         echo "host-contracts host-sc-addresses sepolia" ;;
    host-contracts-polygon) echo "host-contracts polygon-sc-addresses amoy" ;;
    gateway-contracts)      echo "gateway-contracts gw-sc-addresses gateway" ;;
    *) return 1 ;;
  esac
}
contract_info() { # <dir> <Contract>
  case "$1/$2" in
    host-contracts/ACL)                  echo "acl.address ACL_CONTRACT_ADDRESS" ;;
    host-contracts/FHEVMExecutor)        echo "fhevm_executor.address FHEVM_EXECUTOR_CONTRACT_ADDRESS" ;;
    host-contracts/HCULimit)             echo "hcu_limit.address HCU_LIMIT_CONTRACT_ADDRESS" ;;
    host-contracts/InputVerifier)        echo "input_verifier.address INPUT_VERIFIER_CONTRACT_ADDRESS" ;;
    host-contracts/KMSGeneration)        echo "kms_generation.address KMS_GENERATION_CONTRACT_ADDRESS" ;;
    host-contracts/KMSVerifier)          echo "kms_verifier.address KMS_VERIFIER_CONTRACT_ADDRESS" ;;
    host-contracts/ProtocolConfig)       echo "protocol_config.address PROTOCOL_CONFIG_CONTRACT_ADDRESS" ;;
    gateway-contracts/CiphertextCommits) echo "ciphertext_commits.address CIPHERTEXT_COMMITS_ADDRESS" ;;
    gateway-contracts/Decryption)        echo "decryption.address DECRYPTION_ADDRESS" ;;
    gateway-contracts/GatewayConfig)     echo "gateway_config.address GATEWAY_CONFIG_ADDRESS" ;;
    gateway-contracts/InputVerification) echo "input_verification.address INPUT_VERIFICATION_ADDRESS" ;;
    *) return 1 ;;
  esac
}
contracts_of() { case "$1" in host-contracts) echo "ACL FHEVMExecutor HCULimit InputVerifier KMSGeneration KMSVerifier ProtocolConfig" ;; gateway-contracts) echo "CiphertextCommits Decryption GatewayConfig InputVerification" ;; esac; }

releases=()
for rel in host-contracts host-contracts-polygon gateway-contracts; do
  helm status "${rel}" -n "${NAMESPACE}" >/dev/null 2>&1 && releases+=("${rel}")
done
[[ ${#releases[@]} -gt 0 ]] || fail "no contracts release found in ${NAMESPACE}"

# REINITIALIZER_VERSION of <dir>/contracts/<Contract>.sol at a git ref of this repo (image tags are short SHAs or release tags).
source_version() { # <ref> <dir> <Contract>
  git -C "${root}" show "$1:$2/contracts/$3.sol" 2>/dev/null | grep -oE 'REINITIALIZER_VERSION = [0-9]+' | head -1 | grep -oE '[0-9]+' || true
}
resolve_ref() { # an image tag is either a release tag (vX.Y.Z-N) or a short commit SHA
  git -C "${root}" rev-parse --verify --quiet "$1^{commit}" >/dev/null 2>&1 && echo "$1" || fail "tag '$1' is not a commit or tag of this checkout; fetch it first (git fetch --tags)"
}
cm_value() { kubectl get configmap -n "${NAMESPACE}" "$1" -o json | jq -r --arg k "$2" '.data[$k] // empty'; }
rpc_for() { # release -> RPC URL usable from this machine, or empty
  case "$1" in
    host-contracts)         kubectl get secret -n "${NAMESPACE}" rpc -o jsonpath='{.data.ethereum-rpc-url}' | base64 -d ;;
    host-contracts-polygon) kubectl get secret -n "${NAMESPACE}" rpc -o jsonpath='{.data.polygon-rpc-url}' | base64 -d ;;
    gateway-contracts)      echo "${GATEWAY_RPC_URL:-}" ;;
  esac
}
impl_of() { [[ -n "$1" ]] && cast storage "$2" "${IMPL_SLOT}" --rpc-url "$1" 2>/dev/null | sed -E 's/^0x0{24}/0x/' || echo "?"; }

# Decide per release what to upgrade. Sets PLAN (lines "<Contract> <cm key> <env> <from> <to> <proxy>") for contracts with a bump.
plan_release() {
  local rel="$1" dir cm chain current target_ref current_ref
  read -r dir cm chain <<<"$(release_info "${rel}")"
  current="${FROM_TAG:-$(cm_value "${cm}" contracts.version)}"; [[ -n "${current}" ]] || fail "${rel}: ${cm} has no contracts.version"
  current_ref=$(resolve_ref "${current}"); target_ref=$(resolve_ref "${TARGET_TAG}")
  PLAN=(); SKIPPED=()
  for c in $(contracts_of "${dir}"); do
    read -r key env <<<"$(contract_info "${dir}" "${c}")"
    proxy=$(cm_value "${cm}" "${key}")
    [[ -n "${proxy}" ]] || { SKIPPED+=("${c}: not deployed on this chain"); continue; }
    from=$(source_version "${current_ref}" "${dir}" "${c}"); to=$(source_version "${target_ref}" "${dir}" "${c}")
    [[ -n "${from}" && -n "${to}" ]] || { SKIPPED+=("${c}: no REINITIALIZER_VERSION found (${from:-?} -> ${to:-?})"); continue; }
    if [[ "${to}" -gt "${from}" ]]; then PLAN+=("${c} ${key} ${env} ${from} ${to} ${proxy}"); else SKIPPED+=("${c}: reinitializer ${from} unchanged, no upgrade"); fi
  done
  echo "== ${rel} (${chain}): deployed ${current} -> target ${TARGET_TAG}"
  for line in "${PLAN[@]+"${PLAN[@]}"}"; do read -r c _ _ from to proxy <<<"${line}"; echo "   upgrade ${c} ${proxy} (reinitializer ${from} -> ${to}) impl=$(impl_of "$(rpc_for "${rel}")" "${proxy}")"; done
  for line in "${SKIPPED[@]+"${SKIPPED[@]}"}"; do echo "   skip    ${line}"; done
}

upgrade_release() {
  local rel="$1" dir cm chain current
  read -r dir cm chain <<<"$(release_info "${rel}")"
  current="${FROM_TAG:-$(cm_value "${cm}" contracts.version)}"
  plan_release "${rel}"
  [[ ${#PLAN[@]} -gt 0 ]] || { echo "   nothing to upgrade"; return 0; }
  local name="${rel}-upgrade-${TARGET_TAG}" base="${work}/${rel}.yaml" cmds="${work}/${rel}-cmds.json" env_json="${work}/${rel}-env.json"
  helm get values "${rel}" -n "${NAMESPACE}" -o yaml > "${base}"
  # Upgrade commands: old sources are copied to /app/oldContracts by the chart's init container.
  jq -n '[]' > "${cmds}"; jq -n '[]' > "${env_json}"
  for line in "${PLAN[@]}"; do
    read -r c key env _ _ _ <<<"${line}"
    jq --arg c "${c}" '. + ["npx hardhat task:upgrade\($c) --current-implementation oldContracts/\($c).sol:\($c) --new-implementation contracts/\($c).sol:\($c) --use-internal-proxy-address false --verify-contract false"]' "${cmds}" > "${cmds}.tmp" && mv "${cmds}.tmp" "${cmds}"
    jq --arg n "${env}" --arg cm "${cm}" --arg k "${key}" '. + [{"name": $n, "valueFrom": {"configMapKeyRef": {"name": $cm, "key": $k}}}]' "${env_json}" > "${env_json}.tmp" && mv "${env_json}.tmp" "${env_json}"
  done
  # Deploy env (owner key, RPC, HARDHAT_NETWORK, reinitializer inputs) + the proxy addresses the tasks read.
  ENV_ADD="$(cat "${env_json}")" yq -i '.scDeploy.env = (.scDeploy.env // []) + (strenv(ENV_ADD) | fromjson)' "${base}"
  local image_name
  image_name=$(yq -r '.scDeploy.image.name' "${base}")
  local helm_args=(-n "${NAMESPACE}" -f "${base}"
    --set scUpgrade.enabled=true
    --set-string "scUpgrade.oldContracts.image.name=${image_name}"
    --set-string "scUpgrade.oldContracts.image.tag=${current}"
    --set-string "scDeploy.image.tag=${TARGET_TAG}"
    --set-json "scUpgrade.upgradeCommands=$(cat "${cmds}")"
    --set persistence.enabled=false)
  if [[ "${DRY_RUN}" == "true" ]]; then
    echo "   DRY_RUN: rendering ${name}"
    helm template "${name}" "${CONTRACTS_CHART}" "${helm_args[@]}" > "${work}/${rel}-render.yaml"
    yq 'select(.kind == "ConfigMap" and (.metadata.name | test("-config$"))) | .data["upgrade-contracts.sh"]' "${work}/${rel}-render.yaml" | grep -E "npx hardhat|kubectl patch" | sed 's/^/     /'
    yq 'select(.kind == "Job") | "     job " + .metadata.name + " image=" + .spec.template.spec.containers[0].image + " old=" + .spec.template.spec.initContainers[0].image' "${work}/${rel}-render.yaml"
    return 0
  fi
  echo "   installing ${name}: $(jq -r 'length' "${cmds}") upgrade command(s)"
  helm upgrade --install "${name}" "${CONTRACTS_CHART}" "${helm_args[@]}" \
    --wait --wait-for-jobs --timeout 30m >/dev/null || {
      echo "::error::${name}: upgrade Job failed; last log lines:" >&2
      kubectl logs -n "${NAMESPACE}" -l "app.kubernetes.io/name=${name}-deploy" --tail=60 2>/dev/null | cut -c1-300 >&2 || true
      exit 1
    }
  local after
  after=$(cm_value "${cm}" contracts.version)
  [[ "${after}" == "${TARGET_TAG}" ]] || fail "${rel}: ${cm} contracts.version is ${after}, expected ${TARGET_TAG}"
  echo "   ${cm} contracts.version = ${after}"
  for line in "${PLAN[@]}"; do read -r c _ _ _ _ proxy <<<"${line}"; echo "   ${c} impl now $(impl_of "$(rpc_for "${rel}")" "${proxy}")"; done
}

case "${verb}" in
status)
  for rel in "${releases[@]}"; do plan_release "${rel}"; done
  ;;
upgrade)
  for rel in "${releases[@]}"; do upgrade_release "${rel}"; done
  echo "== contracts upgrade done: target ${TARGET_TAG} on ${#releases[@]} release(s). Next: bg-green.sh migrate, then start, then resume traffic."
  ;;
*)
  fail "unknown verb '${verb}' (status|upgrade)"
  ;;
esac
