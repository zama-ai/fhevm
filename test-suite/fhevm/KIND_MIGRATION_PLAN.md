# Kind Migration Plan: test-suite/fhevm

This document outlines a migration plan from the current docker-compose based test infrastructure to Kind (Kubernetes in Docker).

## Executive Summary

The existing infrastructure has significant Helm chart coverage in `charts/`, making this migration more about orchestration than resource creation. The main work involves:
1. Creating a Kind cluster configuration
2. Adapting the `fhevm-cli` to use `helm` and `kubectl` instead of `docker-compose`
3. Creating missing Helm charts (MinIO, KMS Core, Relayer, Test Suite)
4. Implementing deployment sequencing in Kubernetes
5. Creating CI orchestration scripts (following KMS patterns)
6. Adding GitHub Actions workflow for automated Kind testing

## Reference Implementation

The KMS repository (`../kms/ci/kube-testing/`) contains a working Kind testing implementation that serves as the reference for this migration. Key patterns to adopt:

| Pattern | KMS Location | Description |
|---------|--------------|-------------|
| Setup Script | `scripts/setup_kms_in_kind.sh` | Main setup with CLI flags |
| CI Manager | `scripts/manage_kind_setup.sh` | Background execution + log monitoring |
| Kind Config | `infra/kind-config.yaml` | Simplified cluster config |
| Values Files | `kms/values-kms-test.yaml` | Helm values with `<namespace>` placeholders |
| Port Forwarding | In setup script | kubectl port-forward instead of NodePort |

## Current State Analysis

### Existing Helm Charts (Ready to Use)
| Service | Chart Location | Status |
|---------|---------------|--------|
| Coprocessor (all 8 services) | `charts/coprocessor/` | Ready |
| KMS Connector (all 4 services) | `charts/kms-connector/` | Ready |
| Anvil Node (host/gateway) | `charts/anvil-node/` | Ready |
| Contracts Deployment | `charts/contracts/` | Ready |
| SQL Exporter | `charts/coprocessor-sql-exporter/` | Ready |

### External Charts (Check Before Creating)
| Service | Potential Source | Action |
|---------|-----------------|--------|
| KMS Core | `../kms/charts/kms-core/` | **Check if exists and reuse** |
| Relayer | Separate relayer repo | **Check if chart exists** |
| MinIO | Bitnami `minio` chart | Use external chart |
| PostgreSQL | Bitnami `postgresql` chart | Use external chart |
| LocalStack | `localstack-charts/localstack` | Alternative to MinIO (used by KMS) |

### Missing Helm Charts (Create if not found externally)
| Service | Docker Compose File | Priority |
|---------|-------------------|----------|
| KMS Core | `core-docker-compose.yml` | High (check KMS repo first) |
| Relayer | `relayer-docker-compose.yml` | High |
| Test Suite | `test-suite-docker-compose.yml` | Medium |

## Migration Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         KIND CLUSTER                                    │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │                      fhevm-local namespace                         │ │
│  │                                                                    │ │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐               │ │
│  │  │   MinIO     │  │  PostgreSQL │  │  KMS Core   │               │ │
│  │  │  (Bitnami)  │  │  (Bitnami)  │  │(from KMS repo)│              │ │
│  │  └─────────────┘  └─────────────┘  └─────────────┘               │ │
│  │         │                │                │                        │ │
│  │  ┌──────┴────────────────┴────────────────┴──────┐               │ │
│  │  │           Shared Dependencies                  │               │ │
│  │  └───────────────────────────────────────────────┘               │ │
│  │         │                              │                          │ │
│  │  ┌──────┴──────┐              ┌───────┴───────┐                  │ │
│  │  │ Host Chain  │              │ Gateway Chain │                  │ │
│  │  │ (anvil-node)│              │ (anvil-node)  │                  │ │
│  │  └─────────────┘              └───────────────┘                  │ │
│  │         │                              │                          │ │
│  │  ┌──────┴──────┐              ┌───────┴───────┐                  │ │
│  │  │Host Contracts│             │Gateway Contracts│                │ │
│  │  │  (contracts) │             │  (contracts)   │                 │ │
│  │  └─────────────┘              └───────────────┘                  │ │
│  │         │                              │                          │ │
│  │  ┌──────┴──────────────────────────────┴──────┐                  │ │
│  │  │              Coprocessor                    │                  │ │
│  │  │  (host-listener, gw-listener, tfhe-worker, │                  │ │
│  │  │   zkproof-worker, sns-worker, tx-sender)   │                  │ │
│  │  └────────────────────────────────────────────┘                  │ │
│  │                        │                                          │ │
│  │  ┌─────────────────────┴─────────────────────┐                   │ │
│  │  │           KMS Connector                    │                   │ │
│  │  │  (gw-listener, kms-worker, tx-sender)     │                   │ │
│  │  └───────────────────────────────────────────┘                   │ │
│  │                        │                                          │ │
│  │  ┌─────────────────────┴─────────────────────┐                   │ │
│  │  │              Relayer                       │                   │ │
│  │  └───────────────────────────────────────────┘                   │ │
│  │                        │                                          │ │
│  │  ┌─────────────────────┴─────────────────────┐                   │ │
│  │  │           Test Suite Pod                   │                   │ │
│  │  └───────────────────────────────────────────┘                   │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                                                                         │
│  Port Forwards (kubectl port-forward):                                  │
│    localhost:8545  → host-anvil-node:8545                              │
│    localhost:8546  → gateway-anvil-node:8546                           │
│    localhost:3000  → relayer:3000                                      │
│    localhost:9000  → minio:9000                                        │
└─────────────────────────────────────────────────────────────────────────┘
```

## Phase 0: Reference Implementation Study

Before starting implementation, study the KMS kind-testing workflow:

### 0.1 Analyze KMS Implementation

```bash
# Review the KMS kind-testing structure
ls -la ../kms/ci/kube-testing/
├── scripts/
│   ├── setup_kms_in_kind.sh      # Main setup script
│   └── manage_kind_setup.sh      # CI orchestration wrapper
├── infra/
│   ├── kind-config.yaml          # Kind cluster config
│   └── localstack-s3-values.yaml # S3-compatible storage
└── kms/
    └── values-kms-test.yaml      # KMS Core Helm values
