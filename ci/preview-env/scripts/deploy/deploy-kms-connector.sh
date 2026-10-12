#!/usr/bin/env bash
# Install one kms-connector release per KMS party.
# Env: NAMESPACE, NB_KMS_CORE, DEPLOY_POLYGON, OBSERVABILITY, OTLP_ENDPOINT,
# KMS_CONNECTOR_CHART, TAGS_JSON.
set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
# shellcheck source-path=SCRIPTDIR
# shellcheck source=../lib.sh
source "${script_dir}/../lib.sh"

# Must match charts/kms-connector/templates/_helpers.tpl (kmsConnectorProxyName:
# "<release>-kms-connector-proxy") and kmsConnectorProxy.ports.https (8443).
proxy_service() { echo "kms-connector-${1}-kms-connector-proxy"; }
proxy_port=8443
tls_secret=kms-connector-proxy-tls          # kmsConnectorProxy.tls.secretName in the overlay
api_key_secret=kms-connector-proxy          # kmsConnectorProxy.apiKeyDigest.secret.name (chart default)
api_key=fhevm-e2e-kms-connector-api-key     # test literal; relayer-http reads it from ${api_key_secret}
http_configmap=kms-connector-http           # read by the test-suite overlays

decryption=$(kubectl get configmap gw-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.decryption\.address}')
gateway_config=$(kubectl get configmap gw-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.gateway_config\.address}')
kms_generation=$(kubectl get configmap host-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.kms_generation\.address}')
protocol_config=$(kubectl get configmap host-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.protocol_config\.address}')
acl=$(kubectl get configmap host-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.acl\.address}')
require_nonempty "${decryption}" "gw-sc-addresses[decryption.address] is empty"
require_nonempty "${gateway_config}" "gw-sc-addresses[gateway_config.address] is empty"
require_nonempty "${kms_generation}" "host-sc-addresses[kms_generation.address] is empty"
require_nonempty "${protocol_config}" "host-sc-addresses[protocol_config.address] is empty"
require_nonempty "${acl}" "host-sc-addresses[acl.address] is empty"

# Sets host chain ACL contract address in commonConfig.hostChains.
set_host_chain_acl() {
  # $1 values file, $2 hostChains entry name, $3 ACL address
  NAME="$2" ACL="$3" yq -i \
    '.commonConfig.hostChains[strenv(NAME)].aclAddress = strenv(ACL)' "$1"
}
kms_values=$(mktemp --suffix=.yaml)
cp ci/preview-env/kms-connector/values-kms-connector-e2e.yaml "${kms_values}"
set_host_chain_acl "${kms_values}" ethereum "${acl}"

polygon_args=()
if [[ "${DEPLOY_POLYGON}" == "true" ]]; then
  polygon_acl=$(kubectl get configmap polygon-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.acl\.address}')
  require_nonempty "${polygon_acl}" "polygon-sc-addresses[acl.address] is empty"
  polygon_values=$(mktemp --suffix=.yaml)
  cp ci/preview-env/kms-connector/values-kms-connector-polygon-e2e.yaml "${polygon_values}"
  set_host_chain_acl "${polygon_values}" ethereum "${acl}"
  set_host_chain_acl "${polygon_values}" polygon "${polygon_acl}"
  polygon_args=(-f "${polygon_values}")
fi

tracing=()
if [[ "${OBSERVABILITY}" == "true" ]]; then
  tracing=(
    --set "commonConfig.tracing.enabled=true"
    --set-string "commonConfig.tracing.endpoint=${OTLP_ENDPOINT}"
  )
fi

######################################################################
# Proxy TLS: one test CA and one server certificate per namespace, shared
# by every party's proxy. The chart only mounts an existing
# `kubernetes.io/tls` Secret, so it is created here, before the first
# release installs. The SANs cover each party's proxy Service name (what
# the e2e suite and relayer-http dial). Clients trust ca.crt. The server
# certificate is not a CA: rustls clients such as relayer-http reject a CA
# certificate served as a server certificate (CaUsedAsEndEntity).
######################################################################
if kubectl get secret "${tls_secret}" -n "${NAMESPACE}" >/dev/null 2>&1; then
  echo "Secret ${tls_secret} already exists in ${NAMESPACE}; keeping it."
else
  sans="DNS:$(proxy_service 1)"
  for i in $(seq 2 "${NB_KMS_CORE}"); do
    sans="${sans},DNS:$(proxy_service "${i}")"
  done
  # Also match the fully-qualified in-cluster forms in case a client dials them.
  fq=""
  for i in $(seq 1 "${NB_KMS_CORE}"); do
    fq="${fq},DNS:$(proxy_service "${i}").${NAMESPACE}.svc,DNS:$(proxy_service "${i}").${NAMESPACE}.svc.cluster.local"
  done
  sans="${sans}${fq}"
  tls_dir=$(mktemp -d)
  openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes -days 30 \
    -keyout "${tls_dir}/ca.key" -out "${tls_dir}/ca.crt" -subj "/CN=kms-connector-proxy test CA" \
    -addext "basicConstraints=critical,CA:TRUE" \
    -addext "keyUsage=critical,keyCertSign" 2>/dev/null
  openssl req -new -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes \
    -keyout "${tls_dir}/tls.key" -subj "/CN=$(proxy_service 1)" -out "${tls_dir}/tls.csr" 2>/dev/null
  printf '%s\n' "subjectAltName=${sans}" "basicConstraints=critical,CA:FALSE" \
    "keyUsage=critical,digitalSignature" "extendedKeyUsage=serverAuth" > "${tls_dir}/tls.ext"
  openssl x509 -req -in "${tls_dir}/tls.csr" -CA "${tls_dir}/ca.crt" -CAkey "${tls_dir}/ca.key" \
    -set_serial "0x$(openssl rand -hex 16)" -days 30 -extfile "${tls_dir}/tls.ext" \
    -out "${tls_dir}/tls.crt" 2>/dev/null
  kubectl create secret generic "${tls_secret}" -n "${NAMESPACE}" --type=kubernetes.io/tls \
    --from-file=tls.crt="${tls_dir}/tls.crt" --from-file=tls.key="${tls_dir}/tls.key" \
    --from-file=ca.crt="${tls_dir}/ca.crt"
  rm -rf "${tls_dir}"
  echo "Created ${tls_secret} (SANs: ${sans})"
