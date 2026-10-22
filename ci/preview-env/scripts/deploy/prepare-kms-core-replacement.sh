#!/usr/bin/env bash
# Prepare a preview namespace so one KMS party can be replaced by a new core
# before kms-context-switch.sh.
#
# Usage: NAMESPACE=<ns> prepare-kms-core-replacement.sh <party-id> <new-name-id>
# Example: NAMESPACE=fhevm-ci-abc-123 prepare-kms-core-replacement.sh 1 5
#
# <party-id> is the committee slot that stays in the config (kmsPeers.id and
# peersList[].id). <new-name-id> is used only in the release name, the pod
# name, and the vault prefix. Replacing party 1 with name 5 installs release
# kms-core-5, pod kms-core-5-core-1, prefixes PUB-p5 / PRIV-p5 / BACKUP-p5,
# and leaves kmsPeers.id at 1.
#
# The chart fetches that party's CA from PUB-p<party-id>. This script points
# that fetch at PUB-p<new-name-id> and then removes the peer list. The server
# must boot with peers: None. A peer list at boot on this core would store the
# new identity inside the current context, and the same list on the other
# cores would delete their private context. This script does not restart the
# other cores and does not broadcast the context switch.
#
# Env: NAMESPACE (required).
#      KMS_CORE_CHART (default the OCI kms-core chart).
#      KMS_CONNECTOR_CHART (default the OCI kms-connector chart).
# Run from anywhere. Not part of preview-env-deploy.
set -euo pipefail

: "${NAMESPACE:?NAMESPACE is required}"