```

### 0.2 Key Patterns to Adopt

1. **CLI Flags Pattern** (from `setup_kms_in_kind.sh`):
```bash
--namespace <name>      # Kubernetes namespace
--deployment-type <t>   # centralized|threshold
--cleanup               # Cleanup before setup
--build                 # Build and load images locally
--local                 # Interactive mode with resource prompts
--collect-logs          # Collect logs for CI
```

2. **CI Management Pattern** (from `manage_kind_setup.sh`):
```bash
# Run setup in background, monitor for completion
./setup_fhevm_in_kind.sh "$@" > "${LOG_FILE}" 2>&1 &
while ! grep -q "Stack ready" "${LOG_FILE}"; do sleep 5; done
```

3. **Resource Reservation** (from Kind config):
```yaml
kubeletExtraArgs:
  system-reserved: memory=1Gi,cpu=1
```

4. **Namespace Placeholder Pattern**:
```yaml
# In values files, use <namespace> placeholder
serviceName: "kms-core-<namespace>"
# Replace at runtime with sed
sed "s/<namespace>/${NAMESPACE}/g" values.yaml
```

### 0.3 Check Existing Charts

- [ ] Check `../kms/charts/kms-core/` for KMS Core chart
- [ ] Check if relayer has its own repo with charts
- [ ] Verify Bitnami chart versions for MinIO and PostgreSQL

## Phase 1: Foundation Setup

### 1.1 Kind Cluster Configuration (Simplified - Following KMS Pattern)

Create `kind-config.yaml` (using port-forward instead of NodePort):

```yaml
# kind-config.yaml
# Simplified config - services accessed via kubectl port-forward
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
name: fhevm-local
nodes:
  - role: control-plane
    kubeadmConfigPatches:
      - |
        kind: InitConfiguration
        nodeRegistration:
          kubeletExtraArgs:
            system-reserved: memory=2Gi,cpu=2
  - role: worker
    kubeadmConfigPatches:
      - |
        kind: JoinConfiguration
        nodeRegistration:
          kubeletExtraArgs:
            system-reserved: memory=2Gi,cpu=2
  - role: worker
    kubeadmConfigPatches:
      - |
        kind: JoinConfiguration
        nodeRegistration:
          kubeletExtraArgs:
            system-reserved: memory=2Gi,cpu=2
```

### 1.2 Main Setup Script (Following KMS Pattern)

Create `scripts/setup_fhevm_in_kind.sh`:

```bash
#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

# Default values
NAMESPACE="fhevm-local"
CLUSTER_NAME="fhevm-local"
CLEANUP=false
BUILD=false
LOCAL_MODE=false
COLLECT_LOGS=false

# Image versions (can be overridden)
COPROCESSOR_VERSION="${COPROCESSOR_VERSION:-latest}"
KMS_CONNECTOR_VERSION="${KMS_CONNECTOR_VERSION:-latest}"
RELAYER_VERSION="${RELAYER_VERSION:-latest}"

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Options:
    --namespace <name>      Kubernetes namespace (default: fhevm-local)
    --cleanup               Cleanup existing deployment before setup
    --build                 Build and load Docker images locally
    --local                 Run in local mode (interactive resource adjustment)
    --collect-logs          Collect logs from pods (for CI)
    -h, --help              Show this help message

Examples:
    # Basic setup with pre-built images
    $(basename "$0")

    # Setup with local builds
    $(basename "$0") --build

    # CI setup with cleanup and log collection
    $(basename "$0") --cleanup --collect-logs
EOF
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --namespace) NAMESPACE="$2"; shift 2 ;;
            --cleanup) CLEANUP=true; shift ;;
            --build) BUILD=true; shift ;;
            --local) LOCAL_MODE=true; shift ;;
            --collect-logs) COLLECT_LOGS=true; shift ;;
            -h|--help) usage; exit 0 ;;
            *) echo "Unknown option: $1"; usage; exit 1 ;;
        esac
    done
}

