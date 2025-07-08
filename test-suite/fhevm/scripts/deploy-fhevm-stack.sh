#!/bin/bash

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Global project vars
PROJECT="fhevm"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Argument Parsing
FORCE_BUILD=false
NEW_ARGS=()
for arg in "$@"; do
  if [[ "$arg" == "--build" ]]; then
    FORCE_BUILD=true
    log_info "Force build option detected. Services will be rebuilt."
  else
    NEW_ARGS+=("$arg")
  fi
done
# Overwrite original arguments with the filtered list (removes --build from $@)
set -- "${NEW_ARGS[@]}"

# Function to check if services are ready based on expected state
wait_for_service() {
    local compose_file=$1
    local service_name=$2
    local expect_running="${3:-true}"  # By default, expect service to stay running
    local max_retries=30
    local retry_interval=5

    if [[ "$expect_running" == "true" ]]; then
        log_info "Waiting for $service_name to be running..."
    else
        log_info "Waiting for $service_name to complete..."
    fi

    for ((i=1; i<=max_retries; i++)); do
        # Check container status using docker container directly to handle completed containers
        local container_id=$(docker ps -a --filter name="${service_name}$" --format "{{.ID}}")

        if [[ -z "$container_id" ]]; then
            log_warn "Container for $service_name not found, waiting..."
            sleep "$retry_interval"
            continue
        fi

        local status=$(docker inspect --format "{{.State.Status}}" "$container_id")
        local exit_code=$(docker inspect --format "{{.State.ExitCode}}" "$container_id")

        # Check if service meets the expected state
        if [[ "$expect_running" == "true" && "$status" == "running" ]]; then
            log_info "$service_name is now running"
            return 0
        elif [[ "$expect_running" == "false" && "$status" == "exited" && "$exit_code" == "0" ]]; then
            log_info "$service_name completed successfully"
            return 0
        elif [[ "$status" == "exited" && "$exit_code" != "0" ]]; then
            log_error "$service_name failed with exit code $exit_code"
            docker logs "$container_id"
            return 1
        fi

        # Still waiting
        if [ "$i" -lt "$max_retries" ]; then
            log_warn "$service_name not ready yet (status: $status), waiting ${retry_interval}s... (${i}/${max_retries})"
            sleep "$retry_interval"
        else
            log_error "$service_name failed to reach desired state within the expected time"
            docker logs "$container_id"
            return 1
        fi
    done
}
# Function to prepare the local environment file for a component
prepare_local_env_file() {
    local component=$1
    local base_env_file="$SCRIPT_DIR/../env/staging/.env.$component"
    local local_env_file="$SCRIPT_DIR/../env/staging/.env.$component.local"

    if [[ ! -f "$base_env_file" ]]; then
        echo -e "${RED}[ERROR]${NC} Base environment file for $component not found: $base_env_file" >&2
        return 1
    else
        echo -e "${GREEN}[INFO]${NC} Creating/updating local environment file for $component..." >&2
        cp "$base_env_file" "$local_env_file"
    fi

    printf "%s" "$local_env_file"
}

prepare_local_config_relayer() {
    local base_config_file="$SCRIPT_DIR/../config/relayer/local.yaml"
    local local_config_file="$SCRIPT_DIR/../config/relayer/local.yaml.local"

    if [[ ! -f "$base_config_file" ]]; then
        echo -e "${RED}[ERROR]${NC} Base configuration file for relayer not found: $base_config_file" >&2
        return 1
    else
        # Always copy the base file to the local file
        echo -e "${GREEN}[INFO]${NC} Creating/updating local configuration file for relayer..." >&2
        cp "$base_config_file" "$local_config_file"
    fi

    printf "%s" "$local_config_file"
}

# Add this function after prepare_local_env_file
prepare_all_env_files() {
    log_info "Preparing all local environment files..."

    local components=("minio" "core" "gateway" "host" "connector" "coprocessor" "relayer" "test-suite")

    for component in "${components[@]}"; do
        prepare_local_env_file "$component" > /dev/null
    done

    log_info "All local environment files prepared successfully"
}

