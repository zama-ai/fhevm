#!/usr/bin/env bash

#=============================================================================
# FHEVM Setup Script for Kind (Kubernetes in Docker)
#
# This script sets up the complete FHEVM stack in a local Kind cluster for
# testing purposes. It deploys all services in the correct order with proper
# dependency handling.
#
# Usage:
#   ./setup_fhevm_in_kind.sh [OPTIONS]
#
# Options:
#   --namespace <name>   Kubernetes namespace (default: fhevm-local)
#   --cleanup            Cleanup existing deployment before setup
#   --build              Build and load Docker images locally
#   --local              Run in local mode (keeps port-forwarding active)
#   --collect-logs       Collect logs from pods (for CI use)
#   -h, --help           Show this help message
#
# Examples:
#   # Basic setup with pre-built images
#   ./setup_fhevm_in_kind.sh
#
#   # Setup with local builds
#   ./setup_fhevm_in_kind.sh --build
#
#   # CI setup with cleanup and log collection
#   ./setup_fhevm_in_kind.sh --cleanup --collect-logs
#
#   # Local development with port forwarding
#   ./setup_fhevm_in_kind.sh --local --build
#
#=============================================================================

# Exit on error, undefined variables, and pipe failures
set -euo pipefail

#=============================================================================
# Configuration Variables
#=============================================================================
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"
ENV_DIR="${SCRIPT_DIR}/../env/kind"

# Default values
NAMESPACE="${NAMESPACE:-fhevm-local}"
CLUSTER_NAME="${CLUSTER_NAME:-fhevm-local}"
KUBE_CONFIG="${HOME}/.kube/kind_config_fhevm"
CLEANUP=false
BUILD=false
LOCAL_MODE=false
COLLECT_LOGS=false

# Image versions (can be overridden by environment)
COPROCESSOR_VERSION="${COPROCESSOR_VERSION:-latest}"
KMS_CONNECTOR_VERSION="${KMS_CONNECTOR_VERSION:-latest}"
KMS_CORE_VERSION="${KMS_CORE_VERSION:-latest}"
RELAYER_VERSION="${RELAYER_VERSION:-latest}"
TEST_SUITE_VERSION="${TEST_SUITE_VERSION:-latest}"

# Port forward PIDs tracking
PORT_FORWARD_PIDS=()

#=============================================================================
# Color Codes for Output
#=============================================================================
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

#=============================================================================
# Logging Functions
#=============================================================================
log_info() {
    echo -e "${GREEN}[INFO]${NC} $*"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

log_step() {
    echo -e "${BLUE}[STEP]${NC} $*"
}

#=============================================================================
# Usage Function
#=============================================================================
usage() {
    # Extract usage from script header comments
    grep "^#" "$0" | grep -v "^#!/" | sed 's/^# //' | head -35
    exit 0
}

#=============================================================================
# Argument Parsing
#=============================================================================
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --namespace)
                NAMESPACE="$2"
                shift 2
                ;;
            --cleanup)
                CLEANUP=true
                shift
                ;;
            --build)
                BUILD=true
                shift
                ;;
            --local)
                LOCAL_MODE=true
                shift
                ;;
            --collect-logs)
                COLLECT_LOGS=true
                shift
                ;;
            -h|--help)
                usage
                ;;
            *)
                log_error "Unknown option: $1"
                usage
                exit 1
                ;;
        esac
    done
}

