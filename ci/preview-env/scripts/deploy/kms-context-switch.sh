#!/usr/bin/env bash
# Same-committee KMS context switch on an already deployed preview namespace.
# Copies the live host-contracts and gateway-contracts committee, then replaces
# KMS_NODE_CA_CERT_<i> from each party's public-vault PEM, the node URL and MPC
# identity with the peer service the TLS certificate names, and
# KMS_SOFTWARE_VERSION and KMS_PCR_VALUES from the running kms-core-1 image
# and its trusted-release PCRs. Broadcasts defineNewKmsContextAndEpoch, then
# updateKmsContext with the printed next id.
#
# Env: NAMESPACE (required). CONTRACTS_CHART (default charts/contracts).
#      CONTEXT_SWITCH_TIMEOUT (helm --timeout, default 20m).
# Run from a checkout of this repo. Not part of preview-env-deploy.
set -euo pipefail

: "${NAMESPACE:?NAMESPACE is required}"

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
root=$(cd "${script_dir}/../../../.." && pwd)
cd "${root}"

chart="${CONTRACTS_CHART:-charts/contracts}"
timeout="${CONTEXT_SWITCH_TIMEOUT:-20m}"
host_overlay=ci/preview-env/host-chain/values-host-define-new-kms-context-e2e.yaml
gw_overlay=ci/preview-env/gateway-chain/values-gateway-update-kms-context-e2e.yaml
host_release=host-define-new-kms-context
gw_release=gateway-update-kms-context

require_release() {
  if ! helm status "$1" -n "${NAMESPACE}" >/dev/null 2>&1; then
    echo "::error::Helm release $1 is not installed in ${NAMESPACE}" >&2
    exit 1
  fi
}

