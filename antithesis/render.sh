#!/usr/bin/env bash
# Render the preview-env stack (ci/preview-env) into raw manifests for Antithesis:
# N=1 coprocessor, anvil host + gateway chains, 4 insecure KMS parties, in-cluster S3.
#
# Each release is `helm template`d with the same values files and --set flags as
# ci/preview-env/scripts/deploy/*.sh, then normalized for the Antithesis k3s cluster
# (normalize below). Startup order is kapp change-groups, one per preview-env deploy
# step, so `kapp deploy` waits the way the preview's `helm --wait` sequence does.
#
# Usage (repo root): antithesis/render.sh, then antithesis/mirror-images.sh < antithesis/images.txt
# Needs: helm, yq, jq, cast, kubectl (client only), openssl, and the hub.zama.org Helm login.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${here}/lib.sh"
cd "${here}/.."

NAMESPACE=fhevm
# fhevm images: CI tags every main commit's images with its short SHA.
FHEVM_TAG="${FHEVM_TAG:-89e9380}"
KMS_REF=v0.15.0-1 # KMS_REPO_REF / KMS_CORE_TAG in .github/workflows/preview-env-deploy.yml
# Keygen params type for host-trigger-keygen: 0 = Default (production-size), 1 = Test.
KEYGEN_PARAMS_TYPE="${KEYGEN_PARAMS_TYPE:-1}"
NB_KMS_CORE=4
COMMON_CHART=oci://hub.zama.org/ghcr/zama-zws/helm-charts/common
COMMON_CHART_VERSION=0.3.3
REDIS_CHART=oci://hub.zama.org/cgr/zama.ai/iamguarded-charts/redis
REDIS_CHART_VERSION=25.3.8
S3=http://minio:9000
# Registered on-chain as each KMS party's storage URL. Not minio:9000: host-listener
# rewrites that to a docker bridge IP (see static/s3.yaml).
KMS_STORAGE_URL=http://s3:9000/kms-public
MNEMONIC="test test test test test test test test test test test junk"

cache="${here}/.cache"
work="$(mktemp -d)"
trap 'rm -rf "${work}"' EXIT
# Rendered into ${work} and moved into place at the end, so a failed or concurrent run
# never leaves a partial manifest set behind.
out="${work}/manifests"
mkdir -p "${out}" "${cache}"

kms_repo="${cache}/kms-${KMS_REF}"
if [[ ! -d "${kms_repo}" ]]; then
  git clone --quiet --depth 1 --branch "${KMS_REF}" --filter=blob:none --sparse \
    https://github.com/zama-ai/kms.git "${kms_repo}"
  git -C "${kms_repo}" sparse-checkout set charts/kms-core ci/kube-testing ci/values
fi
kms_chart="${kms_repo}/charts/kms-core"
kms_values="${kms_repo}/ci/kube-testing/kms"

# Wallets: anvil mode derives every role from the Foundry mnemonic. HD offsets as in
# preview-env-deploy.yml: KMS tx-senders from 10, the coprocessor right after them.
wallet_key() { cast wallet private-key --mnemonic "${MNEMONIC}" --mnemonic-index "$1"; }
wallet_addr() { cast wallet address --private-key "$1"; }
kms_wallets='[]'
for i in $(seq 1 "${NB_KMS_CORE}"); do
  key=$(wallet_key $((9 + i)))
  kms_wallets=$(jq -c --argjson p "${i}" --arg a "$(wallet_addr "${key}")" --arg k "${key}" \
    '. + [{party: $p, address: $a, privateKey: $k}]' <<<"${kms_wallets}")
done
coproc_key=$(wallet_key $((10 + NB_KMS_CORE)))
coproc_wallets=$(jq -nc --arg a "$(wallet_addr "${coproc_key}")" --arg k "${coproc_key}" \
  '[{party: 1, address: $a, privateKey: $k}]')

