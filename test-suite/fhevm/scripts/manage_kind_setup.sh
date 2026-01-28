#!/usr/bin/env bash

#=============================================================================
# Manage FHEVM Kind Setup Lifecycle
#
# This script manages the lifecycle of the FHEVM Kind setup script,
# providing CI orchestration capabilities:
#   - start: Launch setup in background and wait for completion
#   - stop:  Gracefully stop setup and cleanup resources
#   - logs:  Show setup logs
#
# Usage:
#   ./manage_kind_setup.sh start [setup options]
#   ./manage_kind_setup.sh stop
#   ./manage_kind_setup.sh logs
#
# Environment Variables:
#   TIMEOUT       - Maximum time to wait for setup (default: 1800 seconds / 30 min)
#   NAMESPACE     - Kubernetes namespace (default: fhevm-local)
#   LOG_FILE      - Path to log file (default: /tmp/fhevm-kind-setup.log)
#
# Examples:
#   # Start setup and wait for completion
#   ./manage_kind_setup.sh start --cleanup --collect-logs
#
#   # Check logs while setup is running
#   ./manage_kind_setup.sh logs
#
#   # Stop setup and cleanup
#   ./manage_kind_setup.sh stop
#
#=============================================================================

set -euo pipefail

#=============================================================================
# Configuration
#=============================================================================
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SETUP_LOG="${LOG_FILE:-/tmp/fhevm-kind-setup.log}"
SETUP_PID_FILE="/tmp/fhevm-kind-setup.pid"
TAIL_PID_FILE="/tmp/fhevm-kind-setup-tail.pid"
TIMEOUT="${TIMEOUT:-1800}"  # 30 minutes default
NAMESPACE="${NAMESPACE:-fhevm-local}"
CLUSTER_NAME="${CLUSTER_NAME:-fhevm-local}"
KUBE_CONFIG="${HOME}/.kube/kind_config_fhevm"

# Completion marker that the setup script prints on success
COMPLETION_MARKER="FHEVM stack deployed successfully!"

#=============================================================================
# Color Codes
#=============================================================================
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

#=============================================================================
# Logging
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

#=============================================================================
# Start Setup
#=============================================================================
start_setup() {
    local setup_args=("$@")

    log_info "Starting FHEVM Kind setup in background..."
    log_info "Log file: ${SETUP_LOG}"
    log_info "Timeout: ${TIMEOUT} seconds"

    # Clean up any previous run
    rm -f "${SETUP_LOG}" "${SETUP_PID_FILE}" "${TAIL_PID_FILE}"
    touch "${SETUP_LOG}"

    # Run setup script in background
    "${SCRIPT_DIR}/setup_fhevm_in_kind.sh" "${setup_args[@]}" > "${SETUP_LOG}" 2>&1 &
    local setup_pid=$!
    echo "${setup_pid}" > "${SETUP_PID_FILE}"

    # Start tailing the log file for real-time output
    tail -f "${SETUP_LOG}" &
    local tail_pid=$!
    echo "${tail_pid}" > "${TAIL_PID_FILE}"

    log_info "Setup PID: ${setup_pid}"
    log_info "Tail PID: ${tail_pid}"
    log_info "Waiting for setup to complete..."

    # Monitor for completion
    local elapsed=0
    local check_interval=10

    while [[ $elapsed -lt $TIMEOUT ]]; do
        # Check if completion marker appears in log
        if grep -q "${COMPLETION_MARKER}" "${SETUP_LOG}" 2>/dev/null; then
            log_info "============================================"
            log_info "FHEVM stack ready!"
            log_info "============================================"

            # Stop the tail process but keep setup running (for port-forwards)
            if [[ -f "${TAIL_PID_FILE}" ]]; then
                kill "$(cat "${TAIL_PID_FILE}")" 2>/dev/null || true
                rm -f "${TAIL_PID_FILE}"
            fi

            return 0
        fi

        # Check if setup process is still running
        if ! kill -0 "${setup_pid}" 2>/dev/null; then
            log_error "Setup process died unexpectedly!"
            log_error "Last 50 lines of log:"
            tail -50 "${SETUP_LOG}" || true
            cleanup_pids
            return 1
        fi

        # Progress update every check_interval seconds
        sleep "${check_interval}"
        elapsed=$((elapsed + check_interval))
        echo "[$(date '+%H:%M:%S')] Waiting for stack... (${elapsed}s / ${TIMEOUT}s)"
    done

    # Timeout reached
    log_error "============================================"
    log_error "Timeout waiting for stack to be ready"
    log_error "============================================"
    log_error "Last 100 lines of log:"
    tail -100 "${SETUP_LOG}" || true

    # Clean up
    cleanup_pids
    return 1
}