check_prerequisites() {
    local missing=()
    command -v kubectl &>/dev/null || missing+=("kubectl")
    command -v helm &>/dev/null || missing+=("helm")
    command -v kind &>/dev/null || missing+=("kind")
    command -v docker &>/dev/null || missing+=("docker")

    if [[ ${#missing[@]} -gt 0 ]]; then
        echo "ERROR: Missing required tools: ${missing[*]}"
        exit 1
    fi
}

setup_kind_cluster() {
    if kind get clusters | grep -q "^${CLUSTER_NAME}$"; then
        if [[ "$CLEANUP" == "true" ]]; then
            echo "Deleting existing cluster..."
            kind delete cluster --name "${CLUSTER_NAME}"
        else
            echo "Cluster ${CLUSTER_NAME} already exists, reusing..."
            return 0
        fi
    fi

    echo "Creating Kind cluster..."
    kind create cluster --name "${CLUSTER_NAME}" --config "${SCRIPT_DIR}/../kind-config.yaml"
}

setup_namespace() {
    echo "Setting up namespace ${NAMESPACE}..."
    kubectl create namespace "${NAMESPACE}" --dry-run=client -o yaml | kubectl apply -f -
}

setup_registry_credentials() {
    echo "Setting up registry credentials..."
    if [[ -n "${GITHUB_TOKEN:-}" ]]; then
        kubectl create secret docker-registry ghcr-creds \
            --namespace "${NAMESPACE}" \
            --docker-server=ghcr.io \
            --docker-username="${GITHUB_ACTOR:-github}" \
            --docker-password="${GITHUB_TOKEN}" \
            --dry-run=client -o yaml | kubectl apply -f -
    fi
}

build_and_load_images() {
    if [[ "$BUILD" != "true" ]]; then
        return 0
    fi

    echo "Building and loading images into Kind..."

    # Build coprocessor
    echo "Building coprocessor..."
    docker build -t "ghcr.io/zama-ai/fhevm-coprocessor:local" \
        -f "${REPO_ROOT}/coprocessor/fhevm-engine/Dockerfile.workspace" \
        "${REPO_ROOT}/coprocessor/fhevm-engine"
    kind load docker-image "ghcr.io/zama-ai/fhevm-coprocessor:local" --name "${CLUSTER_NAME}"

    # Build kms-connector
    echo "Building kms-connector..."
    docker build -t "ghcr.io/zama-ai/kms-connector:local" \
        "${REPO_ROOT}/kms-connector"
    kind load docker-image "ghcr.io/zama-ai/kms-connector:local" --name "${CLUSTER_NAME}"
}

deploy_infrastructure() {
    echo "Deploying infrastructure (MinIO, PostgreSQL)..."

    # Add Bitnami repo
    helm repo add bitnami https://charts.bitnami.com/bitnami
    helm repo update

    # Deploy MinIO
    helm upgrade --install minio bitnami/minio \
        --namespace "${NAMESPACE}" \
        -f "${SCRIPT_DIR}/../env/kind/minio-values.yaml" \
        --wait --timeout 5m

    # Deploy PostgreSQL
    helm upgrade --install postgresql bitnami/postgresql \
        --namespace "${NAMESPACE}" \
        -f "${SCRIPT_DIR}/../env/kind/postgresql-values.yaml" \
        --wait --timeout 5m
}

deploy_kms_core() {
    echo "Deploying KMS Core..."

    # Use KMS Core chart from KMS repo if available
    local kms_chart="${REPO_ROOT}/../kms/charts/kms-core"
    if [[ -d "$kms_chart" ]]; then
        echo "Using KMS Core chart from KMS repo"
        helm upgrade --install kms-core "$kms_chart" \
            --namespace "${NAMESPACE}" \
            -f "${SCRIPT_DIR}/../env/kind/kms-core-values.yaml" \
            --wait --timeout 10m
    else
        echo "Using local KMS Core chart"
        helm upgrade --install kms-core "${REPO_ROOT}/charts/kms-core" \
            --namespace "${NAMESPACE}" \
            -f "${SCRIPT_DIR}/../env/kind/kms-core-values.yaml" \
            --wait --timeout 10m
    fi
}

deploy_blockchain_nodes() {
    echo "Deploying blockchain nodes..."

    # Deploy host node
    helm upgrade --install host-anvil-node "${REPO_ROOT}/charts/anvil-node" \
        --namespace "${NAMESPACE}" \
        -f "${SCRIPT_DIR}/../env/kind/host-node-values.yaml" \
        --wait --timeout 5m &

    # Deploy gateway node (parallel)
    helm upgrade --install gateway-anvil-node "${REPO_ROOT}/charts/anvil-node" \
        --namespace "${NAMESPACE}" \
        -f "${SCRIPT_DIR}/../env/kind/gateway-node-values.yaml" \
        --wait --timeout 5m &

    wait
}

deploy_coprocessor() {
    echo "Deploying coprocessor..."

    local image_tag="${COPROCESSOR_VERSION}"
    [[ "$BUILD" == "true" ]] && image_tag="local"

    helm upgrade --install coprocessor "${REPO_ROOT}/charts/coprocessor" \
        --namespace "${NAMESPACE}" \
        -f "${SCRIPT_DIR}/../env/kind/coprocessor-values.yaml" \
        --set global.image.tag="${image_tag}" \
        --wait --timeout 10m
}

deploy_kms_connector() {
    echo "Deploying KMS connector..."

    local image_tag="${KMS_CONNECTOR_VERSION}"
    [[ "$BUILD" == "true" ]] && image_tag="local"

    helm upgrade --install kms-connector "${REPO_ROOT}/charts/kms-connector" \
        --namespace "${NAMESPACE}" \
        -f "${SCRIPT_DIR}/../env/kind/kms-connector-values.yaml" \
        --set global.image.tag="${image_tag}" \
        --wait --timeout 10m
}

deploy_contracts() {
    echo "Deploying contracts..."

    # Gateway contracts first
    helm upgrade --install gateway-contracts "${REPO_ROOT}/charts/contracts" \
        --namespace "${NAMESPACE}" \
        -f "${SCRIPT_DIR}/../env/kind/gateway-contracts-values.yaml" \
        --wait --timeout 10m

    # Then host contracts
    helm upgrade --install host-contracts "${REPO_ROOT}/charts/contracts" \
        --namespace "${NAMESPACE}" \
        -f "${SCRIPT_DIR}/../env/kind/host-contracts-values.yaml" \
        --wait --timeout 10m
}

deploy_relayer() {
    echo "Deploying relayer..."

    helm upgrade --install relayer "${REPO_ROOT}/charts/relayer" \
        --namespace "${NAMESPACE}" \
        -f "${SCRIPT_DIR}/../env/kind/relayer-values.yaml" \
        --wait --timeout 5m
}

deploy_test_suite() {
    echo "Deploying test suite..."

    helm upgrade --install test-suite "${REPO_ROOT}/charts/test-suite" \
        --namespace "${NAMESPACE}" \
        -f "${SCRIPT_DIR}/../env/kind/test-suite-values.yaml" \
        --wait --timeout 5m
}

setup_port_forwarding() {
    echo "Setting up port forwarding..."

    # Kill any existing port-forwards
    pkill -f "kubectl port-forward.*${NAMESPACE}" || true

    # Start port forwards in background
    kubectl port-forward -n "${NAMESPACE}" svc/host-anvil-node 8545:8545 &
    kubectl port-forward -n "${NAMESPACE}" svc/gateway-anvil-node 8546:8546 &
    kubectl port-forward -n "${NAMESPACE}" svc/relayer 3000:3000 &
    kubectl port-forward -n "${NAMESPACE}" svc/minio 9000:9000 &

    echo "Port forwarding active:"
    echo "  Host RPC:    http://localhost:8545"
    echo "  Gateway RPC: http://localhost:8546"
    echo "  Relayer:     http://localhost:3000"
    echo "  MinIO:       http://localhost:9000"
}

collect_logs() {
    if [[ "$COLLECT_LOGS" != "true" ]]; then
        return 0
    fi

    local log_dir="/tmp/fhevm-kind-logs"
    mkdir -p "$log_dir"

    echo "Collecting logs to ${log_dir}..."

    for pod in $(kubectl get pods -n "${NAMESPACE}" -o jsonpath='{.items[*].metadata.name}'); do
        kubectl logs -n "${NAMESPACE}" "$pod" --all-containers > "${log_dir}/${pod}.log" 2>&1 || true
    done

    echo "Logs collected to ${log_dir}"
}

cleanup_on_exit() {
    if [[ "$LOCAL_MODE" == "true" ]]; then
        echo "Cleaning up..."
        collect_logs
        pkill -f "kubectl port-forward.*${NAMESPACE}" || true
    fi
}

main() {
    parse_args "$@"
    check_prerequisites

    trap cleanup_on_exit EXIT INT TERM

    setup_kind_cluster
    setup_namespace
    setup_registry_credentials
    build_and_load_images

    # Deploy in order
    deploy_infrastructure
    deploy_kms_core
    deploy_blockchain_nodes
    deploy_coprocessor
    deploy_kms_connector
    deploy_contracts
    deploy_relayer
    deploy_test_suite

    setup_port_forwarding

    echo ""
    echo "============================================"
    echo "FHEVM stack deployed successfully!"
    echo "============================================"
    echo ""
    echo "Press Ctrl+C to stop port forwarding and exit"

    # Keep script running for port-forwarding
    if [[ "$LOCAL_MODE" == "true" ]]; then
        wait
    fi
}

main "$@"
```

### 1.3 CI Management Script (Following KMS Pattern)

Create `scripts/manage_kind_setup.sh`:

```bash
#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SETUP_LOG="/tmp/fhevm-kind-setup.log"
SETUP_PID_FILE="/tmp/fhevm-kind-setup.pid"
TIMEOUT="${TIMEOUT:-1800}"  # 30 minutes default

start_setup() {
    echo "Starting FHEVM Kind setup in background..."

    # Start setup script in background
    "${SCRIPT_DIR}/setup_fhevm_in_kind.sh" "$@" > "${SETUP_LOG}" 2>&1 &
    local pid=$!
    echo "$pid" > "${SETUP_PID_FILE}"

    echo "Setup started with PID ${pid}"
    echo "Log file: ${SETUP_LOG}"

    # Wait for completion marker
    local elapsed=0
    while [[ $elapsed -lt $TIMEOUT ]]; do
        if grep -q "FHEVM stack deployed successfully" "${SETUP_LOG}" 2>/dev/null; then
            echo "FHEVM stack ready!"
            return 0
        fi

        if ! kill -0 "$pid" 2>/dev/null; then
            echo "ERROR: Setup process died unexpectedly"
            cat "${SETUP_LOG}"
            return 1
        fi

        sleep 10
        elapsed=$((elapsed + 10))
        echo "Waiting for stack... (${elapsed}s / ${TIMEOUT}s)"
    done

    echo "ERROR: Timeout waiting for stack to be ready"
    cat "${SETUP_LOG}"
    return 1
}

stop_setup() {
    echo "Stopping FHEVM Kind setup..."

    if [[ -f "${SETUP_PID_FILE}" ]]; then
        local pid
        pid=$(cat "${SETUP_PID_FILE}")
        if kill -0 "$pid" 2>/dev/null; then
            kill "$pid" || true
        fi
        rm -f "${SETUP_PID_FILE}"
    fi

    # Kill port-forwards
    pkill -f "kubectl port-forward.*fhevm" || true
}

show_logs() {
    if [[ -f "${SETUP_LOG}" ]]; then
        cat "${SETUP_LOG}"
    else
        echo "No log file found"
    fi
}

case "${1:-start}" in
    start) shift; start_setup "$@" ;;
    stop) stop_setup ;;
    logs) show_logs ;;
    *) echo "Usage: $0 {start|stop|logs} [setup options]"; exit 1 ;;