# Function to start an entire docker-compose file and wait for specified services
run_compose() {
    local component=$1
    local service_desc=$2
    local env_file="$SCRIPT_DIR/../env/staging/.env.$component.local"
    local compose_file="$SCRIPT_DIR/../docker-compose/$component-docker-compose.yml"
    shift 2

    local services=("$@")
    local service_states=()
    local service_names=()

    # Parse service names and states
    for arg in "${services[@]}"; do
        IFS=':' read -r name state <<< "$arg"
        service_names+=("$name")
        service_states+=("$state")
    done

    log_info "Starting $service_desc using local environment file..."
    log_info "Using environment file: $env_file"

    # Start all services
    if ! docker compose -p "${PROJECT}" --env-file "$env_file" -f "$compose_file" up -d; then
        log_error "Failed to start $service_desc"
        return 1
    fi

    # Wait for each specified service
    for i in "${!service_names[@]}"; do
        local name="${service_names[$i]}"
        local expect_running=true

        if [[ "${service_states[$i]}" == "complete" ]]; then
            expect_running=false
        fi

        wait_for_service "$compose_file" "$name" "$expect_running"
        if [ $? -ne 0 ]; then
            return 1
        fi
    done
}

# Function to start an entire docker-compose file with --build and wait for specified services
run_compose_with_build() {
    local component=$1
    local service_desc=$2
    local env_file="$SCRIPT_DIR/../env/staging/.env.$component.local"
    local compose_file="$SCRIPT_DIR/../docker-compose/$component-docker-compose.yml"
    shift 2

    local services=("$@")
    local service_states=()
    local service_names=()

    # Parse service names and states
    for arg in "${services[@]}"; do
        IFS=':' read -r name state <<< "$arg"
        service_names+=("$name")
        service_states+=("$state")
    done

    log_info "Building and starting $service_desc using local environment file..."
    log_info "Using environment file: $env_file"

    # Start all services with --build
    if ! docker compose -p "${PROJECT}" --env-file "$env_file" -f "$compose_file" up --build -d; then
        log_error "Failed to build and start $service_desc"
        return 1
    fi

    # Wait for each specified service
    for i in "${!service_names[@]}"; do
        local name="${service_names[$i]}"
        local expect_running=true

        if [[ "${service_states[$i]}" == "complete" ]]; then
            expect_running=false
        fi

        wait_for_service "$compose_file" "$name" "$expect_running"
        if [ $? -ne 0 ]; then
            return 1
        fi
    done
}

get_minio_ip() {
    # Get IP address of minio container and update AWS_ENDPOINT_URL
    # IMPORTANT: this is a workaround as sns-worker is not able to resolve the container name
    local minio_container_name=$1
    local minio_ip
    minio_ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$minio_container_name")
    local coprocessor_file="$SCRIPT_DIR/../env/staging/.env.coprocessor.local"
    if [ -n "$minio_ip" ]; then
    echo "Found $minio_container_name container IP: $minio_ip"
    sed -i.bak "s|AWS_ENDPOINT_URL=http://[^:]*:9000|AWS_ENDPOINT_URL=http://$minio_ip:9000|" \
            "$coprocessor_file"
    echo "Updated AWS_ENDPOINT_URL to http://$minio_ip:9000"
    else
    echo "Error: Could not find IP address for $minio_container_name container"
    exit 1
    fi
}

cleanup() {
    log_warn "Setup new environment, cleaning up..."
    docker compose -p "${PROJECT}" down -v --remove-orphans

    # Only exit if requested
    if [ "$1" = "exit" ]; then
        exit 1
    fi
}

cleanup "$@"

prepare_all_env_files
prepare_local_config_relayer

log_info "Deploying FHEVM Stack..."

BUILD_TAG=""
if [ "$FORCE_BUILD" = true ]; then
  BUILD_TAG=" (local build)"
fi

