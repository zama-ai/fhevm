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

# Deployment steps registry - defines all steps in execution order
# These names are used for --resume functionality
DEPLOYMENT_STEPS=(
    "minio"
    "core"
    "kms-signer"
    "database"
    "host-node"
    "gateway-node"
    "coprocessor"
    "kms-connector"
    "gateway-mocked-payment"
    "gateway-sc"
    "host-sc"
    "relayer"
    "test-suite"
)

# Helper to get index of step in DEPLOYMENT_STEPS array
get_step_index() {
    local step_name=$1
    for i in "${!DEPLOYMENT_STEPS[@]}"; do
        if [[ "${DEPLOYMENT_STEPS[$i]}" == "$step_name" ]]; then
            echo "$i"
            return 0
        fi
    done
    echo "-1"
    return 1
}

# Helper to check if step should be skipped (when resuming)
should_skip_step() {
    local current_step=$1
    if [[ -z "$RESUME_STEP" ]]; then
        return 1  # No resume, don't skip
    fi
    local resume_index=$(get_step_index "$RESUME_STEP")
    local current_index=$(get_step_index "$current_step")
    [[ "$current_index" -lt "$resume_index" ]]
}

# Argument Parsing
FORCE_BUILD=false
LOCAL_BUILD=false
RESUME_STEP=""
RESUME_FLAG_DETECTED=false
NEW_ARGS=()

for arg in "$@"; do
  if [[ "$arg" == "--build" ]]; then
    FORCE_BUILD=true
    log_info "Force build option detected. Services will be rebuilt."
  elif [[ "$arg" == "--local" || "$arg" == "--dev" ]]; then
    LOCAL_BUILD=true
    log_info "Local optimization option detected."
  elif [[ "$arg" == "--resume" ]]; then
    RESUME_FLAG_DETECTED=true
  elif [[ "$RESUME_FLAG_DETECTED" == true ]]; then
    RESUME_STEP="$arg"
    RESUME_FLAG_DETECTED=false
    # Validate step name
    if [[ $(get_step_index "$RESUME_STEP") -eq -1 ]]; then
      log_error "Invalid resume step: $RESUME_STEP"
      log_error "Valid steps are: ${DEPLOYMENT_STEPS[*]}"
      exit 1
    fi
    log_info "Resume mode: starting from step '$RESUME_STEP'"
  else
    NEW_ARGS+=("$arg")
  fi
done

# Check for incomplete --resume flag
if [[ "$RESUME_FLAG_DETECTED" == true ]]; then
  log_error "--resume requires a step name"
  log_error "Valid steps are: ${DEPLOYMENT_STEPS[*]}"
  exit 1
fi

# Overwrite original arguments with the filtered list (removes local flags from $@)
set -- "${NEW_ARGS[@]}"

if [ "$LOCAL_BUILD" = true ]; then
    log_info "Enabling local BuildKit cache and disabling provenance attestations."
    export DOCKER_BUILDKIT=1
    export COMPOSE_DOCKER_CLI_BUILD=1
    export BUILDX_NO_DEFAULT_ATTESTATIONS=1
    export DOCKER_BUILD_PROVENANCE=false
    export FHEVM_CARGO_PROFILE=local
    FHEVM_BUILDX_CACHE_DIR="${FHEVM_BUILDX_CACHE_DIR:-.buildx-cache}"
    mkdir -p "$FHEVM_BUILDX_CACHE_DIR"
    set_local_cache_vars() {
        local service_name="$1"
        local service_key
        service_key=$(echo "${service_name//-/_}" | tr '[:lower:]' '[:upper:]')
        local cache_dir="${FHEVM_BUILDX_CACHE_DIR}/${service_name}"
        mkdir -p "$cache_dir"
        export "FHEVM_CACHE_FROM_${service_key}=type=local,src=${cache_dir}"
        export "FHEVM_CACHE_TO_${service_key}=type=local,dest=${cache_dir},mode=max"
    }
    # Unified coprocessor workspace cache (all services share one cache since they
    # are built from a single Dockerfile.workspace with multi-stage targets)
    coprocessor_cache_dir="${FHEVM_BUILDX_CACHE_DIR}/coprocessor"
    mkdir -p "$coprocessor_cache_dir"
    export "FHEVM_CACHE_FROM_COPROCESSOR=type=local,src=${coprocessor_cache_dir}"
    export "FHEVM_CACHE_TO_COPROCESSOR=type=local,dest=${coprocessor_cache_dir},mode=max"

    # Unified kms-connector workspace cache (gw-listener, kms-worker, tx-sender
    # share Dockerfile.workspace; db-migration uses separate Dockerfile)
    kms_connector_cache_dir="${FHEVM_BUILDX_CACHE_DIR}/kms-connector"
    mkdir -p "$kms_connector_cache_dir"
    export "FHEVM_CACHE_FROM_KMS_CONNECTOR=type=local,src=${kms_connector_cache_dir}"
    export "FHEVM_CACHE_TO_KMS_CONNECTOR=type=local,dest=${kms_connector_cache_dir},mode=max"

    # Other services still use individual caches
    LOCAL_CACHE_SERVICES=(
        gateway-deploy-mocked-zama-oft
        gateway-sc-add-network
        gateway-sc-add-pausers
        gateway-sc-deploy
        gateway-sc-pause
        gateway-sc-trigger-crsgen
        gateway-sc-trigger-keygen
        gateway-sc-unpause
        gateway-set-relayer-mocked-payment
        host-sc-add-pausers
        host-sc-deploy
        host-sc-pause
        host-sc-unpause
        kms-connector-db-migration
        test-suite-e2e-debug
    )
    for service_name in "${LOCAL_CACHE_SERVICES[@]}"; do
        set_local_cache_vars "$service_name"
    done