esac
```

### 1.4 Log Collection Script

Create `scripts/collect_logs.sh`:

```bash
#!/bin/bash
set -euo pipefail

NAMESPACE="${NAMESPACE:-fhevm-local}"
LOG_DIR="${LOG_DIR:-/tmp/fhevm-kind-logs}"

mkdir -p "${LOG_DIR}"

echo "Collecting logs from namespace ${NAMESPACE}..."

# Collect pod logs
for pod in $(kubectl get pods -n "${NAMESPACE}" -o jsonpath='{.items[*].metadata.name}' 2>/dev/null); do
    echo "  Collecting logs from ${pod}..."
    kubectl logs -n "${NAMESPACE}" "${pod}" --all-containers > "${LOG_DIR}/${pod}.log" 2>&1 || true

    # Also get previous logs if available
    kubectl logs -n "${NAMESPACE}" "${pod}" --all-containers --previous > "${LOG_DIR}/${pod}-previous.log" 2>&1 || true
done

# Collect events
kubectl get events -n "${NAMESPACE}" --sort-by='.lastTimestamp' > "${LOG_DIR}/events.log" 2>&1 || true

# Collect pod descriptions
kubectl describe pods -n "${NAMESPACE}" > "${LOG_DIR}/pod-descriptions.log" 2>&1 || true

echo "Logs collected to ${LOG_DIR}"
ls -la "${LOG_DIR}"
```

### 1.5 Namespace and RBAC Setup

```yaml
# manifests/base/namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: fhevm-local
  labels:
    app.kubernetes.io/name: fhevm
    app.kubernetes.io/part-of: fhevm-test-suite