#=============================================================================
# Prerequisite Checks
#=============================================================================
check_prerequisites() {
    log_step "Checking prerequisites..."

    local missing_tools=()

    for tool in kubectl helm kind docker; do
        if ! command -v "$tool" &> /dev/null; then
            missing_tools+=("$tool")
        fi
    done

    if [[ ${#missing_tools[@]} -gt 0 ]]; then
        log_error "Missing required tools: ${missing_tools[*]}"
        log_error "Please install them before running this script"
        exit 1
    fi

    # Check Docker is running
    if ! docker info &> /dev/null; then
        log_error "Docker is not running. Please start Docker first."
        exit 1
    fi

    log_info "All prerequisites met"
}

#=============================================================================
# Kind Cluster Setup
#=============================================================================
setup_kind_cluster() {
    log_step "Setting up Kind cluster..."

    local kind_config="${SCRIPT_DIR}/../kind-config.yaml"

    if kind get clusters 2>/dev/null | grep -q "^${CLUSTER_NAME}$"; then
        log_warn "Kind cluster '${CLUSTER_NAME}' already exists"
        if [[ "$CLEANUP" == "true" ]]; then
            log_info "Deleting existing cluster..."
            kind delete cluster --name "${CLUSTER_NAME}" || true
            rm -f "${KUBE_CONFIG}"
        else
            log_info "Reusing existing cluster"
            # Update kubeconfig
            kind export kubeconfig --name "${CLUSTER_NAME}" --kubeconfig "${KUBE_CONFIG}"
            return 0
        fi
    fi

    log_info "Creating Kind cluster '${CLUSTER_NAME}'..."

    if [[ -f "$kind_config" ]]; then
        kind create cluster --name "${CLUSTER_NAME}" --kubeconfig "${KUBE_CONFIG}" --config "$kind_config"
    else
        # Use default config if kind-config.yaml doesn't exist
        kind create cluster --name "${CLUSTER_NAME}" --kubeconfig "${KUBE_CONFIG}" --config - <<EOF
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
name: ${CLUSTER_NAME}
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
EOF
    fi

    log_info "Kind cluster created successfully"
}

#=============================================================================
# Namespace Setup
#=============================================================================
setup_namespace() {
    log_step "Setting up namespace: ${NAMESPACE}..."

    if kubectl get namespace "${NAMESPACE}" --kubeconfig "${KUBE_CONFIG}" &> /dev/null; then
        if [[ "$CLEANUP" == "true" ]]; then
            log_info "Deleting existing namespace..."
            kubectl delete namespace "${NAMESPACE}" --kubeconfig "${KUBE_CONFIG}" --wait=true || true
            # Wait a moment for cleanup
            sleep 5
        else
            log_warn "Namespace '${NAMESPACE}' already exists, skipping creation"
            return 0
        fi
    fi

    kubectl create namespace "${NAMESPACE}" --kubeconfig "${KUBE_CONFIG}"
    log_info "Namespace '${NAMESPACE}' created"
}

#=============================================================================
# Registry Credentials Setup
#=============================================================================
setup_registry_credentials() {
    log_step "Setting up registry credentials..."

    if [[ -z "${GITHUB_TOKEN:-}" ]]; then
        log_warn "GITHUB_TOKEN not set, skipping registry credentials setup"
        log_warn "Set GITHUB_TOKEN environment variable to enable private image pulls"
        return 0
    fi

    local github_user="${GITHUB_ACTOR:-github}"

    # Create docker-registry secret for ghcr.io
    kubectl create secret docker-registry ghcr-creds \
        --namespace "${NAMESPACE}" \
        --kubeconfig "${KUBE_CONFIG}" \
        --docker-server=ghcr.io \
        --docker-username="${github_user}" \
        --docker-password="${GITHUB_TOKEN}" \
        --dry-run=client -o yaml | kubectl apply -f - --kubeconfig "${KUBE_CONFIG}"

    # Patch default service account to use the secret
    kubectl patch serviceaccount default \
        --namespace "${NAMESPACE}" \
        --kubeconfig "${KUBE_CONFIG}" \
        -p '{"imagePullSecrets": [{"name": "ghcr-creds"}]}' || true

    log_info "Registry credentials configured"
}

#=============================================================================
# Build and Load Docker Images
#=============================================================================
build_and_load_images() {
    if [[ "$BUILD" != "true" ]]; then
        log_info "Skipping local image builds (--build not specified)"
        return 0
    fi

    log_step "Building and loading Docker images into Kind..."

    # Build coprocessor images
    log_info "Building coprocessor images..."
    local coprocessor_dockerfile="${REPO_ROOT}/coprocessor/fhevm-engine/Dockerfile.workspace"
    if [[ -f "$coprocessor_dockerfile" ]]; then
        docker build -t "ghcr.io/zama-ai/fhevm-coprocessor:local" \
            -f "$coprocessor_dockerfile" \
            "${REPO_ROOT}/coprocessor/fhevm-engine"
        kind load docker-image "ghcr.io/zama-ai/fhevm-coprocessor:local" \
            --name "${CLUSTER_NAME}"
    else
        log_warn "Coprocessor Dockerfile not found, skipping build"
    fi

    # Build kms-connector images
    log_info "Building kms-connector images..."
    local connector_dockerfile="${REPO_ROOT}/kms-connector/Dockerfile.workspace"
    if [[ -f "$connector_dockerfile" ]]; then
        docker build -t "ghcr.io/zama-ai/kms-connector:local" \
            -f "$connector_dockerfile" \
            "${REPO_ROOT}/kms-connector"
        kind load docker-image "ghcr.io/zama-ai/kms-connector:local" \
            --name "${CLUSTER_NAME}"
    elif [[ -d "${REPO_ROOT}/kms-connector" ]]; then
        docker build -t "ghcr.io/zama-ai/kms-connector:local" \
            "${REPO_ROOT}/kms-connector"
        kind load docker-image "ghcr.io/zama-ai/kms-connector:local" \
            --name "${CLUSTER_NAME}"
    else
        log_warn "KMS connector directory not found, skipping build"
    fi

    # Build contract deployment images
    log_info "Building contract deployment images..."
    for contract_dir in host-contracts gateway-contracts; do
        if [[ -d "${REPO_ROOT}/${contract_dir}" ]]; then
            docker build -t "ghcr.io/zama-ai/fhevm-${contract_dir}:local" \
                "${REPO_ROOT}/${contract_dir}" || log_warn "Failed to build ${contract_dir}"
            kind load docker-image "ghcr.io/zama-ai/fhevm-${contract_dir}:local" \
                --name "${CLUSTER_NAME}" || true
        fi
    done

    log_info "Docker images built and loaded"
}

#=============================================================================
# Deploy Infrastructure (MinIO, PostgreSQL)
#=============================================================================
deploy_infrastructure() {
    log_step "Deploying infrastructure (MinIO, PostgreSQL)..."

    # Add Bitnami Helm repository
    helm repo add bitnami https://charts.bitnami.com/bitnami 2>/dev/null || true
    helm repo update

    # Deploy MinIO
    log_info "Deploying MinIO..."
    local minio_values="${ENV_DIR}/minio-values.yaml"
    if [[ -f "$minio_values" ]]; then
        helm upgrade --install minio bitnami/minio \
            --namespace "${NAMESPACE}" \
            --kubeconfig "${KUBE_CONFIG}" \
            -f "$minio_values" \
            --wait --timeout 5m
    else
        log_warn "MinIO values file not found, using defaults"
        helm upgrade --install minio bitnami/minio \
            --namespace "${NAMESPACE}" \
            --kubeconfig "${KUBE_CONFIG}" \
            --set auth.rootUser=fhevm-access-key \
            --set auth.rootPassword=fhevm-access-secret-key \
            --set defaultBuckets="kms-public,ct64,ct128" \
            --wait --timeout 5m
    fi

    # Deploy PostgreSQL
    log_info "Deploying PostgreSQL..."
    local postgresql_values="${ENV_DIR}/postgresql-values.yaml"
    if [[ -f "$postgresql_values" ]]; then
        helm upgrade --install postgresql bitnami/postgresql \
            --namespace "${NAMESPACE}" \
            --kubeconfig "${KUBE_CONFIG}" \
            -f "$postgresql_values" \
            --wait --timeout 5m
    else
        log_warn "PostgreSQL values file not found, using defaults"
        helm upgrade --install postgresql bitnami/postgresql \
            --namespace "${NAMESPACE}" \
            --kubeconfig "${KUBE_CONFIG}" \
            --set auth.postgresPassword=postgres \
            --set auth.database=coprocessor \
            --wait --timeout 5m
    fi

    log_info "Infrastructure deployed successfully"
}

#=============================================================================
# Deploy KMS Core
#=============================================================================
deploy_kms_core() {
    log_step "Deploying KMS Core..."

    # Check for KMS Core chart in multiple locations
    local kms_chart=""
    local kms_values="${ENV_DIR}/kms-core-values.yaml"

    # First, check the KMS repository
    if [[ -d "${REPO_ROOT}/../kms/charts/kms-core" ]]; then
        kms_chart="${REPO_ROOT}/../kms/charts/kms-core"
        log_info "Using KMS Core chart from KMS repository"
    elif [[ -d "${REPO_ROOT}/charts/kms-core" ]]; then
        kms_chart="${REPO_ROOT}/charts/kms-core"
        log_info "Using KMS Core chart from FHEVM repository"
    else
        log_warn "KMS Core chart not found, skipping deployment"
        log_warn "Expected locations:"
        log_warn "  - ${REPO_ROOT}/../kms/charts/kms-core"
        log_warn "  - ${REPO_ROOT}/charts/kms-core"
        return 0
    fi

    local helm_args=(
        upgrade --install kms-core "$kms_chart"
        --namespace "${NAMESPACE}"
        --kubeconfig "${KUBE_CONFIG}"
        --wait --timeout 10m
    )

    if [[ -f "$kms_values" ]]; then
        helm_args+=(-f "$kms_values")
    fi

    helm "${helm_args[@]}"

    log_info "KMS Core deployed successfully"
}

#=============================================================================
# Deploy Blockchain Nodes (Host and Gateway)
#=============================================================================
deploy_blockchain_nodes() {
    log_step "Deploying blockchain nodes (host and gateway)..."

    local anvil_chart="${REPO_ROOT}/charts/anvil-node"
    if [[ ! -d "$anvil_chart" ]]; then
        log_error "Anvil node chart not found at ${anvil_chart}"
        exit 1
    fi

    # Deploy host node in background
    log_info "Deploying host anvil node..."
    local host_values="${ENV_DIR}/host-node-values.yaml"
    (
        local helm_args=(
            upgrade --install host-anvil-node "$anvil_chart"
            --namespace "${NAMESPACE}"
            --kubeconfig "${KUBE_CONFIG}"
            --wait --timeout 5m
        )
        if [[ -f "$host_values" ]]; then
            helm_args+=(-f "$host_values")
        fi
        helm "${helm_args[@]}"
    ) &
    local host_pid=$!

    # Deploy gateway node in background
    log_info "Deploying gateway anvil node..."
    local gateway_values="${ENV_DIR}/gateway-node-values.yaml"
    (
        local helm_args=(
            upgrade --install gateway-anvil-node "$anvil_chart"
            --namespace "${NAMESPACE}"
            --kubeconfig "${KUBE_CONFIG}"
            --wait --timeout 5m
        )
        if [[ -f "$gateway_values" ]]; then
            helm_args+=(-f "$gateway_values")
        fi
        helm "${helm_args[@]}"
    ) &
    local gateway_pid=$!

    # Wait for both deployments
    wait $host_pid || { log_error "Host node deployment failed"; exit 1; }
    wait $gateway_pid || { log_error "Gateway node deployment failed"; exit 1; }

    log_info "Blockchain nodes deployed successfully"
}

#=============================================================================
# Deploy Coprocessor
#=============================================================================
deploy_coprocessor() {
    log_step "Deploying coprocessor..."

    local coprocessor_chart="${REPO_ROOT}/charts/coprocessor"
    if [[ ! -d "$coprocessor_chart" ]]; then
        log_error "Coprocessor chart not found at ${coprocessor_chart}"
        exit 1
    fi

    local image_tag="${COPROCESSOR_VERSION}"
    [[ "$BUILD" == "true" ]] && image_tag="local"

    local coprocessor_values="${ENV_DIR}/coprocessor-values.yaml"
    local helm_args=(
        upgrade --install coprocessor "$coprocessor_chart"
        --namespace "${NAMESPACE}"
        --kubeconfig "${KUBE_CONFIG}"
        --wait --timeout 10m
    )

    if [[ -f "$coprocessor_values" ]]; then
        helm_args+=(-f "$coprocessor_values")
    fi

    # Add image tag override if building locally
    if [[ "$BUILD" == "true" ]]; then
        helm_args+=(--set "global.image.tag=${image_tag}")
    fi

    helm "${helm_args[@]}"

    log_info "Coprocessor deployed successfully"
}

#=============================================================================
# Deploy KMS Connector
#=============================================================================
deploy_kms_connector() {
    log_step "Deploying KMS connector..."

    local connector_chart="${REPO_ROOT}/charts/kms-connector"
    if [[ ! -d "$connector_chart" ]]; then
        log_error "KMS connector chart not found at ${connector_chart}"
        exit 1
    fi

    local image_tag="${KMS_CONNECTOR_VERSION}"
    [[ "$BUILD" == "true" ]] && image_tag="local"

    local connector_values="${ENV_DIR}/kms-connector-values.yaml"
    local helm_args=(
        upgrade --install kms-connector "$connector_chart"
        --namespace "${NAMESPACE}"
        --kubeconfig "${KUBE_CONFIG}"
        --wait --timeout 10m
    )

    if [[ -f "$connector_values" ]]; then
        helm_args+=(-f "$connector_values")
    fi

    if [[ "$BUILD" == "true" ]]; then
        helm_args+=(--set "global.image.tag=${image_tag}")
    fi

    helm "${helm_args[@]}"

    log_info "KMS connector deployed successfully"
}

#=============================================================================
# Deploy Contracts (Gateway then Host)
#=============================================================================
deploy_contracts() {
    log_step "Deploying contracts..."

    local contracts_chart="${REPO_ROOT}/charts/contracts"
    if [[ ! -d "$contracts_chart" ]]; then
        log_error "Contracts chart not found at ${contracts_chart}"
        exit 1
    fi

    # Deploy gateway contracts first
    log_info "Deploying gateway contracts..."
    local gateway_contracts_values="${ENV_DIR}/gateway-contracts-values.yaml"
    local helm_args=(
        upgrade --install gateway-contracts "$contracts_chart"
        --namespace "${NAMESPACE}"
        --kubeconfig "${KUBE_CONFIG}"
        --wait --timeout 10m
    )

    if [[ -f "$gateway_contracts_values" ]]; then
        helm_args+=(-f "$gateway_contracts_values")
    fi

    helm "${helm_args[@]}"

    # Deploy host contracts
    log_info "Deploying host contracts..."
    local host_contracts_values="${ENV_DIR}/host-contracts-values.yaml"
    helm_args=(
        upgrade --install host-contracts "$contracts_chart"
        --namespace "${NAMESPACE}"
        --kubeconfig "${KUBE_CONFIG}"
        --wait --timeout 10m
    )

    if [[ -f "$host_contracts_values" ]]; then
        helm_args+=(-f "$host_contracts_values")
    fi

    helm "${helm_args[@]}"

    log_info "Contracts deployed successfully"
}

#=============================================================================
# Deploy Relayer
#=============================================================================
deploy_relayer() {
    log_step "Deploying relayer..."

    local relayer_chart="${REPO_ROOT}/charts/relayer"
    if [[ ! -d "$relayer_chart" ]]; then
        log_warn "Relayer chart not found at ${relayer_chart}, skipping"
        return 0
    fi

    local relayer_values="${ENV_DIR}/relayer-values.yaml"
    local helm_args=(
        upgrade --install relayer "$relayer_chart"
        --namespace "${NAMESPACE}"
        --kubeconfig "${KUBE_CONFIG}"
        --wait --timeout 5m
    )

    if [[ -f "$relayer_values" ]]; then
        helm_args+=(-f "$relayer_values")
    fi

    helm "${helm_args[@]}"

    log_info "Relayer deployed successfully"
}

#=============================================================================
# Deploy Test Suite
#=============================================================================
deploy_test_suite() {
    log_step "Deploying test suite..."

    local test_suite_chart="${REPO_ROOT}/charts/test-suite"
    if [[ ! -d "$test_suite_chart" ]]; then
        log_warn "Test suite chart not found at ${test_suite_chart}, skipping"
        return 0
    fi

    local test_suite_values="${ENV_DIR}/test-suite-values.yaml"
    local helm_args=(
        upgrade --install test-suite "$test_suite_chart"
        --namespace "${NAMESPACE}"
        --kubeconfig "${KUBE_CONFIG}"
        --wait --timeout 5m
    )

    if [[ -f "$test_suite_values" ]]; then
        helm_args+=(-f "$test_suite_values")
    fi

    helm "${helm_args[@]}"

    log_info "Test suite deployed successfully"
}

#=============================================================================
# Port Forwarding Setup
#=============================================================================
setup_port_forwarding() {
    log_step "Setting up port forwarding..."

    # Kill any existing port-forwards for this namespace
    pkill -f "kubectl port-forward.*${NAMESPACE}" 2>/dev/null || true
    sleep 1

    # Port forward host anvil node (8545)
    log_info "Port forwarding host-anvil-node (8545:8545)..."
    kubectl port-forward \
        -n "${NAMESPACE}" \
        svc/host-anvil-node 8545:8545 \
        --kubeconfig "${KUBE_CONFIG}" \
        > /dev/null 2>&1 &
    PORT_FORWARD_PIDS+=($!)

    # Port forward gateway anvil node (8546)
    log_info "Port forwarding gateway-anvil-node (8546:8546)..."
    kubectl port-forward \
        -n "${NAMESPACE}" \
        svc/gateway-anvil-node 8546:8546 \
        --kubeconfig "${KUBE_CONFIG}" \
        > /dev/null 2>&1 &
    PORT_FORWARD_PIDS+=($!)

    # Port forward relayer (3000)
    log_info "Port forwarding relayer (3000:3000)..."
    kubectl port-forward \
        -n "${NAMESPACE}" \
        svc/relayer 3000:3000 \
        --kubeconfig "${KUBE_CONFIG}" \
        > /dev/null 2>&1 &
    PORT_FORWARD_PIDS+=($!)

    # Port forward MinIO (9000)
    log_info "Port forwarding minio (9000:9000)..."
    kubectl port-forward \
        -n "${NAMESPACE}" \
        svc/minio 9000:9000 \
        --kubeconfig "${KUBE_CONFIG}" \
        > /dev/null 2>&1 &
    PORT_FORWARD_PIDS+=($!)

    # Give port forwards a moment to establish
    sleep 2

    log_info "Port forwarding setup complete:"
    log_info "  Host RPC:    http://localhost:8545"
    log_info "  Gateway RPC: http://localhost:8546"
    log_info "  Relayer:     http://localhost:3000"
    log_info "  MinIO:       http://localhost:9000"
}

#=============================================================================
# Log Collection
#=============================================================================
collect_logs() {
    local log_dir="${LOG_DIR:-/tmp/fhevm-kind-logs}"

    if [[ "$COLLECT_LOGS" != "true" ]] && [[ "${1:-}" != "force" ]]; then
        return 0
    fi

    log_step "Collecting logs to ${log_dir}..."
    mkdir -p "${log_dir}"

    # Collect pod logs
    for pod in $(kubectl get pods -n "${NAMESPACE}" --kubeconfig "${KUBE_CONFIG}" \
                 -o jsonpath='{.items[*].metadata.name}' 2>/dev/null); do
        log_info "  Collecting logs from ${pod}..."
        kubectl logs -n "${NAMESPACE}" "${pod}" --all-containers \
            --kubeconfig "${KUBE_CONFIG}" \
            > "${log_dir}/${pod}.log" 2>&1 || true

        # Also get previous logs if available
        kubectl logs -n "${NAMESPACE}" "${pod}" --all-containers --previous \
            --kubeconfig "${KUBE_CONFIG}" \
            > "${log_dir}/${pod}-previous.log" 2>&1 || true
    done

    # Collect events
    kubectl get events -n "${NAMESPACE}" --sort-by='.lastTimestamp' \
        --kubeconfig "${KUBE_CONFIG}" \
        > "${log_dir}/events.log" 2>&1 || true

    # Collect pod descriptions
    kubectl describe pods -n "${NAMESPACE}" \
        --kubeconfig "${KUBE_CONFIG}" \
        > "${log_dir}/pod-descriptions.log" 2>&1 || true

    log_info "Logs collected to ${log_dir}"
}

#=============================================================================
# Cleanup on Exit
#=============================================================================
cleanup_on_exit() {
    local exit_code=$?

    log_info "========================================="
    log_info "Cleanup triggered"
    log_info "========================================="

    # Collect logs if requested or in local mode
    if [[ "$COLLECT_LOGS" == "true" ]] || [[ "$LOCAL_MODE" == "true" ]]; then
        collect_logs force
    fi

    # Stop port forwarding processes
    log_info "Stopping port-forward processes..."
    for pid in "${PORT_FORWARD_PIDS[@]}"; do
        kill "$pid" 2>/dev/null || true
    done
    pkill -f "kubectl port-forward.*${NAMESPACE}" 2>/dev/null || true

    # In local mode, perform full cleanup
    if [[ "$LOCAL_MODE" == "true" ]]; then
        log_info "Local mode: performing full cleanup..."
        kind delete cluster --name "${CLUSTER_NAME}" 2>/dev/null || true
        rm -f "${KUBE_CONFIG}"
    fi

    log_info "Cleanup completed"
    exit $exit_code
}

#=============================================================================
# Main Function
#=============================================================================
main() {
    parse_args "$@"

    log_info "========================================="
    log_info "Starting FHEVM Kind Setup"
    log_info "========================================="
    log_info "Configuration:"
    log_info "  Namespace:     ${NAMESPACE}"
    log_info "  Cluster:       ${CLUSTER_NAME}"
    log_info "  Cleanup:       ${CLEANUP}"
    log_info "  Build:         ${BUILD}"
    log_info "  Local Mode:    ${LOCAL_MODE}"
    log_info "  Collect Logs:  ${COLLECT_LOGS}"
    log_info "========================================="

    # Set up signal traps for cleanup
    trap cleanup_on_exit EXIT INT TERM

    # Execute setup steps in order
    check_prerequisites
    setup_kind_cluster
    setup_namespace
    setup_registry_credentials
    build_and_load_images

    # Deploy services in dependency order
    # Phase 1: Infrastructure (MinIO, PostgreSQL) - can run in parallel
    deploy_infrastructure

    # Phase 2: KMS Core (depends on MinIO)
    deploy_kms_core

    # Phase 3: Blockchain nodes (depends on KMS Core) - can run in parallel
    deploy_blockchain_nodes

    # Phase 4: Coprocessor (depends on PostgreSQL, both nodes)
    deploy_coprocessor

    # Phase 5: KMS Connector (depends on Coprocessor)
    deploy_kms_connector

    # Phase 6: Contracts (gateway first, then host)
    deploy_contracts

    # Phase 7: Relayer (depends on contracts)
    deploy_relayer

    # Phase 8: Test Suite (depends on relayer)
    deploy_test_suite

    # Setup port forwarding
    setup_port_forwarding

    # Success message (used by CI to detect completion)
    echo ""
    log_info "============================================"
    log_info "FHEVM stack deployed successfully!"
    log_info "============================================"
    echo ""
    log_info "Service Access URLs:"
    log_info "  Host RPC:    http://localhost:8545"
    log_info "  Gateway RPC: http://localhost:8546"
    log_info "  Relayer:     http://localhost:3000"
    log_info "  MinIO:       http://localhost:9000"
    echo ""
    log_info "Useful commands:"
    log_info "  View pods:   kubectl get pods -n ${NAMESPACE} --kubeconfig ${KUBE_CONFIG}"
    log_info "  View logs:   kubectl logs -n ${NAMESPACE} <pod-name> --kubeconfig ${KUBE_CONFIG}"
    log_info "  Cleanup:     kind delete cluster --name ${CLUSTER_NAME}"
    echo ""

    # In local mode, keep script running for port-forwarding
    if [[ "$LOCAL_MODE" == "true" ]]; then
        log_info "Port forwarding is running in the background."
        log_info "Press Ctrl+C to stop port forwarding and exit."
        log_info "============================================"

        # Wait indefinitely - this makes the script properly interruptible
        while true; do
            sleep 3600 &
            wait $! || true
        done
    fi
}

#=============================================================================
# Script Execution
#=============================================================================
main "$@"