fi

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

    local components=("minio" "database" "core" "gateway-node" "host-node" "gateway-sc" "gateway-mocked-payment" "host-sc" "kms-connector" "coprocessor" "relayer" "test-suite")

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

# Only run cleanup if NOT in resume mode
if [[ -z "$RESUME_STEP" ]]; then
    cleanup "$@"
else
    log_info "Resume mode: skipping cleanup to preserve existing containers/volumes"
fi

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
log_info "  coprocessor/db-migration:${COPROCESSOR_DB_MIGRATION_VERSION}${BUILD_TAG}"
log_info "  coprocessor/gw-listener:${COPROCESSOR_GW_LISTENER_VERSION}${BUILD_TAG}"
log_info "  coprocessor/host-listener:${COPROCESSOR_HOST_LISTENER_VERSION}${BUILD_TAG}"
log_info "  coprocessor/poller:${COPROCESSOR_HOST_LISTENER_VERSION}${BUILD_TAG}"
log_info "  coprocessor/tx-sender:${COPROCESSOR_TX_SENDER_VERSION}${BUILD_TAG}"
log_info "  coprocessor/tfhe-worker:${COPROCESSOR_TFHE_WORKER_VERSION}${BUILD_TAG}"
log_info "  coprocessor/sns-worker:${COPROCESSOR_SNS_WORKER_VERSION}${BUILD_TAG}"
log_info "  coprocessor/zkproof-worker:${COPROCESSOR_ZKPROOF_WORKER_VERSION}${BUILD_TAG}"
log_info "FHEVM KMS Connector Services:"
log_info "  kms-connector/db-migration:${CONNECTOR_DB_MIGRATION_VERSION}${BUILD_TAG}"
log_info "  kms-connector/gw-listener:${CONNECTOR_GW_LISTENER_VERSION}${BUILD_TAG}"
log_info "  kms-connector/kms-worker:${CONNECTOR_KMS_WORKER_VERSION}${BUILD_TAG}"
log_info "  kms-connector/tx-sender:${CONNECTOR_TX_SENDER_VERSION}${BUILD_TAG}"
log_info "FHEVM Test Suite:"
log_info "  test-suite/e2e:${TEST_SUITE_VERSION}${BUILD_TAG}"
log_info "External Dependencies:"
log_info "  kms-core-service:${CORE_VERSION}"
log_info "  fhevm-relayer:${RELAYER_VERSION}"

# Step 1: minio
if ! should_skip_step "minio"; then
    run_compose "minio" "MinIO Services" \
        "${PROJECT}-minio:running" \
        "${PROJECT}-minio-setup:complete"
    get_minio_ip "fhevm-minio"
else
    log_info "Skipping step: minio (resuming from $RESUME_STEP)"
    # Still need minio IP for coprocessor env if container is running
    if docker ps --filter name=fhevm-minio --format "{{.Names}}" | grep -q fhevm-minio; then
        get_minio_ip "fhevm-minio"
    fi
fi

# Step 2: core
if ! should_skip_step "core"; then
    run_compose "core" "Core Services" "kms-core:running"