---
apiVersion: v1
kind: ServiceAccount
metadata:
  name: fhevm-deployer
  namespace: fhevm-local
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: fhevm-deployer
  namespace: fhevm-local
rules:
  - apiGroups: ["", "apps", "batch"]
    resources: ["*"]
    verbs: ["*"]
  - apiGroups: [""]
    resources: ["configmaps", "secrets"]
    verbs: ["*"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: fhevm-deployer
  namespace: fhevm-local
subjects:
  - kind: ServiceAccount
    name: fhevm-deployer
    namespace: fhevm-local
roleRef:
  kind: Role
  name: fhevm-deployer
  apiGroup: rbac.authorization.k8s.io
```

## Phase 2: Create Missing Helm Charts

### 2.1 MinIO Values (Using Bitnami Chart)

```yaml
# env/kind/minio-values.yaml
auth:
  rootUser: "fhevm-access-key"
  rootPassword: "fhevm-access-secret-key"

defaultBuckets: "kms-public,ct64,ct128"

service:
  type: ClusterIP
  ports:
    api: 9000
    console: 9001

persistence:
  enabled: true
  size: 2Gi

resources:
  requests:
    cpu: 100m
    memory: 256Mi
  limits:
    cpu: 500m
    memory: 512Mi
```

### 2.2 KMS Core Chart

**Note:** Check if `../kms/charts/kms-core/` exists first. If so, use it directly with custom values.

If creating new, use `charts/kms-core/`:

```yaml
# charts/kms-core/Chart.yaml
apiVersion: v2
name: kms-core
description: KMS Core service for FHEVM
version: 0.1.0
appVersion: "0.13.0"

# charts/kms-core/values.yaml
image:
  repository: ghcr.io/zama-ai/kms-service-dev
  tag: v0.13.0
  pullPolicy: IfNotPresent

config:
  kmsType: "centralized"
  serverPort: 50051
  nbBits: 2048

minio:
  endpoint: "http://minio:9000"
  bucketName: "kms-public"
  accessKey: "fhevm-access-key"
  secretKey: "fhevm-access-secret-key"

service:
  type: ClusterIP
  port: 50051

resources:
  requests:
    cpu: 200m
    memory: 512Mi
  limits:
    cpu: 1000m
    memory: 2Gi

initContainers:
  waitForMinio:
    enabled: true
```

### 2.3 Relayer Chart

Create `charts/relayer/`:

```yaml
# charts/relayer/Chart.yaml
apiVersion: v2
name: relayer
description: Transaction relayer for FHEVM
version: 0.1.0
appVersion: "0.8.4"

dependencies:
  - name: postgresql
    version: "15.x.x"
    repository: "oci://registry-1.docker.io/bitnamicharts"
    condition: postgresql.enabled

# charts/relayer/values.yaml
image:
  repository: ghcr.io/zama-ai/fhevm-relayer
  tag: v0.8.4
  pullPolicy: IfNotPresent

postgresql:
  enabled: true
  auth:
    database: relayer
    username: relayer
    password: relayer-password
  primary:
    persistence:
      size: 1Gi

config:
  hostRpcUrl: "http://host-anvil-node:8545"
  gatewayRpcUrl: "http://gateway-anvil-node:8546"

service:
  type: ClusterIP
  port: 3000

resources:
  requests:
    cpu: 100m
    memory: 256Mi
  limits:
    cpu: 500m
    memory: 512Mi
```

### 2.4 Test Suite Chart

Create `charts/test-suite/`:

```yaml
# charts/test-suite/Chart.yaml
apiVersion: v2
name: test-suite
description: E2E test suite for FHEVM
version: 0.1.0

# charts/test-suite/values.yaml
image:
  repository: ghcr.io/zama-ai/fhevm-test-suite
  tag: latest
  pullPolicy: IfNotPresent

# Run as a long-running pod for interactive testing
command: ["tail", "-f", "/dev/null"]

env:
  NETWORK: staging
  HOST_RPC_URL: "http://host-anvil-node:8545"
  GATEWAY_RPC_URL: "http://gateway-anvil-node:8546"

resources:
  requests:
    cpu: 100m
    memory: 512Mi
  limits:
    cpu: 1000m
    memory: 2Gi
```

## Phase 3: Deployment Orchestration

### 3.1 Helmfile for Orchestration

Create `helmfile.yaml` in `test-suite/fhevm/`:

```yaml
# helmfile.yaml
repositories:
  - name: bitnami
    url: https://charts.bitnami.com/bitnami

helmDefaults:
  wait: true
  timeout: 600
  createNamespace: true

releases:
  # Phase 1: Infrastructure
  - name: minio
    namespace: fhevm-local
    chart: bitnami/minio
    version: 14.x.x
    values:
      - env/kind/minio-values.yaml

  - name: postgresql
    namespace: fhevm-local
    chart: bitnami/postgresql
    version: 15.5.0
    values:
      - env/kind/postgresql-values.yaml

  - name: kms-core
    namespace: fhevm-local
    chart: ../../charts/kms-core
    needs:
      - fhevm-local/minio
    values:
      - env/kind/kms-core-values.yaml

  # Phase 2: Blockchain Networks
  - name: host-anvil-node
    namespace: fhevm-local
    chart: ../../charts/anvil-node
    needs:
      - fhevm-local/kms-core
    values:
      - env/kind/host-node-values.yaml

  - name: gateway-anvil-node
    namespace: fhevm-local
    chart: ../../charts/anvil-node
    needs:
      - fhevm-local/kms-core
    values:
      - env/kind/gateway-node-values.yaml

  # Phase 3: Coprocessor
  - name: coprocessor
    namespace: fhevm-local
    chart: ../../charts/coprocessor
    needs:
      - fhevm-local/postgresql
      - fhevm-local/host-anvil-node
      - fhevm-local/gateway-anvil-node
    values:
      - env/kind/coprocessor-values.yaml

  # Phase 4: KMS Connector
  - name: kms-connector
    namespace: fhevm-local
    chart: ../../charts/kms-connector
    needs:
      - fhevm-local/coprocessor
    values:
      - env/kind/kms-connector-values.yaml

  # Phase 5: Gateway Contracts
  - name: gateway-contracts
    namespace: fhevm-local
    chart: ../../charts/contracts
    needs:
      - fhevm-local/gateway-anvil-node
      - fhevm-local/kms-connector
    values:
      - env/kind/gateway-contracts-values.yaml

  # Phase 6: Host Contracts
  - name: host-contracts
    namespace: fhevm-local
    chart: ../../charts/contracts
    needs:
      - fhevm-local/host-anvil-node
      - fhevm-local/gateway-contracts
    values:
      - env/kind/host-contracts-values.yaml

  # Phase 7: Relayer
  - name: relayer
    namespace: fhevm-local
    chart: ../../charts/relayer
    needs:
      - fhevm-local/host-contracts
      - fhevm-local/gateway-contracts
    values:
      - env/kind/relayer-values.yaml

  # Phase 8: Test Suite
  - name: test-suite
    namespace: fhevm-local
    chart: ../../charts/test-suite
    needs:
      - fhevm-local/relayer
    values:
      - env/kind/test-suite-values.yaml
```

### 3.2 Updated fhevm-cli Commands

Update `fhevm-cli` to support Kind:

```bash
#!/bin/bash
# fhevm-cli additions for Kind support

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLUSTER_NAME="fhevm-local"
NAMESPACE="fhevm-local"

cmd_kind_create() {
    echo "Creating Kind cluster..."
    "${SCRIPT_DIR}/scripts/setup_fhevm_in_kind.sh" --cleanup "$@"
}

cmd_kind_delete() {
    echo "Deleting Kind cluster..."
    kind delete cluster --name "$CLUSTER_NAME"
}

cmd_kind_deploy() {
    "${SCRIPT_DIR}/scripts/setup_fhevm_in_kind.sh" "$@"
}

cmd_kind_test() {
    local test_type="${1:-all}"
    local grep_pattern=""

    case "$test_type" in
        input-proof) grep_pattern="test user input" ;;
        user-decryption) grep_pattern="test user decrypt" ;;
        public-decryption) grep_pattern="test async decrypt" ;;
        erc20) grep_pattern="should transfer tokens between two users" ;;
        operators) grep_pattern="test operator|FHEVM manual operations" ;;
        random) grep_pattern="generate and decrypt|generating rand" ;;
        *) grep_pattern="$test_type" ;;
    esac

    echo "Running tests: $test_type"
    kubectl exec -n "$NAMESPACE" deploy/test-suite -- \
        ./run-tests.sh -n staging -g "$grep_pattern"
}