# Overlay keeps its commands. Live release wins on a shared env name (testnet
# RPC and deployer) and supplies the indexed committee. KMS_CONTEXT_ID is not
# copied; the gateway step sets the next id. CA certs, node URL, MPC identity,
# software version, and PCRs are replaced by apply_running_kms_material before
# the host broadcast.
stage_values() {
  local live_release="$1" overlay="$2" out="$3"
  local live
  live=$(mktemp)
  helm get values "${live_release}" -n "${NAMESPACE}" -o yaml > "${live}"
  cp "${overlay}" "${out}"
  LIVE="${live}" yq -i '
    .scDeploy.image.tag = (load(strenv(LIVE)).scDeploy.image.tag // .scDeploy.image.tag) |
    .scDeploy.env = (
      .scDeploy.env as $base |
      (load(strenv(LIVE)).scDeploy.env // []) as $live |
      (
        $base | map(
          .name as $n |
          (($live | map(select(.name == $n)))[0] // .)
        )
      ) + (
        $live | map(select(
          (
            .name == "NUM_KMS_NODES" or
            .name == "PUBLIC_DECRYPTION_THRESHOLD" or
            .name == "USER_DECRYPTION_THRESHOLD" or
            .name == "KMS_GENERATION_THRESHOLD" or
            .name == "KMS_GEN_THRESHOLD" or
            .name == "MPC_THRESHOLD" or
            .name == "KMS_SOFTWARE_VERSION" or
            .name == "KMS_PCR_VALUES" or
            (.name | test("^(KMS_TX_SENDER_ADDRESS_|KMS_SIGNER_ADDRESS_|KMS_NODE_)"))
          ) and (.name as $n | $base | all_c(.name != $n))
        ))
      )
    )
  ' "${out}"
  rm -f "${live}"
  if ! yq -e '.scDeploy.env[] | select(.name == "NUM_KMS_NODES")' "${out}" >/dev/null; then
    echo "::error::${live_release} has no NUM_KMS_NODES to copy" >&2
    exit 1
  fi
}

env_value() {
  yq -r ".scDeploy.env[] | select(.name == \"${2}\") | .value" "$1"
}

set_env() {
  local file="$1" name="$2" value="$3"
  if NAME="${name}" yq -e '.scDeploy.env[] | select(.name == strenv(NAME))' "${file}" >/dev/null 2>&1; then
    NAME="${name}" VALUE="${value}" yq -i '
      (.scDeploy.env[] | select(.name == strenv(NAME))).value = strenv(VALUE)
    ' "${file}"
  else
    NAME="${name}" VALUE="${value}" yq -i '.scDeploy.env += [{"name": strenv(NAME), "value": strenv(VALUE)}]' "${file}"
  fi
}

# Newest PUB-p<i>/CACert object. The body is the PEM kms-gen-keys stored; the
# host task wants it as 0x-hex of those bytes.
fetch_ca_cert() {
  STORAGE_URL="$1" STORAGE_PREFIX="$2" python3 - <<'PY'
import os, re, sys, urllib.request
from xml.etree import ElementTree as ET

base = os.environ["STORAGE_URL"].rstrip("/")
prefix = os.environ["STORAGE_PREFIX"].strip("/")
list_url = f"{base}?list-type=2&prefix={prefix}/CACert/"
try:
    xml = urllib.request.urlopen(list_url, timeout=30).read()
except Exception as exc:
    sys.exit(f"listing {list_url} failed: {exc}")
root = ET.fromstring(xml)
ns = {"s": "http://s3.amazonaws.com/doc/2006-03-01/"}
items = []
for contents in root.findall("s:Contents", ns) or root.findall("Contents"):
    key = contents.findtext("s:Key", default="", namespaces=ns) or contents.findtext("Key", default="")
    modified = contents.findtext("s:LastModified", default="", namespaces=ns) or contents.findtext("LastModified", default="")
    if key and re.search(r"/CACert/[^/]+$", key):
        items.append((modified, key))
if not items:
    sys.exit(f"no CACert object under {prefix} at {base}")
items.sort(reverse=True)
key = items[0][1]
try:
    data = urllib.request.urlopen(f"{base}/{key}", timeout=30).read()
except Exception as exc:
    sys.exit(f"fetching {base}/{key} failed: {exc}")
if b"-----BEGIN CERTIFICATE-----" not in data or b"-----END CERTIFICATE-----" not in data:
    sys.exit(f"{key} is not a PEM certificate")
sys.stdout.write("0x" + data.hex())
PY
}

# NewMpcContext parses ca_cert as a PEM, ipAddress as a URL, and checks
# mpc_identity against the TLS certificate CN (kms-core-<i>-core-<i>). The
# cores attest the image that is actually running.
apply_running_kms_material() {
  local values="$1"
  local n pod image tag cm toml pcr_json i idx url prefix cert
  n=$(env_value "${values}" NUM_KMS_NODES)
  if [[ ! "${n}" =~ ^[0-9]+$ ]] || [[ "${n}" -lt 1 ]]; then
    echo "::error::NUM_KMS_NODES is not a party count (${n})" >&2
    exit 1
  fi
  pod=$(kubectl get pods -n "${NAMESPACE}" -o jsonpath='{range .items[*]}{.metadata.name}{"\n"}{end}' \
    | grep -E '^kms-core-1-core-1$' | head -1)
  if [[ -z "${pod}" ]]; then
    echo "::error::pod kms-core-1-core-1 is not in ${NAMESPACE}" >&2
    exit 1
  fi
  image=$(kubectl get pod "${pod}" -n "${NAMESPACE}" -o json | python3 -c '
import json, sys
doc = json.load(sys.stdin)
print(next(c["image"] for c in doc["spec"]["containers"] if "core-service-enclave" in c["image"]))
')
  tag="${image##*:}"
  if [[ -z "${tag}" || "${tag}" == "${image}" ]]; then
    echo "::error::could not read the kms-core image tag from ${image}" >&2
    exit 1
  fi
  set_env "${values}" KMS_SOFTWARE_VERSION "${tag}"
  echo "KMS software version: ${tag}"

  cm=$(kubectl get pod "${pod}" -n "${NAMESPACE}" -o json | python3 -c '
import json, sys
doc = json.load(sys.stdin)
for volume in doc["spec"]["volumes"]:
    config = volume.get("configMap") or {}
    name = config.get("name", "")
    if name.endswith("core-config"):
        print(name)
        break
else:
    sys.exit(1)
')
  toml=$(kubectl get configmap "${cm}" -n "${NAMESPACE}" -o go-template='{{index .data "kms-server.toml"}}')
  pcr_json=$(python3 -c '
import json, re, sys
found = re.findall(r"pcr([012])\s*=\s*\"([0-9a-fA-F]+)\"", sys.stdin.read())
triples = []
index = 0
while index + 2 < len(found):
    if [item[0] for item in found[index:index + 3]] == ["0", "1", "2"]:
        def hx(value):
            return value if value.startswith("0x") else "0x" + value
        triples.append({
            "pcr0": hx(found[index][1]),
            "pcr1": hx(found[index + 1][1]),
            "pcr2": hx(found[index + 2][1]),
        })
        index += 3
    else:
        index += 1
if not triples:
    sys.exit("no trusted-release PCR triple in kms-server.toml")
sys.stdout.write(json.dumps(triples, separators=(",", ":")))
' <<<"${toml}")
  set_env "${values}" KMS_PCR_VALUES "${pcr_json}"
  echo "KMS PCR triples from ${cm}: $(python3 -c 'import json,sys; print(len(json.loads(sys.argv[1])))' "${pcr_json}")"

  for ((i = 1; i <= n; i++)); do
    idx=$((i - 1))
    url=$(env_value "${values}" "KMS_NODE_STORAGE_URL_${idx}")
    prefix=$(env_value "${values}" "KMS_NODE_STORAGE_PREFIX_${idx}")
    if [[ -z "${url}" || -z "${prefix}" || "${url}" == "null" || "${prefix}" == "null" ]]; then
      echo "::error::party ${i} is missing a storage URL or prefix" >&2
      exit 1
    fi
    if ! cert=$(fetch_ca_cert "${url}" "${prefix}"); then
      echo "::error::party ${i} CA cert: ${cert}" >&2
      exit 1
    fi
    node_host="kms-core-${i}-core-${i}"
    set_env "${values}" "KMS_NODE_IP_${idx}" "http://${node_host}:50001"
    set_env "${values}" "KMS_NODE_MPC_IDENTITY_${idx}" "${node_host}"
    set_env "${values}" "KMS_NODE_CA_CERT_${idx}" "${cert}"
    echo "Party ${i}: CA cert from ${prefix}/CACert ($(wc -c <<<"${cert}" | tr -d ' ') hex chars)"
  done
}

install_release() {
  local release="$1" values="$2"
  if helm status "${release}" -n "${NAMESPACE}" >/dev/null 2>&1; then
    helm uninstall "${release}" -n "${NAMESPACE}"
  fi
  helm upgrade --install "${release}" "${chart}" \
    -n "${NAMESPACE}" -f "${values}" \
    --wait --wait-for-jobs --timeout="${timeout}"
}

job_name() {
  kubectl get job -n "${NAMESPACE}" -o jsonpath='{range .items[*]}{.metadata.name}{"\n"}{end}' \
    | grep "^${1}-deploy" | head -1
}

require_release host-contracts
require_release gateway-contracts

host_values=$(mktemp)
stage_values host-contracts "${host_overlay}" "${host_values}"
apply_running_kms_material "${host_values}"
echo "Broadcasting defineNewKmsContextAndEpoch in ${NAMESPACE}"
install_release "${host_release}" "${host_values}"
rm -f "${host_values}"

host_job=$(job_name "${host_release}")
if [[ -z "${host_job}" ]]; then
  echo "::error::no Job found for release ${host_release}" >&2
  exit 1
fi
new_id=$(kubectl logs -n "${NAMESPACE}" "job/${host_job}" \
  | sed -n 's/.*KMS_CONTEXT_ID): //p' | tail -1 | tr -d '[:space:]')
if [[ ! "${new_id}" =~ ^[0-9]+$ ]]; then
  echo "::error::could not read the new context id from job/${host_job}" >&2
  exit 1
fi
echo "New KMS context id: ${new_id}"

gw_values=$(mktemp)
stage_values gateway-contracts "${gw_overlay}" "${gw_values}"
gw_n=$(env_value "${gw_values}" NUM_KMS_NODES)
for ((i = 1; i <= gw_n; i++)); do
  idx=$((i - 1))
  set_env "${gw_values}" "KMS_NODE_IP_ADDRESS_${idx}" "http://kms-core-${i}-core-${i}:50001"
done
ID="${new_id}" yq -i '
  (.scDeploy.env[] | select(.name == "KMS_CONTEXT_ID")).value = strenv(ID)
' "${gw_values}"
if ! ID="${new_id}" yq -e '.scDeploy.env[] | select(.name == "KMS_CONTEXT_ID")' "${gw_values}" >/dev/null 2>&1; then
  ID="${new_id}" yq -i '.scDeploy.env += [{"name": "KMS_CONTEXT_ID", "value": strenv(ID)}]' "${gw_values}"
fi
echo "Broadcasting updateKmsContext ${new_id} in ${NAMESPACE}"
install_release "${gw_release}" "${gw_values}"
rm -f "${gw_values}"
echo "Host and gateway txs are mined. The cores still confirm the new context and activate its epoch."