if [[ $# -ne 2 ]]; then
  echo "usage: NAMESPACE=<ns> $0 <party-id> <new-name-id>" >&2
  exit 2
fi
party="$1"
name="$2"
if [[ ! "${party}" =~ ^[1-9][0-9]*$ || ! "${name}" =~ ^[1-9][0-9]*$ ]]; then
  echo "::error::party id and new name id must be positive integers" >&2
  exit 2
fi
if [[ "${party}" -eq "${name}" ]]; then
  echo "::error::the new name id must differ from the party id; the party id stays in the config" >&2
  exit 2
fi

kms_chart="${KMS_CORE_CHART:-oci://hub.zama.org/ghcr/zama-ai/kms/charts/kms-core}"
connector_chart="${KMS_CONNECTOR_CHART:-oci://hub.zama.org/ghcr/zama-ai/fhevm/charts/kms-connector}"
source_release="kms-core-${party}"
release="kms-core-${name}"
sts="${release}-core"
pod="${release}-core-${party}"
configmap="${sts}-config"
old_prefix="PUB-p${party}"
new_prefix="PUB-p${name}"
source_sts="${source_release}-core"
connector_release="kms-connector-${party}"

for cmd in helm kubectl yq python3; do
  command -v "${cmd}" >/dev/null || { echo "::error::missing ${cmd}" >&2; exit 1; }
done

chart_version() {
  helm get metadata "$1" -n "${NAMESPACE}" | awk '/^VERSION:/{print $2; exit}'
}

tmp_values=$(mktemp)
tmp_sts=$(mktemp)
tmp_cm=$(mktemp)
cleanup() { rm -f "${tmp_values}" "${tmp_sts}" "${tmp_cm}"; }
trap cleanup EXIT

if ! helm status "${source_release}" -n "${NAMESPACE}" >/dev/null 2>&1; then
  echo "::error::Helm release ${source_release} is not installed in ${NAMESPACE}" >&2
  exit 1
fi
helm get values "${source_release}" -n "${NAMESPACE}" -o yaml > "${tmp_values}"

party_hits=$(PARTY="${party}" yq '[.kmsCore.thresholdMode.peersList[] | select(.id == (strenv(PARTY) | tonumber))] | length' "${tmp_values}")
if [[ "${party_hits}" == "0" ]]; then
  echo "::error::${source_release} has no peersList entry with id ${party}" >&2
  exit 1
fi
name_hits=$(NAME="${name}" yq '[.kmsCore.thresholdMode.peersList[] | select(.id == (strenv(NAME) | tonumber))] | length' "${tmp_values}")
if [[ "${name_hits}" != "0" ]]; then
  echo "::error::name id ${name} is already a party id in the peer list; it is only a release name" >&2
  exit 1
fi

# A replaced party keeps its id and runs as kms-core-<name>-core-<id>, so the
# original pod kms-core-<id>-core-<id> may already be scaled to 0.
running_core() {
  local id="$1"
  kubectl get pods -n "${NAMESPACE}" --field-selector=status.phase=Running -o jsonpath='{range .items[*]}{.metadata.name}{"\n"}{end}' \
    | grep -E "^kms-core-[0-9]+-core-${id}$" | head -1 || true
}

while IFS= read -r other; do
  [[ -z "${other}" || "${other}" == "${party}" ]] && continue
  live=$(running_core "${other}")
  if [[ -z "${live}" ]]; then
    echo "::error::party ${other} has no running core (kms-core-*-core-${other}); this script leaves every party other than ${party} untouched" >&2
    exit 1
  fi
  echo "Party ${other} stays on ${live}"
done < <(yq -r '.kmsCore.thresholdMode.peersList[].id' "${tmp_values}")

echo "Scaling ${source_sts} to 0. Every other party stays up."
kubectl scale sts "${source_sts}" -n "${NAMESPACE}" --replicas=0

kms_version=$(chart_version "${source_release}")
if [[ -z "${kms_version}" ]]; then
  echo "::error::could not read the ${source_release} chart version" >&2
  exit 1
fi
echo "Installing ${release} from kms-core chart ${kms_version}. Config party id stays ${party}. Name id ${name} is the release, the pod, and ${new_prefix}."
PARTY="${party}" NAME="${name}" HOST="${pod}" yq -i '
  .kmsPeers.id = (strenv(PARTY) | tonumber) |
  .kmsPeers.count = 1 |
  .kmsCore.publicVault.s3.prefix = "PUB-p" + strenv(NAME) |
  .kmsCore.privateVault.s3.prefix = "PRIV-p" + strenv(NAME) |
  .kmsCore.backupVault.s3.prefix = "BACKUP-p" + strenv(NAME) |
  (.kmsCore.thresholdMode.peersList[] | select(.id == (strenv(PARTY) | tonumber))).host = strenv(HOST)
' "${tmp_values}"
if ! PARTY="${party}" HOST="${pod}" yq -e '.kmsCore.thresholdMode.peersList[] | select(.id == (strenv(PARTY) | tonumber) and .host == strenv(HOST))' "${tmp_values}" >/dev/null; then
  echo "::error::failed to set the peersList host for party ${party}" >&2
  exit 1
fi

# No --wait. The peer list is still present, so the server would crashloop on
# the old prefix until the patches below. Helm still finishes the pre-install
# keygen hook before it returns.
helm upgrade --install "${release}" "${kms_chart}" \
  --version "${kms_version}" -n "${NAMESPACE}" -f "${tmp_values}"
kubectl scale sts "${sts}" -n "${NAMESPACE}" --replicas=0
kubectl rollout status "sts/${sts}" -n "${NAMESPACE}" --timeout=3m

echo "Pointing the party-${party} CA fetch at ${new_prefix}"
kubectl get sts "${sts}" -n "${NAMESPACE}" -o json > "${tmp_sts}"
python3 - "${tmp_sts}" "${old_prefix}" "${new_prefix}" << 'PY'
import json, sys
path, old_prefix, new_prefix = sys.argv[1:]
old = f"prefix={old_prefix}/CACert/"
new = f"prefix={new_prefix}/CACert/"
sts = json.load(open(path))
sts.pop("status", None)
meta = sts["metadata"]
for key in ("resourceVersion", "uid", "managedFields", "creationTimestamp", "generation"):
    meta.pop(key, None)
found = False
for container in sts["spec"]["template"]["spec"].get("initContainers") or []:
    if container["name"] != "kms-core-init-load-env":
        continue
    args = container.get("args") or []
    container["args"] = [arg.replace(old, new) for arg in args]
    found = any(new in arg for arg in container["args"])
if not found:
    sys.exit(f"init container kms-core-init-load-env has no {old} fetch")
json.dump(sts, open(path, "w"))
PY
kubectl apply -n "${NAMESPACE}" -f "${tmp_sts}"

echo "Removing [[threshold.peers]] from ${configmap}"
kubectl get cm "${configmap}" -n "${NAMESPACE}" -o json > "${tmp_cm}"
python3 - "${tmp_cm}" << 'PY'
import json, re, sys
path = sys.argv[1]
cm = json.load(open(path))
cm.pop("status", None)
meta = cm["metadata"]
for key in ("resourceVersion", "uid", "managedFields", "creationTimestamp"):
    meta.pop(key, None)
toml = cm["data"]["kms-server.toml"]
toml = re.sub(r"\n\[\[threshold\.peers\]\][\s\S]*?(?=\n\[\[|\n\[(?!threshold)|\Z)", "\n", toml)
if "[[threshold.peers]]" in toml:
    sys.exit("peer list is still present after the strip")
cm["data"]["kms-server.toml"] = toml
json.dump(cm, open(path, "w"))
PY
kubectl apply -n "${NAMESPACE}" -f "${tmp_cm}"

echo "Starting ${pod}"
kubectl scale sts "${sts}" -n "${NAMESPACE}" --replicas=1
kubectl rollout status "sts/${sts}" -n "${NAMESPACE}" --timeout=10m

ready=0
logs=""
for _ in $(seq 1 30); do
  logs=$(kubectl logs -n "${NAMESPACE}" "${pod}" -c kms-core-enclave-logger --tail=400 2>/dev/null || true)
  if grep -q "peers: None" <<<"${logs}"; then
    ready=1
    break
  fi
  sleep 2
done
if [[ "${ready}" -ne 1 ]]; then
  echo "::error::${pod} did not log peers: None. Do not helm-upgrade ${release}; that restores the peer list and the ${old_prefix} fetch." >&2
  exit 1
fi
if grep -q "MPC context" <<<"${logs}" && ! grep -q "No MPC context found" <<<"${logs}"; then
  echo "::error::${pod} already has a stored MPC context. A boot with the peer list writes context 1 under PRIV-p${name}." >&2
  exit 1
fi
echo "${pod} is up with peers: None. kmsPeers.id is still ${party}."

connector_version=$(chart_version "${connector_release}")
if [[ -z "${connector_version}" ]]; then
  echo "::error::could not read the ${connector_release} chart version" >&2
  exit 1
fi
echo "Pointing ${connector_release} at http://${pod}:50100 (chart ${connector_version}). The tx-sender secret stays party ${party}."
helm upgrade "${connector_release}" "${connector_chart}" \
  --version "${connector_version}" -n "${NAMESPACE}" --reuse-values \
  --set-string kmsConnectorKmsWorker.config.kmsCoreEndpoints="http://${pod}:50100"

echo "Replacement is prepared. Parties other than ${party} were not restarted."
echo "Do not helm-upgrade ${release}; that restores the peer list and the ${old_prefix} CA fetch."
echo "kms-context-switch.sh uses the Running pod kms-core-<name>-core-<party>."
echo "This party will be ${pod} with prefix PUB-p${name}."
echo "Next, from this repo:"
echo "  CONTRACTS_CHART=\"\$PWD/contracts\" NAMESPACE=${NAMESPACE} bash ci/preview-env/scripts/deploy/kms-context-switch.sh"
echo "To force this slot: KMS_CORE_NAMES=${party}=${name}"