cmd_kind_logs() {
    local service="${1:-coprocessor}"
    kubectl logs -n "$NAMESPACE" -l "app.kubernetes.io/name=$service" -f --all-containers
}

cmd_kind_status() {
    echo "=== Pods ==="
    kubectl get pods -n "$NAMESPACE" -o wide
    echo ""
    echo "=== Services ==="
    kubectl get svc -n "$NAMESPACE"
    echo ""
    echo "=== Jobs ==="
    kubectl get jobs -n "$NAMESPACE"
    echo ""
    echo "=== PVCs ==="
    kubectl get pvc -n "$NAMESPACE"
}

cmd_kind_port_forward() {
    echo "Starting port forwarding..."
    pkill -f "kubectl port-forward.*${NAMESPACE}" || true

    kubectl port-forward -n "$NAMESPACE" svc/host-anvil-node 8545:8545 &
    kubectl port-forward -n "$NAMESPACE" svc/gateway-anvil-node 8546:8546 &
    kubectl port-forward -n "$NAMESPACE" svc/relayer 3000:3000 &
    kubectl port-forward -n "$NAMESPACE" svc/minio 9000:9000 &

    echo "Port forwarding active. Press Ctrl+C to stop."
    wait
}

cmd_kind_shell() {
    local service="${1:-test-suite}"
    kubectl exec -it -n "$NAMESPACE" deploy/"$service" -- /bin/bash
}
```

## Phase 4: GitHub Actions Workflow

Create `.github/workflows/kind-testing.yml`:

```yaml
name: Kind Testing

on:
  pull_request:
    paths:
      - 'coprocessor/**'
      - 'kms-connector/**'
      - 'host-contracts/**'
      - 'gateway-contracts/**'
      - 'library-solidity/**'
      - 'charts/**'
      - 'test-suite/**'
      - '.github/workflows/kind-testing.yml'
  schedule:
    # Run nightly at midnight UTC, Monday-Friday
    - cron: '0 0 * * 1-5'
  workflow_dispatch:
    inputs:
      test_type:
        description: 'Test type to run'
        required: false
        default: 'all'
        type: choice
        options:
          - all
          - input-proof
          - user-decryption
          - public-decryption
          - erc20
          - operators
          - random

concurrency:
  group: kind-testing-${{ github.head_ref || github.run_id }}
  cancel-in-progress: true

env:
  NAMESPACE: fhevm-test