# normalize <file> <change-group> [after-group...]
# Fit rendered manifests to the Antithesis k3s cluster: namespace everything, order with
# kapp, drop what only zws-dev has (karpenter pools, pull secrets, monitoring CRDs,
# ingress), keep total requests under the 1 CPU / 10 GiB cluster cap (memory limits
# stay, CPU limits go), and point images at the Antithesis registry.
normalize() {
  local file="$1" group="$2"
  shift 2
  # kapp takes one rule per annotation: change-rule.0, change-rule.1, ...
  local rules="{}" n=0
  for g in "$@"; do
    rules=$(jq -c --arg k "kapp.k14s.io/change-rule.${n}" --arg v "upsert after upserting ${g}" \
      '. + {($k): $v}' <<<"${rules}")
    n=$((n + 1))
  done
  GROUP="${group}" RULES="${rules}" NS="${NAMESPACE}" yq -i '
    select(.kind != null and .kind != "ServiceMonitor" and .kind != "PodMonitor"
      and .kind != "PrometheusRule" and .kind != "Ingress" and .kind != "ExternalSecret"
      and .kind != "ClusterPolicy" and .kind != "Policy"
      and .metadata.annotations["helm.sh/hook"] != "test")
    | (select(.kind != "Namespace" and .kind != "ClusterRole" and .kind != "ClusterRoleBinding")
        | .metadata.namespace) = strenv(NS)
    | .metadata.annotations["kapp.k14s.io/change-group"] = strenv(GROUP)
    | .metadata.annotations *= (strenv(RULES) | from_json)
    | del(.metadata.annotations["helm.sh/hook"], .metadata.annotations["helm.sh/hook-weight"],
        .metadata.annotations["helm.sh/hook-delete-policy"])
    | del(.spec.ttlSecondsAfterFinished)
    | (select(.kind == "Service" and .spec.type == "LoadBalancer") | .spec.type) = "ClusterIP"
    | del(.spec.storageClassName)
    | del(.spec.volumeClaimTemplates[].spec.storageClassName)
    | (select(.kind == "Deployment" or .kind == "StatefulSet" or .kind == "Job" or .kind == "DaemonSet")
        | .spec.template.spec) |= (
        del(.nodeSelector, .tolerations, .affinity, .imagePullSecrets, .topologySpreadConstraints)
        | .volumes = ((.volumes // []) | map(select(.name != "aws-lc-sysgenid")))
            + [{"name": "aws-lc-sysgenid", "configMap": {"name": "aws-lc-sysgenid"}}]
        | (.containers[], .initContainers[]) |= (
            .imagePullPolicy = "IfNotPresent"
            | del(.resources.limits.cpu)
            | .resources.requests = {"cpu": "10m", "memory": "64Mi"}
            | .env = ((.env // []) | map(select(.name != "NO_COLOR"))) + [{"name": "NO_COLOR", "value": "1"}]
            | .volumeMounts = ((.volumeMounts // []) | map(select(.name != "aws-lc-sysgenid")))
                + [{"name": "aws-lc-sysgenid", "mountPath": "/dev/sysgenid", "subPath": "sysgenid", "readOnly": true}]))
  ' "${file}"
  # Same name mapping as mirror-images.sh; the source refs are what it must copy.
  local src dst
  while read -r src; do
    dst=$(antithesis_image "${src}")
    echo "${src}" >>"${work}/images.txt"
    SRC="${src}" DST="${dst}" yq -i '
      (select(.spec.template.spec != null)
        | (.spec.template.spec.containers[], .spec.template.spec.initContainers[]))
        |= select(.image == strenv(SRC)).image = strenv(DST)
    ' "${file}"
  done < <(yq -N '.spec.template.spec | (.containers[], .initContainers[]) | .image' "${file}" |
    sort -u | grep -v -e '^---$' -e '^null$' || true)
  yq -i '(select(.spec.template.spec != null) | .spec.template.spec) |= del(.initContainers | select(. == []))' \
    "${file}"
}

# render <release> <chart> <group> <after-groups (space-separated)> -- <helm args...>
render() {
  local release="$1" chart="$2" group="$3" after="$4"
  shift 5
  local file="${out}/${group}--${release}.yaml"
  helm template "${release}" "${chart}" -n "${NAMESPACE}" "$@" >"${file}"
  # shellcheck disable=SC2086
  normalize "${file}" "${group}" ${after}
}

# static <file> <group> <after-groups>
static() {
  local file
  file="${out}/$2--$(basename "$1")"
  cp "$1" "${file}"
  # shellcheck disable=SC2086
  normalize "${file}" "$2" $3
}

# --- 01 base: namespace, S3, databases, chains, redis --------------------------------
cat >"${out}/00--namespace.yaml" <<EOF
apiVersion: v1
kind: Namespace
metadata:
  name: ${NAMESPACE}
  annotations:
    kapp.k14s.io/change-group: namespace
---
# Every Rust image (KMS, coprocessor, connector, listener) links AWS-LC, which seeds its
# RNG from CPU jitter entropy timed with RDTSC. Antithesis makes RDTSC deterministic, the
# jitter health test fails and AWS-LC calls abort() (run 1: kms-gen-keys SIGABRT on its
# first TLS key). AWS-LC skips jitter and seeds from getrandom when /dev/sysgenid exists
# (aws-lc crypto/fipsmodule/rand/entropy/entropy_sources.c, crypto/ube/vm_ube_detect.c);
# normalize mounts this 4-byte counter there in every container. It must be mmap-able.
apiVersion: v1
kind: ConfigMap
metadata:
  name: aws-lc-sysgenid
  namespace: ${NAMESPACE}
  annotations:
    kapp.k14s.io/change-group: namespace
binaryData:
  sysgenid: AAAAAA==
EOF
static "${here}/static/s3.yaml" s3 namespace
sed "s|host-contracts:TAG|host-contracts:${FHEVM_TAG}|" "${here}/static/setup-milestones.yaml" \
  >"${work}/setup-milestones.yaml"
static "${work}/setup-milestones.yaml" s3 namespace
for db in coprocessor-1:coprocessor-infra/values-postgres-coprocessor-e2e.yaml \
  listener-1:listener/values-postgres-listener-e2e.yaml \
  $(for i in $(seq 1 "${NB_KMS_CORE}"); do echo "connector-${i}:kms-connector/values-postgres-connector-e2e.yaml"; done); do
  render "postgres-${db%%:*}" "${COMMON_CHART}" base namespace -- --version "${COMMON_CHART_VERSION}" \
    -f "ci/preview-env/${db#*:}" --set-string "fullnameOverride=postgres-${db%%:*}"
done
render anvil-host charts/anvil-node base namespace -- -f ci/preview-env/host-chain/values-anvil-host-e2e.yaml
render anvil-gateway charts/anvil-node base namespace -- -f ci/preview-env/gateway-chain/values-anvil-gateway-e2e.yaml
render coprocessor-redis-1 "${REDIS_CHART}" base namespace -- --version "${REDIS_CHART_VERSION}" \
  -f ci/preview-env/coprocessor/values-coprocessor-redis-e2e.yaml --set-string fullnameOverride=coprocessor-redis-1

# --- 02 kms: 4 insecure threshold cores (kms repo kind-ci path) + kms-init --------------
peers="${work}/kms-peers.yaml"
{
  echo "kmsCore: {thresholdMode: {peersList: ["
  for i in $(seq 1 "${NB_KMS_CORE}"); do echo "  {id: ${i}, host: kms-core-${i}-core-${i}, port: 50001},"; done
  echo "]}}"
} >"${peers}"
kms_overlay="${work}/kms-overlay.yaml"
cat >"${kms_overlay}" <<EOF
kmsCore:
  image: {name: ghcr.io/zama-ai/kms/core-service-insecure, tag: "${KMS_REF}"}
  storage: {storageClassName: ""}
  # Sized for one simulated CPU, as in the KMS team's Antithesis setup on this tenant.
  thresholdMode: {tokioWorkerThreads: 1, rayonNumThreads: 4, numSessionsPreproc: 1}
kmsCoreClient:
  image: {name: ghcr.io/zama-ai/kms/core-client-insecure, tag: "${KMS_REF}"}
  num_majority: 2
  num_reconstruct: 3
kmsGenCertAndKeys: {enabled: false}
minio: {endpoint: "${S3}"}
EOF
for i in $(seq 1 "${NB_KMS_CORE}"); do
  render "kms-core-${i}" "${kms_chart}" kms "s3" -- \
    -f "${kms_values}/values-kms-test.yaml" -f "${cache}/kms/ci/values/overrides/dev-minimal.yaml" \
    -f "${peers}" -f "${kms_overlay}" \
    --set "kmsPeers.id=${i}" --set "kmsCore.thresholdMode.thresholdValue=1" \
    --set "kmsCore.publicVault.s3.prefix=PUB-p${i}" --set "kmsCore.privateVault.s3.prefix=PRIV-p${i}" \
    --set "kmsCore.backupVault.s3.prefix=BACKUP-p${i}"
done
render kms-core-init "${kms_chart}" kms-init kms -- \
  -f "${kms_values}/values-kms-service-init-kms-test.yaml" -f "${peers}" -f "${kms_overlay}"

# --- 03 kms-signers: signer addresses + CA certs from S3, for the contract deploys -----
# Preview resolves these on the runner (wallets/discover-kms-signers.mjs) and pastes them
# into the contracts values. Here a Job runs the same script and writes a ConfigMap the
# contract Jobs read with envFrom.
kubectl create configmap kms-signers-script --dry-run=client -o yaml \
  --from-file=discover-kms-signers.mjs=ci/preview-env/scripts/wallets/discover-kms-signers.mjs \
  >"${work}/kms-signers-script.yaml"
cat "${work}/kms-signers-script.yaml" - >"${work}/kms-signers.yaml" <<EOF
---
apiVersion: v1
kind: ServiceAccount
metadata: {name: kms-signers}
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata: {name: kms-signers}
rules:
  - {apiGroups: [""], resources: [configmaps], verbs: [get, create, patch, update]}
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata: {name: kms-signers}
roleRef: {apiGroup: rbac.authorization.k8s.io, kind: Role, name: kms-signers}
subjects: [{kind: ServiceAccount, name: kms-signers}]
---
apiVersion: batch/v1
kind: Job
metadata: {name: kms-signers}
spec:
  backoffLimit: 30
  template:
    spec:
      serviceAccountName: kms-signers
      restartPolicy: OnFailure
      containers:
        - name: discover
          image: ghcr.io/zama-ai/fhevm/host-contracts:${FHEVM_TAG}
          env:
            - {name: NB_KMS_CORE, value: "${NB_KMS_CORE}"}
            - {name: S3_ENDPOINT, value: "${S3}/kms-public"}
          command: [/bin/bash, -c]
          args:
            - |
              set -euo pipefail
              node /script/discover-kms-signers.mjs > /tmp/signers.json
              args=()
              for row in \$(jq -c '.[]' /tmp/signers.json); do
                idx=\$((\$(jq -r .party <<<"\${row}") - 1))
                args+=(--from-literal="KMS_SIGNER_ADDRESS_\${idx}=\$(jq -r .signer <<<"\${row}")")
                args+=(--from-literal="KMS_NODE_CA_CERT_\${idx}=\$(jq -r .caCert <<<"\${row}")")
              done
              kubectl create configmap kms-signers "\${args[@]}" --dry-run=client -o yaml | kubectl apply -f -
          volumeMounts: [{name: script, mountPath: /script}]
      volumes: [{name: script, configMap: {name: kms-signers-script}}]
EOF
static "${work}/kms-signers.yaml" kms-signers "kms kms-init"

# --- 04 contracts: gateway, host, register host chain ---------------------------------
# Contract values come from wire-contracts-values.sh, the preview's single source for the
# per-party env names and thresholds. Runtime-only inputs are stubbed: the KMS signer and
# CA cert entries are removed again and arrive via envFrom kms-signers, and the
# coprocessor bucket is the in-cluster one.
stub_bin="${work}/bin"
mkdir -p "${stub_bin}"
# The only kubectl call wire-contracts-values.sh makes is the coprocessor bucket lookup.
cat >"${stub_bin}/kubectl" <<'EOF'
#!/bin/sh
case "$*" in
  "get configmap coprocessor-1 "*) echo coproc-0 ;;
  *) echo "render.sh kubectl stub: unexpected call: kubectl $*" >&2; exit 1 ;;
esac
EOF
chmod +x "${stub_bin}/kubectl"
signer_stub=$(jq -c '[.[] | {party, signer: "0x0", caCert: "0x0"}]' <<<"${kms_wallets}")
contracts_values() {
  local target="$1" gh_out="${work}/gh-output-$1"
  : >"${gh_out}"
  PATH="${stub_bin}:${PATH}" TARGET="${target}" NB_KMS_CORE="${NB_KMS_CORE}" NB_COPROCESSOR=1 \
    NAMESPACE="${NAMESPACE}" SIGNERS_JSON="${signer_stub}" WALLETS_JSON="${kms_wallets}" \
    COPROC_WALLETS_JSON="${coproc_wallets}" S3_ENDPOINT="${KMS_STORAGE_URL}" KMS_CORE_TAG="${KMS_REF}" \
    PCR0=00 PCR1=00 PCR2=00 GITHUB_OUTPUT="${gh_out}" \
    bash ci/preview-env/scripts/deploy/wire-contracts-values.sh >/dev/null
  # The script writes to a fixed /tmp path; take a private copy right away.
  local file="${work}/values-contracts-${target}.yaml"
  cp "$(sed -n 's/^values_file=//p' "${gh_out}")" "${file}"
  BUCKET="${S3}/coproc-0" yq -i '
    del(.scDeploy.env[] | select(.name | test("^KMS_SIGNER_ADDRESS_|^KMS_NODE_CA_CERT_")))
    | (.scDeploy.env[] | select(.name == "COPROCESSOR_S3_BUCKET_URL_0")).value = strenv(BUCKET)
    | .scDeploy.envFrom += [{"configMapRef": {"name": "kms-signers"}}]
  ' "${file}"
  echo "${file}"
}
# Assigned first: set -e does not stop on a failure inside "$(...)" used as an argument.
gateway_values=$(contracts_values gateway)
host_values=$(contracts_values host)
render gateway-contracts charts/contracts gateway-contracts "base kms-signers" -- \
  -f "${gateway_values}" --set-string "scDeploy.image.tag=${FHEVM_TAG}"
render host-contracts charts/contracts host-contracts gateway-contracts -- \
  -f "${host_values}" --set-string "scDeploy.image.tag=${FHEVM_TAG}"
render gateway-add-host-chains charts/contracts add-host-chains host-contracts -- \
  -f ci/preview-env/gateway-chain/values-gateway-add-host-chains-e2e.yaml \
  --set-string "scDeploy.image.tag=${FHEVM_TAG}"

# --- 05 app: listener, coprocessor, poller, keygen redirects, kms-connectors ----------
# Replaces coprocessor-infra (Crossplane bucket + IRSA ServiceAccount + ConfigMap).
cat >"${work}/coprocessor-infra.yaml" <<EOF
apiVersion: v1
kind: ServiceAccount
metadata: {name: coprocessor-1}
---
apiVersion: v1
kind: ConfigMap
metadata: {name: coprocessor-1}
data: {S3_BUCKET_NAME: coproc-0}
EOF
static "${work}/coprocessor-infra.yaml" app add-host-chains
render listener-1 charts/listener app add-host-chains -- -f ci/preview-env/listener/values-listener-e2e.yaml \
  --set-string "image.tag=${FHEVM_TAG}" \
  --set-string "env[0].value=postgresql://zama:zama@postgres-listener-1:5432/listener?sslmode=disable" \
  --set-string "env[1].value=redis://coprocessor-redis-1-master:6379"
coproc_images=()
for c in dbMigration:db-migration gwListener:gw-listener hostListenerShared:host-listener \
  hostListenerPollerShared:host-listener hostListenerCatchupOnlyShared:host-listener \
  hostListenerConsumerShared:host-listener snsWorker:sns-worker tfheWorker:tfhe-worker \
  txSender:tx-sender zkProofWorker:zkproof-worker; do
  coproc_images+=(--set-string "${c%%:*}.image.tag=${FHEVM_TAG}")
done
coproc_accounts=()
for c in hostListenerShared hostListenerPollerShared hostListenerCatchupOnlyShared \
  hostListenerConsumerShared tfheWorker zkProofWorker snsWorker; do
  coproc_accounts+=(--set-string "${c}.serviceAccountName=coprocessor-1")
done
# As deploy-coprocessor.sh party_common, without the coprocessor-proxy / prf-service-fake
# pair (input-proof proxy path; not needed until the workload uses it).
render coprocessor-1 charts/coprocessor app add-host-chains -- \
  -f ci/preview-env/coprocessor/values-coprocessor-e2e.yaml --set-string fullnameOverride=coprocessor-1 \
  --set-string "commonConfig.databaseEndpoint.value=postgres-coprocessor-1:5432" \
  "${coproc_accounts[@]}" \
  --set "snsWorker.config.s3BucketName.valueFrom.configMapKeyRef.name=coprocessor-1" \
  --set-string "hostListenerConsumerShared.config.brokerUrl.value=redis://coprocessor-redis-1-master:6379" \
  --set-string "txSender.wallet.secret.value=${coproc_key}" \
  --set-string "snsWorker.wallet.secret.value=${coproc_key}" \
  --set-string "consensusDetector.wallet.secret.value=${coproc_key}" \
  "${coproc_images[@]}"
render coprocessor-poller-1 charts/coprocessor app add-host-chains -- \
  -f ci/preview-env/coprocessor/values-coprocessor-poller-e2e.yaml \
  --set-string "commonConfig.databaseEndpoint.value=postgres-coprocessor-1:5432" \
  --set-string "hostListenerPollerShared.serviceAccountName=coprocessor-1" \
  --set-string "hostListenerPollerShared.image.tag=${FHEVM_TAG}"
# Preview's IRSA role gave the coprocessor S3 access; here it is MinIO with static keys.
for f in "${out}"/app--coprocessor-1.yaml "${out}"/app--coprocessor-poller-1.yaml; do
  S3="${S3}" yq -i '
    (select(.spec.template.spec != null) | (.spec.template.spec.containers[], .spec.template.spec.initContainers[]))
      .env += [
        {"name": "AWS_ENDPOINT_URL", "value": strenv(S3)},
        {"name": "AWS_ACCESS_KEY_ID", "value": "kms-access-key-id"},
        {"name": "AWS_SECRET_ACCESS_KEY", "value": "kms-secret-access-key"},
        {"name": "AWS_REGION", "value": "eu-west-1"}]
  ' "${f}"
done

# Insecure keygen: the connector's kms-worker calls the secure keygen RPCs; this proxy
# rewrites them to the insecure ones (test-suite/fhevm/scripts/lib/kms-test-keygen-proxy.cjs).
# Preview removes it after keygen (bypass-kms-keygen-redirect.sh); here it stays in the
# decryption path, single-threaded.
for i in $(seq 1 "${NB_KMS_CORE}"); do
  render "kms-insecure-keygen-redirect-${i}" "${COMMON_CHART}" app add-host-chains -- \
    --version "${COMMON_CHART_VERSION}" -f ci/preview-env/kms-connector/values-kms-insecure-keygen-redirect-e2e.yaml \
    --set-string "fullnameOverride=kms-insecure-keygen-redirect-${i}" \
    --set-file "configmapFiles.files.kms-test-keygen-proxy\.cjs=test-suite/fhevm/scripts/lib/kms-test-keygen-proxy.cjs" \
    --set-json "args=[\"http://kms-core-${i}-core-${i}:50100\"]"
done

# kms-connector: contract addresses exist only after the contract Jobs. Rendered as
# $(VAR) references that Kubernetes expands from env vars read from the address
# ConfigMaps, including the ACL address inside KMS_CONNECTOR_HOST_CHAINS.
# Self-signed test cert for the connector proxy's HTTPS port, as deploy-kms-connector.sh
# makes at deploy time; LibreSSL needs named_curve for Go to parse the key.
sans=""
for i in $(seq 1 "${NB_KMS_CORE}"); do
  s="kms-connector-${i}-kms-connector-proxy"
  sans="${sans}DNS:${s},DNS:${s}.${NAMESPACE}.svc,DNS:${s}.${NAMESPACE}.svc.cluster.local,"
done
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -pkeyopt ec_param_enc:named_curve \
  -nodes -days 3650 -keyout "${work}/tls.key" -out "${work}/tls.crt" \
  -subj "/CN=kms-connector-1-kms-connector-proxy" -addext "subjectAltName=${sans%,}" \
  -addext "basicConstraints=critical,CA:TRUE" -addext "keyUsage=critical,digitalSignature,keyCertSign" \
  -addext "extendedKeyUsage=serverAuth" 2>"${work}/openssl.log" || { cat "${work}/openssl.log" >&2; exit 1; }
kms_wallet() { jq -r --argjson p "$1" ".[] | select(.party == \$p) | .$2" <<<"${kms_wallets}"; }
kubectl create secret tls kms-connector-proxy-tls --cert="${work}/tls.crt" --key="${work}/tls.key" \
  --dry-run=client -o yaml >"${work}/kms-connector-secrets.yaml"
for i in $(seq 1 "${NB_KMS_CORE}"); do
  key=$(kms_wallet "${i}" privateKey)
  echo "---" >>"${work}/kms-connector-secrets.yaml"
  kubectl create secret generic "kms-connector-tx-sender-${i}" --dry-run=client -o yaml \
    --from-literal=kms-wallet="${key}" >>"${work}/kms-connector-secrets.yaml"
done
static "${work}/kms-connector-secrets.yaml" app add-host-chains
connector_values="${work}/values-kms-connector.yaml"
cp ci/preview-env/kms-connector/values-kms-connector-e2e.yaml "${connector_values}"
yq -i '.commonConfig.hostChains.ethereum.aclAddress = "$(ACL_ADDRESS)"' "${connector_values}"
for i in $(seq 1 "${NB_KMS_CORE}"); do
  tx_sender=$(kms_wallet "${i}" address)
  connector_images=()
  for c in DbMigration:db-migration GwListener:gw-listener KmsWorker:kms-worker TxSender:tx-sender \
    Endpoint:endpoint Proxy:proxy; do
    connector_images+=(--set-string "kmsConnector${c%%:*}.image.name=ghcr.io/zama-ai/fhevm/kms-connector/${c#*:}"
      --set-string "kmsConnector${c%%:*}.image.tag=${FHEVM_TAG}")
  done
  render "kms-connector-${i}" charts/kms-connector app add-host-chains -- -f "${connector_values}" \
    --set-string 'commonConfig.gatewayContractAddresses.decryption=$(DECRYPTION_ADDRESS)' \
    --set-string 'commonConfig.gatewayContractAddresses.gatewayConfig=$(GATEWAY_CONFIG_ADDRESS)' \
    --set-string 'commonConfig.ethereumContractAddresses.kmsGeneration=$(KMS_GENERATION_ADDRESS)' \
    --set-string 'commonConfig.ethereumContractAddresses.protocolConfig=$(PROTOCOL_CONFIG_ADDRESS)' \
    --set-string 'commonConfig.ethereumContractAddresses.acl=$(ACL_ADDRESS)' \
    --set-string "commonConfig.databaseUrl=postgresql://zama:zama@postgres-connector-${i}:5432/connector" \
    --set-string "kmsConnectorKmsWorker.config.kmsCoreEndpoints=http://kms-insecure-keygen-redirect-${i}:3000" \
    --set-string "kmsConnectorTxSender.wallet.secret.name=kms-connector-tx-sender-${i}" \
    --set-string "kmsConnectorProxy.txSenderAddress=${tx_sender}" \
    "${connector_images[@]}"
  # $(VAR) only expands from variables defined earlier in the list, so these go first.
  yq -i '
    (select(.spec.template.spec != null) | (.spec.template.spec.containers[], .spec.template.spec.initContainers[]))
      .env |= [
        {"name": "DECRYPTION_ADDRESS", "valueFrom": {"configMapKeyRef": {"name": "gw-sc-addresses", "key": "decryption.address"}}},
        {"name": "GATEWAY_CONFIG_ADDRESS", "valueFrom": {"configMapKeyRef": {"name": "gw-sc-addresses", "key": "gateway_config.address"}}},
        {"name": "KMS_GENERATION_ADDRESS", "valueFrom": {"configMapKeyRef": {"name": "host-sc-addresses", "key": "kms_generation.address"}}},
        {"name": "PROTOCOL_CONFIG_ADDRESS", "valueFrom": {"configMapKeyRef": {"name": "host-sc-addresses", "key": "protocol_config.address"}}},
        {"name": "ACL_ADDRESS", "valueFrom": {"configMapKeyRef": {"name": "host-sc-addresses", "key": "acl.address"}}}
      ] + (. // [])
  ' "${out}/app--kms-connector-${i}.yaml"
done

# --- 06 keygen: trigger key + CRS generation and wait for ActivateKey/ActivateCrs -------
keygen_values="${work}/values-host-trigger-keygen.yaml"
cp ci/preview-env/host-chain/values-host-trigger-keygen-e2e.yaml "${keygen_values}"
P="${KEYGEN_PARAMS_TYPE}" yq -i '(.scDeploy.env[] | select(.name == "KEYGEN_PARAMS_TYPE")).value = strenv(P)' \
  "${keygen_values}"
render host-trigger-keygen charts/contracts keygen app -- -f "${keygen_values}" \
  --set-string "scDeploy.image.tag=${FHEVM_TAG}"

# --- 07 ready: the coprocessor holds the activated key (e2e waitForCoprocessorKeyMaterial)
# kapp deploy finishes when this Job completes, which is Antithesis's ready signal.
cat >"${work}/keys-ready.yaml" <<'EOF'
apiVersion: batch/v1
kind: Job
metadata: {name: coprocessor-keys-ready}
spec:
  backoffLimit: 30
  template:
    spec:
      restartPolicy: OnFailure
      containers:
        - name: check
          image: docker.io/library/postgres:17-alpine
          env:
            - {name: PGPASSWORD, value: zama}
          command: [/bin/sh, -c]
          args:
            - |
              q="SELECT COUNT(*), COUNT(sns_pk) FROM (SELECT sns_pk FROM keys ORDER BY sequence_number DESC LIMIT 1) k"
              until [ "$(psql -h postgres-coprocessor-1 -U zama -d fhevm_e2e -tAc "$q")" = "1|1" ]; do
                echo "waiting for coprocessor key material"; sleep 5
              done
              echo "coprocessor key material present"
EOF
static "${work}/keys-ready.yaml" ready keygen

# Every image anywhere in the output must come from the Antithesis registry.
stray=$(yq -N '.. | select(tag == "!!map" and has("image")) | .image | select(tag == "!!str")' "${out}"/*.yaml |
  grep -v -e "^${ANTITHESIS_REGISTRY}/" -e '^---$' || true)
if [[ -n "${stray}" ]]; then
  echo "images not rewritten to ${ANTITHESIS_REGISTRY}:" >&2
  echo "${stray}" >&2
  exit 1
fi
sort -u "${work}/images.txt" >"${here}/images.txt"
rm -rf "${here}/config/manifests"
mkdir -p "${here}/config"
mv "${out}" "${here}/config/manifests"
echo "rendered $(find "${here}/config/manifests" -name '*.yaml' | wc -l | tr -d ' ') files;" \
  "images to mirror: antithesis/images.txt ($(wc -l <"${here}/images.txt" | tr -d ' '))"