fi

######################################################################
# Proxy API key: the proxies check the digest, relayer-http sends the key.
######################################################################
kubectl create secret generic "${api_key_secret}" -n "${NAMESPACE}" \
  --from-literal=api-key="${api_key}" \
  --from-literal=api-key-digest="0x$(printf '%s' "${api_key}" | sha256sum | cut -d' ' -f1)" \
  --dry-run=client -o yaml | kubectl apply -f -

######################################################################
# What the e2e suite needs to exercise the HTTP decryption path (see
# test-suite/e2e/test/sdk/connector/connectorHttp.ts), published as a
# ConfigMap so the static test-suite overlays can configMapKeyRef it
# instead of being yq-patched per party count:
#   endpoint-urls  KMS_CONNECTOR_ENDPOINT_URLS, one https proxy URL per party
#   kms-threshold  KMS_THRESHOLD: MPC t, quorum is 2t+1 (kms_t in lib.sh)
#   ca.crt         the proxies' CA, for NODE_EXTRA_CA_CERTS
# The API key itself is a fixed test literal in the overlays.
######################################################################
urls="https://$(proxy_service 1):${proxy_port}"
for i in $(seq 2 "${NB_KMS_CORE}"); do
  urls="${urls},https://$(proxy_service "${i}"):${proxy_port}"
done
threshold=$(kms_t "${NB_KMS_CORE}")
ca=$(kubectl get secret "${tls_secret}" -n "${NAMESPACE}" -o jsonpath='{.data.ca\.crt}')
require_nonempty "${ca}" "${tls_secret} has no ca.crt: it predates the proxy CA; destroy the preview and deploy it again"
ca_file=$(mktemp)
base64 -d <<<"${ca}" > "${ca_file}"
kubectl create configmap "${http_configmap}" -n "${NAMESPACE}" \
  --from-literal=endpoint-urls="${urls}" \
  --from-literal=kms-threshold="${threshold}" \
  --from-file=ca.crt="${ca_file}" \
  --dry-run=client -o yaml | kubectl apply -f -
rm -f "${ca_file}"
echo "Connector HTTP: urls=${urls} threshold=${threshold}"

for i in $(seq 1 "${NB_KMS_CORE}"); do
  helm upgrade --install "kms-connector-${i}" "${KMS_CONNECTOR_CHART}" \
    -n "${NAMESPACE}" -f "${kms_values}" \
    "${polygon_args[@]}" \
    --set-string "commonConfig.gatewayContractAddresses.decryption=${decryption}" \
    --set-string "commonConfig.gatewayContractAddresses.gatewayConfig=${gateway_config}" \
    --set-string "commonConfig.ethereumContractAddresses.kmsGeneration=${kms_generation}" \
    --set-string "commonConfig.ethereumContractAddresses.protocolConfig=${protocol_config}" \
    --set-string "commonConfig.databaseUrl=postgresql://zama:zama@postgres-connector-${i}:5432/connector" \
    --set-string "kmsConnectorKmsWorker.config.kmsCoreEndpoints=http://kms-core-${i}-core-${i}:50100" \
    --set-string "kmsConnectorTxSender.wallet.secret.name=kms-connector-tx-sender-${i}" \
    --set-string "kmsConnectorDbMigration.image.name=hub.zama.org/ghcr/zama-ai/fhevm/kms-connector/db-migration" \
    --set-string "kmsConnectorDbMigration.image.tag=$(jq -r .kms_connector_db_migration <<<"${TAGS_JSON}")" \
    --set-string "kmsConnectorGwListener.image.name=hub.zama.org/ghcr/zama-ai/fhevm/kms-connector/gw-listener" \
    --set-string "kmsConnectorGwListener.image.tag=$(jq -r .kms_connector_gw_listener <<<"${TAGS_JSON}")" \
    --set-string "kmsConnectorKmsWorker.image.name=hub.zama.org/ghcr/zama-ai/fhevm/kms-connector/kms-worker" \
    --set-string "kmsConnectorKmsWorker.image.tag=$(jq -r .kms_connector_kms_worker <<<"${TAGS_JSON}")" \
    --set-string "kmsConnectorTxSender.image.name=hub.zama.org/ghcr/zama-ai/fhevm/kms-connector/tx-sender" \
    --set-string "kmsConnectorTxSender.image.tag=$(jq -r .kms_connector_tx_sender <<<"${TAGS_JSON}")" \
    --set-string "kmsConnectorEndpoint.image.name=hub.zama.org/ghcr/zama-ai/fhevm/kms-connector/endpoint" \
    --set-string "kmsConnectorEndpoint.image.tag=$(jq -r .kms_connector_endpoint <<<"${TAGS_JSON}")" \
    --set-string "kmsConnectorProxy.image.name=hub.zama.org/ghcr/zama-ai/fhevm/kms-connector/proxy" \
    --set-string "kmsConnectorProxy.image.tag=$(jq -r .kms_connector_proxy <<<"${TAGS_JSON}")" \
    "${tracing[@]}"
done
sleep 60