jobs:
  kind-testing:
    runs-on: ubuntu-latest-16-cores
    timeout-minutes: 60
    strategy:
      fail-fast: false
      matrix:
        test-type:
          - input-proof
          - user-decryption
          - erc20
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Setup Kind
        uses: helm/kind-action@v1
        with:
          cluster_name: fhevm-test
          config: test-suite/fhevm/kind-config.yaml

      - name: Setup Helm
        uses: azure/setup-helm@v4
        with:
          version: 'v3.14.0'

      - name: Setup Helmfile
        run: |
          curl -fsSL -o helmfile https://github.com/helmfile/helmfile/releases/download/v0.162.0/helmfile_0.162.0_linux_amd64.tar.gz
          tar xzf helmfile -C /usr/local/bin
          chmod +x /usr/local/bin/helmfile

      - name: Login to GHCR
        uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Deploy FHEVM Stack
        run: |
          cd test-suite/fhevm
          ./scripts/manage_kind_setup.sh start \
            --namespace ${{ env.NAMESPACE }} \
            --collect-logs
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

      - name: Run Tests - ${{ matrix.test-type }}
        run: |
          cd test-suite/fhevm
          ./fhevm-cli kind-test ${{ matrix.test-type }}

      - name: Collect Logs
        if: always()
        run: |
          cd test-suite/fhevm
          ./scripts/collect_logs.sh
        env:
          NAMESPACE: ${{ env.NAMESPACE }}
          LOG_DIR: /tmp/fhevm-kind-logs

      - name: Upload Logs
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: kind-logs-${{ matrix.test-type }}
          path: /tmp/fhevm-kind-logs/
          retention-days: 7

      - name: Cleanup
        if: always()
        run: |
          cd test-suite/fhevm
          ./scripts/manage_kind_setup.sh stop

  test-reporter:
    needs: kind-testing
    if: always() && github.event_name == 'pull_request'
    runs-on: ubuntu-latest
    steps:
      - name: Download Test Results
        uses: actions/download-artifact@v4
        with:
          pattern: kind-logs-*
          merge-multiple: true

      - name: Publish Test Results
        uses: dorny/test-reporter@v1
        with:
          name: Kind Test Results
          path: '**/junit.xml'
          reporter: java-junit
          fail-on-error: false
```

## Phase 5: Environment Configuration

### 5.1 Kind-specific Values Files

Create `test-suite/fhevm/env/kind/` directory with values files:

```yaml
# env/kind/coprocessor-values.yaml
global:
  namespace: fhevm-local
  image:
    pullPolicy: IfNotPresent

dbMigration:
  enabled: true

hostListener:
  enabled: true
  config:
    rpcWsUrl: "ws://host-anvil-node:8545"

gwListener:
  enabled: true
  config:
    gwUrl: "ws://gateway-anvil-node:8546"

tfheWorker:
  enabled: true
  replicas: 1
  resources:
    requests:
      cpu: 500m
      memory: 1Gi
    limits:
      cpu: 2000m
      memory: 4Gi

zkProofWorker:
  enabled: true
  replicas: 1

snsWorker:
  enabled: true
  config:
    minioEndpoint: "http://minio:9000"

txSender:
  enabled: true
  config:
    gatewayUrl: "http://gateway-anvil-node:8546"

commonConfig:
  databaseUrl: "postgresql://postgres:postgres@postgresql:5432/coprocessor"
```

```yaml
# env/kind/postgresql-values.yaml
auth:
  postgresPassword: postgres
  database: coprocessor

primary:
  persistence:
    size: 2Gi
  resources:
    requests:
      cpu: 100m
      memory: 256Mi
    limits:
      cpu: 500m
      memory: 512Mi

# Create additional databases via init scripts
initdbScripts:
  init.sql: |
    CREATE DATABASE connector;
    CREATE DATABASE relayer;
```

```yaml
# env/kind/host-node-values.yaml
chainId: 12345
mnemonic: "test test test test test test test test test test test junk"
blockTime: 1

service:
  type: ClusterIP
  port: 8545

resources:
  requests:
    cpu: 100m
    memory: 256Mi
  limits:
    cpu: 500m
    memory: 512Mi
```

```yaml
# env/kind/gateway-node-values.yaml
chainId: 54321
mnemonic: "test test test test test test test test test test test junk"
blockTime: 1

service:
  type: ClusterIP
  port: 8546

resources:
  requests:
    cpu: 100m
    memory: 256Mi
  limits:
    cpu: 500m
    memory: 512Mi
```

### 5.2 CI vs Local Values

Create separate values for CI (high resources) and local (reduced resources):

```yaml
# env/kind/values-ci.yaml
# High resource values for CI runners
coprocessor:
  tfheWorker:
    replicas: 2
    resources:
      requests:
        cpu: 2000m
        memory: 4Gi
      limits:
        cpu: 4000m
        memory: 8Gi

# env/kind/values-local.yaml
# Reduced resource values for local development
coprocessor:
  tfheWorker:
    replicas: 1
    resources:
      requests:
        cpu: 500m
        memory: 1Gi
      limits:
        cpu: 2000m
        memory: 4Gi