log_info "FHEVM Stack Versions:"
log_info "FHEVM Contracts:"
log_info "  gateway-contracts:${GATEWAY_VERSION}${BUILD_TAG}"
log_info "  host-contracts:${HOST_VERSION}${BUILD_TAG}"
log_info "FHEVM Coprocessor Services:"
log_info "  coprocessor/db-migration:${DB_MIGRATION_VERSION}${BUILD_TAG}"
log_info "  coprocessor/gw-listener:${GW_LISTENER_VERSION}${BUILD_TAG}"
log_info "  coprocessor/host-listener:${HOST_LISTENER_VERSION}${BUILD_TAG}"
log_info "  coprocessor/tx-sender:${TX_SENDER_VERSION}${BUILD_TAG}"
log_info "  coprocessor/tfhe-worker:${TFHE_WORKER_VERSION}${BUILD_TAG}"
log_info "  coprocessor/sns-worker:${SNS_WORKER_VERSION}${BUILD_TAG}"
log_info "  coprocessor/zkproof-worker:${ZKPROOF_WORKER_VERSION}${BUILD_TAG}"
log_info "FHEVM KMS Connector Services:"
log_info "  kms-connector:${CONNECTOR_VERSION}${BUILD_TAG}"
log_info "FHEVM Test Suite:"
log_info "  test-suite/e2e:${TEST_SUITE_VERSION}${BUILD_TAG}"
log_info "External Dependencies:"
log_info "  kms-service:${CORE_VERSION}"
log_info "  fhevm-relayer:${RELAYER_VERSION}"

run_compose "minio" "MinIO Services" \
    "${PROJECT}-minio:running" \
    "${PROJECT}-minio-setup:complete"

# External dependency - KMS Core
run_compose "core" "Core Services" \
    "kms-core:running" \
    "${PROJECT}-generate-fhe-keys:complete"

"${SCRIPT_DIR}/update-kms-keys.sh"

if [ "$FORCE_BUILD" = true ]; then
  run_compose_with_build "gateway" "Gateway Network Services" \
    "${PROJECT}-gateway-node:running" \
    "${PROJECT}-gateway-sc-deploy:complete" \
    "${PROJECT}-gateway-sc-add-network:complete"
else
  run_compose "gateway" "Gateway Network Services" \
    "${PROJECT}-gateway-node:running" \
    "${PROJECT}-gateway-sc-deploy:complete" \
    "${PROJECT}-gateway-sc-add-network:complete"
fi

if [ "$FORCE_BUILD" = true ]; then
  run_compose_with_build "host" "Host Network Services" \
    "${PROJECT}-host-node:running" \
    "${PROJECT}-host-sc-deploy:complete"
else
  run_compose "host" "Host Network Services" \
    "${PROJECT}-host-node:running" \
    "${PROJECT}-host-sc-deploy:complete"
fi

get_minio_ip "fhevm-minio"

if [ "$FORCE_BUILD" = true ]; then
  run_compose_with_build "coprocessor" "Coprocessor Services" \
    "${PROJECT}-coprocessor-db:running" \
    "${PROJECT}-key-downloader:complete" \
    "${PROJECT}-db-migration:complete" \
    "${PROJECT}-host-listener:running" \
    "${PROJECT}-gw-listener:running" \
    "${PROJECT}-tfhe-worker:running" \
    "${PROJECT}-zkproof-worker:running" \
    "${PROJECT}-sns-worker:running" \
    "${PROJECT}-transaction-sender:running"
else
  run_compose "coprocessor" "Coprocessor Services" \
    "${PROJECT}-coprocessor-db:running" \
    "${PROJECT}-key-downloader:complete" \
    "${PROJECT}-db-migration:complete" \
    "${PROJECT}-host-listener:running" \
    "${PROJECT}-gw-listener:running" \
    "${PROJECT}-tfhe-worker:running" \
    "${PROJECT}-zkproof-worker:running" \
    "${PROJECT}-sns-worker:running" \
    "${PROJECT}-transaction-sender:running"
fi

if [ "$FORCE_BUILD" = true ]; then
  run_compose_with_build "connector" "Connector Services" \
  "kms-connector-gw-listener:running" \
  "kms-connector-kms-worker:running" \
  "kms-connector-tx-sender:running"
else
  run_compose "connector" "Connector Services" \
  "kms-connector-gw-listener:running" \
  "kms-connector-kms-worker:running" \
  "kms-connector-tx-sender:running"
fi

# External dependency - Relayer
run_compose "relayer" "Relayer Services" \
    "${PROJECT}-relayer:running"

if [ "$FORCE_BUILD" = true ]; then
  run_compose_with_build "test-suite" "Test Suite E2E Tests" \
    "${PROJECT}-test-suite-e2e-debug:running"
else
  run_compose "test-suite" "Test Suite E2E Tests" \
    "${PROJECT}-test-suite-e2e-debug:running"
fi

log_info "All services started successfully!"