#=============================================================================
# Stop Setup
#=============================================================================
stop_setup() {
    log_info "Stopping FHEVM Kind setup..."

    cleanup_pids

    # Kill any remaining port-forward processes
    log_info "Terminating port-forward processes..."
    pkill -f "kubectl port-forward.*${NAMESPACE}" 2>/dev/null || true

    # Delete the Kind cluster
    if kind get clusters 2>/dev/null | grep -q "^${CLUSTER_NAME}$"; then
        log_info "Deleting Kind cluster '${CLUSTER_NAME}'..."
        kind delete cluster --name "${CLUSTER_NAME}" 2>/dev/null || true
    fi

    # Clean up kubeconfig
    rm -f "${KUBE_CONFIG}"

    # Clean up PID files
    rm -f "${SETUP_PID_FILE}" "${TAIL_PID_FILE}"

    log_info "Cleanup completed"
}

#=============================================================================
# Show Logs
#=============================================================================
show_logs() {
    if [[ -f "${SETUP_LOG}" ]]; then
        cat "${SETUP_LOG}"
    else
        log_warn "No log file found at ${SETUP_LOG}"
    fi
}

#=============================================================================
# Follow Logs
#=============================================================================
follow_logs() {
    if [[ -f "${SETUP_LOG}" ]]; then
        tail -f "${SETUP_LOG}"
    else
        log_warn "No log file found at ${SETUP_LOG}"
    fi
}

#=============================================================================
# Status Check
#=============================================================================
check_status() {
    log_info "FHEVM Kind Setup Status"
    log_info "========================"

    # Check if setup process is running
    if [[ -f "${SETUP_PID_FILE}" ]]; then
        local pid
        pid=$(cat "${SETUP_PID_FILE}")
        if kill -0 "${pid}" 2>/dev/null; then
            log_info "Setup process: Running (PID: ${pid})"
        else
            log_warn "Setup process: Not running (stale PID file)"
        fi
    else
        log_info "Setup process: Not running"
    fi

    # Check if Kind cluster exists
    if kind get clusters 2>/dev/null | grep -q "^${CLUSTER_NAME}$"; then
        log_info "Kind cluster: Running"

        # Check pods if cluster is running
        if [[ -f "${KUBE_CONFIG}" ]]; then
            echo ""
            log_info "Pod Status:"
            kubectl get pods -n "${NAMESPACE}" --kubeconfig "${KUBE_CONFIG}" 2>/dev/null || true
        fi
    else
        log_info "Kind cluster: Not running"
    fi

    # Check log file
    if [[ -f "${SETUP_LOG}" ]]; then
        log_info "Log file: ${SETUP_LOG} ($(wc -l < "${SETUP_LOG}") lines)"
    else
        log_info "Log file: Not found"
    fi
}

#=============================================================================
# Helper: Cleanup PIDs
#=============================================================================
cleanup_pids() {
    # Stop tail process
    if [[ -f "${TAIL_PID_FILE}" ]]; then
        local tail_pid
        tail_pid=$(cat "${TAIL_PID_FILE}")
        if kill -0 "${tail_pid}" 2>/dev/null; then
            kill "${tail_pid}" 2>/dev/null || true
        fi
        rm -f "${TAIL_PID_FILE}"
    fi

    # Stop setup process
    if [[ -f "${SETUP_PID_FILE}" ]]; then
        local setup_pid
        setup_pid=$(cat "${SETUP_PID_FILE}")
        if kill -0 "${setup_pid}" 2>/dev/null; then
            log_info "Terminating setup process (PID: ${setup_pid})..."
            kill "${setup_pid}" 2>/dev/null || true
            # Give it a moment to clean up gracefully
            sleep 2
            # Force kill if still running
            kill -9 "${setup_pid}" 2>/dev/null || true
        fi
        rm -f "${SETUP_PID_FILE}"
    fi
}

#=============================================================================
# Usage
#=============================================================================
usage() {
    cat <<EOF
Usage: $(basename "$0") <command> [options]

Commands:
    start [options]    Start setup in background and wait for completion
                       Options are passed to setup_fhevm_in_kind.sh
    stop               Stop setup and cleanup all resources
    logs               Show setup log file
    follow             Follow setup log file (tail -f)
    status             Show current status

Environment Variables:
    TIMEOUT       Maximum wait time in seconds (default: 1800)
    NAMESPACE     Kubernetes namespace (default: fhevm-local)
    LOG_FILE      Path to log file (default: /tmp/fhevm-kind-setup.log)

Examples:
    # Start with cleanup and log collection
    $(basename "$0") start --cleanup --collect-logs

    # Start with local image builds
    $(basename "$0") start --build

    # Check status
    $(basename "$0") status

    # View logs
    $(basename "$0") logs

    # Stop everything
    $(basename "$0") stop
EOF
}

#=============================================================================
# Main
#=============================================================================
main() {
    local command="${1:-}"

    if [[ -z "$command" ]]; then
        usage
        exit 1
    fi

    shift

    case "${command}" in
        start)
            start_setup "$@"
            ;;
        stop)
            stop_setup
            ;;
        logs)
            show_logs
            ;;
        follow)
            follow_logs
            ;;
        status)
            check_status
            ;;
        -h|--help|help)
            usage
            ;;
        *)
            log_error "Unknown command: ${command}"
            usage
            exit 1
            ;;
    esac
}

main "$@"