```

## Phase 6: Implementation Steps

### Priority 1: Study and Reuse (Days 1-2)

1. **Study KMS Implementation**
   - [ ] Review `../kms/ci/kube-testing/` scripts and structure
   - [ ] Understand the setup flow and patterns
   - [ ] Document reusable components

2. **Check Existing Charts**
   - [ ] Check `../kms/charts/kms-core/` - reuse if exists
   - [ ] Check for relayer chart in other repos
   - [ ] Verify Bitnami chart versions compatibility

### Priority 2: Foundation (Days 3-5)

1. **Create Kind Configuration**
   - [ ] `kind-config.yaml` (simplified, following KMS pattern)
   - [ ] `manifests/base/namespace.yaml`

2. **Create Setup Scripts**
   - [ ] `scripts/setup_fhevm_in_kind.sh` (main setup)
   - [ ] `scripts/manage_kind_setup.sh` (CI orchestration)
   - [ ] `scripts/collect_logs.sh` (log collection)

3. **Create Values Files**
   - [ ] `env/kind/minio-values.yaml`
   - [ ] `env/kind/postgresql-values.yaml`
   - [ ] `env/kind/coprocessor-values.yaml`
   - [ ] `env/kind/kms-connector-values.yaml`
   - [ ] Other service values

### Priority 3: Charts and CLI (Days 6-10)

1. **Create/Adapt Helm Charts**
   - [ ] KMS Core chart (or reference external)
   - [ ] Relayer chart
   - [ ] Test Suite chart

2. **Update fhevm-cli**
   - [ ] `kind-create` command
   - [ ] `kind-delete` command
   - [ ] `kind-deploy` command
   - [ ] `kind-test` command
   - [ ] `kind-logs` command
   - [ ] `kind-status` command
   - [ ] `kind-port-forward` command
   - [ ] `kind-shell` command

3. **Create Helmfile**
   - [ ] `helmfile.yaml` with dependency ordering
   - [ ] Test Helmfile deployment flow

### Priority 4: CI Integration (Days 11-14)

1. **Create GitHub Actions Workflow**
   - [ ] `.github/workflows/kind-testing.yml`
   - [ ] Test matrix for different test types
   - [ ] Artifact upload for logs
   - [ ] Test reporter integration

2. **Testing and Validation**
   - [ ] Full stack deployment test
   - [ ] All E2E test types pass
   - [ ] CI workflow runs successfully

### Priority 5: Documentation (Days 15-16)

1. **Update Documentation**
   - [ ] Update test-suite README with Kind instructions
   - [ ] Add troubleshooting guide
   - [ ] Update CLAUDE.md with Kind commands

2. **Maintain Backward Compatibility**
   - [ ] Ensure docker-compose commands still work
   - [ ] Document migration path

## File Structure After Migration

```
test-suite/fhevm/
├── fhevm-cli                          # Updated with Kind support
├── kind-config.yaml                   # Kind cluster configuration
├── helmfile.yaml                      # Helmfile for orchestration
├── manifests/
│   └── base/
│       └── namespace.yaml             # Namespace and RBAC
├── scripts/
│   ├── deploy-fhevm-stack.sh          # Keep for docker-compose
│   ├── setup_fhevm_in_kind.sh         # Main Kind setup (KMS pattern)
│   ├── manage_kind_setup.sh           # CI orchestration (KMS pattern)
│   ├── collect_logs.sh                # Log collection
│   └── setup-registry.sh              # Local registry setup (optional)
├── docker-compose/                    # Keep existing (backward compat)
│   └── ...
├── env/
│   ├── staging/                       # Keep existing (docker-compose)
│   │   └── ...
│   └── kind/                          # Kind-specific values
│       ├── minio-values.yaml
│       ├── postgresql-values.yaml
│       ├── kms-core-values.yaml
│       ├── host-node-values.yaml
│       ├── gateway-node-values.yaml
│       ├── coprocessor-values.yaml
│       ├── kms-connector-values.yaml
│       ├── gateway-contracts-values.yaml
│       ├── host-contracts-values.yaml
│       ├── relayer-values.yaml
│       ├── test-suite-values.yaml
│       ├── values-ci.yaml             # CI-specific overrides
│       └── values-local.yaml          # Local dev overrides
└── config/                            # Keep existing configs
    └── ...
```

## Usage After Migration

```bash
# === KIND COMMANDS ===

# Create Kind cluster and deploy full stack
./fhevm-cli kind-create

# Deploy to existing cluster
./fhevm-cli kind-deploy

# Deploy with local builds (for development)
./fhevm-cli kind-deploy --build

# Run specific tests
./fhevm-cli kind-test input-proof
./fhevm-cli kind-test user-decryption
./fhevm-cli kind-test erc20
./fhevm-cli kind-test operators

# View logs
./fhevm-cli kind-logs coprocessor
./fhevm-cli kind-logs relayer
./fhevm-cli kind-logs kms-core

# Check status
./fhevm-cli kind-status

# Start port forwarding (if not already running)
./fhevm-cli kind-port-forward

# Get shell into a service
./fhevm-cli kind-shell test-suite
./fhevm-cli kind-shell coprocessor

# Clean up
./fhevm-cli kind-delete

# === LEGACY DOCKER-COMPOSE (still works) ===
./fhevm-cli deploy
./fhevm-cli test input-proof
./fhevm-cli clean
```

## Benefits of Kind Migration

1. **Closer to Production**: Kind uses real Kubernetes, matching production environments better than docker-compose.

2. **Resource Management**: Kubernetes provides better resource limits, scheduling, and health checks.

3. **Helm Chart Reuse**: Existing Helm charts can be used directly, reducing maintenance burden.

4. **Scaling**: Easy to test with multiple replicas of services.

5. **Networking**: Kubernetes DNS and service discovery is more robust.

6. **CI/CD Integration**: Kind is commonly used in CI pipelines, enabling better testing.

7. **Pattern Consistency**: Follows the same patterns as KMS kind-testing, making cross-team knowledge sharing easier.

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Increased complexity | Keep docker-compose as fallback option |
| Longer startup time | Use parallel Helm deployments where possible |
| Resource requirements | Document minimum requirements, provide local/CI value profiles |
| Image loading overhead | Use local registry for development |
| Learning curve | Provide clear documentation, follow established KMS patterns |
| CI runner costs | Use matrix strategy to parallelize, use appropriate runner sizes |

## Prerequisites for Kind

- Docker Desktop or Docker Engine
- Kind v0.20.0+
- kubectl v1.28+
- Helm v3.12+
- Helmfile v0.157+ (optional but recommended)
- 16GB RAM minimum (32GB recommended for full stack)
- 50GB free disk space

## Appendix: KMS Kind-Testing Reference

The KMS repository's kind-testing implementation at `../kms/ci/kube-testing/` serves as the reference implementation. Key files:

| File | Purpose |
|------|---------|
| `scripts/setup_kms_in_kind.sh` | Main setup script with CLI flags |
| `scripts/manage_kind_setup.sh` | CI orchestration wrapper |
| `infra/kind-config.yaml` | Kind cluster configuration |
| `infra/localstack-s3-values.yaml` | S3-compatible storage values |
| `kms/values-kms-test.yaml` | KMS Core Helm values |

Key patterns from KMS:
- Use `kubectl port-forward` instead of NodePort for service access
- Reserve system resources in Kind config (`system-reserved: memory=1Gi,cpu=1`)
- Use `<namespace>` placeholder in values, replace with `sed` at runtime
- Background execution with log monitoring for CI
- Signal traps for cleanup on exit
- Parallel Helm installations for independent services