else
    log_info "Skipping step: core (resuming from $RESUME_STEP)"
fi

# Step 3: kms-signer
if ! should_skip_step "kms-signer"; then
    sleep 5
    ${SCRIPT_DIR}/setup-kms-signer-address.sh
else
    log_info "Skipping step: kms-signer (resuming from $RESUME_STEP)"
fi

# Step 4: database
if ! should_skip_step "database"; then
    run_compose "database" "Database service" "coprocessor-and-kms-db:running"
else
    log_info "Skipping step: database (resuming from $RESUME_STEP)"
fi

if [ "$FORCE_BUILD" = true ]; then
    RUN_COMPOSE=run_compose_with_build
else
    RUN_COMPOSE=run_compose
fi

# Step 5: host-node
if ! should_skip_step "host-node"; then
    ${RUN_COMPOSE} "host-node" "Host node service" "host-node:running"
else
    log_info "Skipping step: host-node (resuming from $RESUME_STEP)"
fi

# Step 6: gateway-node
if ! should_skip_step "gateway-node"; then
    ${RUN_COMPOSE} "gateway-node" "Gateway node service" "gateway-node:running"
else
    log_info "Skipping step: gateway-node (resuming from $RESUME_STEP)"
fi

# Step 7: coprocessor
if ! should_skip_step "coprocessor"; then
    ${RUN_COMPOSE} "coprocessor" "Coprocessor Services" \
        "coprocessor-and-kms-db:running" \
        "coprocessor-db-migration:complete" \
        "coprocessor-host-listener:running" \
        "coprocessor-gw-listener:running" \
        "coprocessor-tfhe-worker:running" \
        "coprocessor-zkproof-worker:running" \
        "coprocessor-sns-worker:running" \
        "coprocessor-transaction-sender:running"
else
    log_info "Skipping step: coprocessor (resuming from $RESUME_STEP)"
fi

# Step 8: kms-connector
if ! should_skip_step "kms-connector"; then
    ${RUN_COMPOSE} "kms-connector" "KMS Connector Services" \
        "coprocessor-and-kms-db:running" \
        "kms-connector-db-migration:complete" \
        "kms-connector-gw-listener:running" \
        "kms-connector-kms-worker:running" \
        "kms-connector-tx-sender:running"
else
    log_info "Skipping step: kms-connector (resuming from $RESUME_STEP)"
fi

# Step 9: gateway-mocked-payment
if ! should_skip_step "gateway-mocked-payment"; then
    ${RUN_COMPOSE} "gateway-mocked-payment" "Gateway mocked payment" \
        "gateway-deploy-mocked-zama-oft:complete" \
        "gateway-set-relayer-mocked-payment:complete"
else
    log_info "Skipping step: gateway-mocked-payment (resuming from $RESUME_STEP)"
fi

# Step 10: gateway-sc
# Setup Gateway contracts, which will trigger the KMS materials generation. Note
# that the key generation may take a few seconds to complete, meaning that executing
# the e2e tests too soon may fail if the materials are not ready. Hence, the following
# setup is placed here to favor proper sequencing.
if ! should_skip_step "gateway-sc"; then
    ${RUN_COMPOSE} "gateway-sc" "Gateway contracts" \
        "gateway-sc-deploy:complete" \
        "gateway-sc-add-network:complete" \
        "gateway-sc-trigger-keygen:complete" \
        "gateway-sc-trigger-crsgen:complete" \
        "gateway-sc-add-pausers:complete"
else
    log_info "Skipping step: gateway-sc (resuming from $RESUME_STEP)"
fi

# Step 11: host-sc
if ! should_skip_step "host-sc"; then
    ${RUN_COMPOSE} "host-sc" "Host contracts" "host-sc-deploy:complete" "host-sc-add-pausers:complete"
else
    log_info "Skipping step: host-sc (resuming from $RESUME_STEP)"
fi

# Step 12: relayer
if ! should_skip_step "relayer"; then
    ${RUN_COMPOSE} "relayer" "Relayer Services" \
        "${PROJECT}-relayer:running"
else
    log_info "Skipping step: relayer (resuming from $RESUME_STEP)"
fi

# Step 13: test-suite
if ! should_skip_step "test-suite"; then
    ${RUN_COMPOSE} "test-suite" "Test Suite E2E Tests" "${PROJECT}-test-suite-e2e-debug:running"
else
    log_info "Skipping step: test-suite (resuming from $RESUME_STEP)"
fi

log_info "All services started successfully!"
