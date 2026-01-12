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
LOCAL_BUILD=false
RESUME_FROM=""
NEW_ARGS=()
while (( "$#" )); do
  case "$1" in
    --build)
      FORCE_BUILD=true
      log_info "Force build option detected. Services will be rebuilt."
      shift
      ;;
    --local|--dev)
      LOCAL_BUILD=true
      log_info "Local optimization option detected."
      shift
      ;;
    --resume|--resume-from)
      if [ -z "${2:-}" ]; then
        log_error "--resume requires a step name"
        exit 1
      fi
      RESUME_FROM="$2"
      shift 2
      ;;
    *)
      NEW_ARGS+=("$1")
      shift
      ;;
  esac
done
# Overwrite original arguments with the filtered list (removes local flags from $@)
set -- "${NEW_ARGS[@]}"

STEP_ORDER=(
    minio
    core
    kms-signer
    database
    host-node
    gateway-node
    coprocessor
    gateway-mocked-payment
    gateway-sc
    sync-addresses
    restart-coprocessor
    gateway-mocked-payment-approvals
    host-sc
    reset-kms-connector
    kms-connector
    relayer
    test-suite
)

step_index() {
    local step="$1"
    local idx=1
    for s in "${STEP_ORDER[@]}"; do
        if [ "$s" = "$step" ]; then
            echo "$idx"
            return 0
        fi
        idx=$((idx + 1))
    done
    echo 0
}

RESUME_INDEX=0
if [ -n "$RESUME_FROM" ]; then
    RESUME_INDEX=$(step_index "$RESUME_FROM")
    if [ "$RESUME_INDEX" -eq 0 ]; then
        log_error "Unknown resume step: $RESUME_FROM"
        log_info "Valid steps: ${STEP_ORDER[*]}"
        exit 1
    fi
    log_info "Resuming deployment from step: ${RESUME_FROM}"
fi

should_run_step() {
    local step="$1"
    if [ -z "$RESUME_FROM" ]; then
        return 0
    fi
    local idx
    idx=$(step_index "$step")
    if [ "$idx" -eq 0 ]; then
        log_error "Unknown step: $step"
        exit 1
    fi
    if [ "$idx" -lt "$RESUME_INDEX" ]; then
        log_warn "Skipping step ${step} (resume from ${RESUME_FROM})"
        return 1
    fi
    return 0
}

if [ "$LOCAL_BUILD" = true ]; then
    log_info "Enabling local BuildKit cache and disabling provenance attestations."
    export DOCKER_BUILDKIT=1
    export COMPOSE_DOCKER_CLI_BUILD=1
    export BUILDX_NO_DEFAULT_ATTESTATIONS=1
    export DOCKER_BUILD_PROVENANCE=false
    export FHEVM_CARGO_PROFILE=local
    export RELAYER_VERSION=local
    export CONNECTOR_DB_MIGRATION_VERSION=local
    export CONNECTOR_GW_LISTENER_VERSION=local
    export CONNECTOR_KMS_WORKER_VERSION=local
    export CONNECTOR_TX_SENDER_VERSION=local
    export COPROCESSOR_DB_MIGRATION_VERSION=local
    export COPROCESSOR_GW_LISTENER_VERSION=local
    export COPROCESSOR_HOST_LISTENER_VERSION=local
    export COPROCESSOR_TX_SENDER_VERSION=local
    export COPROCESSOR_TFHE_WORKER_VERSION=local
    export COPROCESSOR_SNS_WORKER_VERSION=local
    export COPROCESSOR_ZKPROOF_WORKER_VERSION=local
    export GATEWAY_VERSION=local
    export HOST_VERSION=local
    export TEST_SUITE_VERSION=local
    FHEVM_BUILDX_CACHE_DIR="${FHEVM_BUILDX_CACHE_DIR:-.buildx-cache}"
    mkdir -p "$FHEVM_BUILDX_CACHE_DIR"
    ensure_buildx_driver() {
        local driver
        driver="$(docker buildx inspect --format '{{.Driver}}' 2>/dev/null || true)"
        if [[ "$driver" == "docker" || -z "$driver" ]]; then
            log_warn "Buildx driver is docker; cache export isn't supported. Switching to docker-container builder."
            local builder_name
            builder_name="$(docker buildx ls | awk 'NR>1 && $2 == "docker-container" {print $1; exit}')"
            builder_name="${builder_name%\*}"
            if [[ -z "$builder_name" ]]; then
                builder_name="fhevm-buildx"
                docker buildx create --name "$builder_name" --driver docker-container >/dev/null
            fi
            docker buildx use "$builder_name" >/dev/null
            export BUILDX_BUILDER="$builder_name"
            docker buildx inspect --bootstrap >/dev/null
        fi
    }
    ensure_buildx_driver
    FHEVM_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"
    ZAMA_ROOT="$(cd "${FHEVM_ROOT}/.." && pwd)"
    export CONSOLE_REPO="${CONSOLE_REPO:-${ZAMA_ROOT}/console}"
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

# Function to run a single compose service once (no deps), useful for one-shot tasks
run_compose_service_once() {
    local component=$1
    local service_name=$2
    local service_desc=$3
    local env_file="$SCRIPT_DIR/../env/staging/.env.$component.local"
    local compose_file="$SCRIPT_DIR/../docker-compose/$component-docker-compose.yml"

    log_info "Running $service_desc using local environment file..."
    log_info "Using environment file: $env_file"

    local build_flag=()
    if [ "$FORCE_BUILD" = true ] || [ "$LOCAL_BUILD" = true ]; then
        build_flag=(--build)
    fi

    if ! docker compose -p "${PROJECT}" --env-file "$env_file" -f "$compose_file" run --rm --no-deps "${build_flag[@]}" "$service_name"; then
        log_error "Failed to run $service_desc"
        return 1
    fi
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

if [ -z "$RESUME_FROM" ]; then
    cleanup "$@"
    prepare_all_env_files
    prepare_local_config_relayer
else
    log_warn "Resume requested; skipping cleanup and local env regeneration."
fi

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

if should_run_step "minio"; then
    run_compose "minio" "MinIO Services" \
        "${PROJECT}-minio:running" \
        "${PROJECT}-minio-setup:complete"
    get_minio_ip "fhevm-minio"
fi

# Run KMS core service (External dependency)
if should_run_step "core"; then
    run_compose "core" "Core Services" "kms-core:running"
fi

# Setup KMS signer address used in Gateway and Host contracts
if should_run_step "kms-signer"; then
    sleep 5
    ${SCRIPT_DIR}/setup-kms-signer-address.sh
fi

# Run database shared by Coprocessor and KMS connector services
if should_run_step "database"; then
    run_compose "database" "Database service" "coprocessor-and-kms-db:running"
fi

if [ "$FORCE_BUILD" = true ]; then
    RUN_COMPOSE=run_compose_with_build
else
    RUN_COMPOSE=run_compose
fi

update_env_var() {
    local file=$1
    local key=$2
    local value=$3
    if [ -z "$value" ]; then
        log_warn "Skipping update for ${key} in ${file} (empty value)"
        return
    fi
    if grep -q "^${key}=" "$file"; then
        sed -i.bak "s|^${key}=.*|${key}=${value}|" "$file"
    else
        echo "${key}=${value}" >> "$file"
    fi
}

sync_gateway_addresses_from_volume() {
    local volume_name="${PROJECT}_addresses-volume"
    local addresses_file="/data/.env.gateway"
    local address_content
    address_content=$(docker run --rm -v "${volume_name}:/data" alpine cat "${addresses_file}" 2>/dev/null || true)

    if [ -z "$address_content" ]; then
        log_error "Failed to read ${addresses_file} from volume ${volume_name}"
        return 1
    fi

    local gateway_config_address
    local kms_generation_address
    local protocol_payment_address
    local decryption_address
    local input_verification_address
    local pauser_set_address

    gateway_config_address=$(echo "$address_content" | awk -F= '/^GATEWAY_CONFIG_ADDRESS=/{print $2}')
    kms_generation_address=$(echo "$address_content" | awk -F= '/^KMS_GENERATION_ADDRESS=/{print $2}')
    protocol_payment_address=$(echo "$address_content" | awk -F= '/^PROTOCOL_PAYMENT_ADDRESS=/{print $2}')
    decryption_address=$(echo "$address_content" | awk -F= '/^DECRYPTION_ADDRESS=/{print $2}')
    input_verification_address=$(echo "$address_content" | awk -F= '/^INPUT_VERIFICATION_ADDRESS=/{print $2}')
    pauser_set_address=$(echo "$address_content" | awk -F= '/^PAUSER_SET_ADDRESS=/{print $2}')

    local gateway_env="$SCRIPT_DIR/../env/staging/.env.gateway-sc.local"
    local host_env="$SCRIPT_DIR/../env/staging/.env.host-sc.local"
    local kms_env="$SCRIPT_DIR/../env/staging/.env.kms-connector.local"
    local copro_env="$SCRIPT_DIR/../env/staging/.env.coprocessor.local"
    local relayer_env="$SCRIPT_DIR/../env/staging/.env.relayer.local"
    local test_env="$SCRIPT_DIR/../env/staging/.env.test-suite.local"
    local mocked_env="$SCRIPT_DIR/../env/staging/.env.gateway-mocked-payment.local"
    local relayer_config="$SCRIPT_DIR/../config/relayer/local.yaml.local"

    update_env_var "$gateway_env" "GATEWAY_CONFIG_ADDRESS" "$gateway_config_address"
    update_env_var "$gateway_env" "KMS_GENERATION_ADDRESS" "$kms_generation_address"
    update_env_var "$gateway_env" "PAUSER_SET_ADDRESS" "$pauser_set_address"

    update_env_var "$host_env" "DECRYPTION_ADDRESS" "$decryption_address"
    update_env_var "$host_env" "INPUT_VERIFICATION_ADDRESS" "$input_verification_address"

    update_env_var "$kms_env" "KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS" "$decryption_address"
    update_env_var "$kms_env" "KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS" "$gateway_config_address"
    update_env_var "$kms_env" "KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS" "$kms_generation_address"
    update_env_var "$kms_env" "KMS_CONNECTOR_INPUT_VERIFICATION_CONTRACT__ADDRESS" "$input_verification_address"

    update_env_var "$copro_env" "INPUT_VERIFICATION_ADDRESS" "$input_verification_address"
    update_env_var "$copro_env" "KMS_GENERATION_ADDRESS" "$kms_generation_address"

    update_env_var "$relayer_env" "APP_GATEWAY__CONTRACTS__DECRYPTION_ADDRESS" "$decryption_address"
    update_env_var "$relayer_env" "APP_GATEWAY__CONTRACTS__INPUT_VERIFICATION_ADDRESS" "$input_verification_address"

    update_env_var "$test_env" "DECRYPTION_ADDRESS" "$decryption_address"
    update_env_var "$test_env" "INPUT_VERIFICATION_ADDRESS" "$input_verification_address"

    update_env_var "$mocked_env" "PROTOCOL_PAYMENT_ADDRESS" "$protocol_payment_address"

    if [ -f "$relayer_config" ]; then
        sed -i.bak "s|decryption_address: \".*\"|decryption_address: \"${decryption_address}\"|" "$relayer_config"
        sed -i.bak "s|input_verification_address: \".*\"|input_verification_address: \"${input_verification_address}\"|" "$relayer_config"
        if grep -q "gateway_config_address:" "$relayer_config"; then
            sed -i.bak "s|gateway_config_address: \".*\"|gateway_config_address: \"${gateway_config_address}\"|" "$relayer_config"
        fi
    fi

    log_info "Synchronized gateway contract addresses from ${volume_name}"
}

reset_kms_connector_requests() {
    log_info "Resetting KMS connector request tables to avoid request-id collisions"
    if ! docker exec -i coprocessor-and-kms-db psql -U postgres -d postgres -tAc \
        "SELECT 1 FROM pg_database WHERE datname='kms-connector'" | grep -q 1; then
        log_warn "kms-connector database not found yet; skipping reset"
        return 0
    fi
    docker exec -i coprocessor-and-kms-db psql -U postgres -d "kms-connector" -v ON_ERROR_STOP=1 -c \
        "TRUNCATE public_decryption_requests, public_decryption_responses, user_decryption_requests, user_decryption_responses;"
}

# Run Host and Gateway nodes
if should_run_step "host-node"; then
    ${RUN_COMPOSE} "host-node" "Host node service" "host-node:running"
fi
if should_run_step "gateway-node"; then
    ${RUN_COMPOSE} "gateway-node" "Gateway node service" "gateway-node:running"
fi

# Run coprocessor services
if should_run_step "coprocessor"; then
    ${RUN_COMPOSE} "coprocessor" "Coprocessor Services" \
        "coprocessor-and-kms-db:running" \
        "coprocessor-db-migration:complete" \
        "coprocessor-host-listener:running" \
        "coprocessor-gw-listener:running" \
        "coprocessor-tfhe-worker:running" \
        "coprocessor-zkproof-worker:running" \
        "coprocessor-sns-worker:running" \
        "coprocessor-transaction-sender:running"
fi

# Setup mocked payment contracts (deploy ZamaOFT before gateway-sc)
if should_run_step "gateway-mocked-payment"; then
    run_compose_service_once "gateway-mocked-payment" "gateway-deploy-mocked-zama-oft" "Gateway mocked ZamaOFT deploy"
fi

# Setup Gateway contracts, which will trigger the KMS materials generation. Note
# that the key generation may take a few seconds to complete, meaning that executing
# the e2e tests too soon may fail if the materials are not ready. Hence, the following
# setup is placed here to favor proper sequencing.
if should_run_step "gateway-sc"; then
    ${RUN_COMPOSE} "gateway-sc" "Gateway contracts" \
        "gateway-sc-deploy:complete" \
        "gateway-sc-add-network:complete" \
        "gateway-sc-trigger-keygen:complete" \
        "gateway-sc-trigger-crsgen:complete" \
        "gateway-sc-add-pausers:complete"
fi

if should_run_step "sync-addresses"; then
    sync_gateway_addresses_from_volume
fi

# Restart coprocessor services so they pick up updated addresses from env files
restart_coprocessor_after_sync() {
    local env_file="$SCRIPT_DIR/../env/staging/.env.coprocessor.local"
    local compose_file="$SCRIPT_DIR/../docker-compose/coprocessor-docker-compose.yml"
    log_info "Restarting coprocessor services to pick up synced contract addresses..."
    docker compose -p "${PROJECT}" --env-file "$env_file" -f "$compose_file" up -d --force-recreate --no-deps \
        coprocessor-host-listener \
        coprocessor-host-listener-poller \
        coprocessor-gw-listener \
        coprocessor-tfhe-worker \
        coprocessor-zkproof-worker \
        coprocessor-sns-worker \
        coprocessor-transaction-sender
}

if should_run_step "restart-coprocessor"; then
    restart_coprocessor_after_sync
fi

# Set relayer allowances after protocol payment address is known
if should_run_step "gateway-mocked-payment-approvals"; then
    run_compose_service_once "gateway-mocked-payment" "gateway-set-relayer-mocked-payment" "Gateway mocked payment approvals"
fi

# Setup Host contracts
if should_run_step "host-sc"; then
    ${RUN_COMPOSE} "host-sc" "Host contracts" "host-sc-deploy:complete" "host-sc-add-pausers:complete"
fi

# Reset KMS connector request tables after gateway redeploy to avoid ID collisions
if should_run_step "reset-kms-connector"; then
    reset_kms_connector_requests
fi

# Run KMS connector services (after Gateway contracts are deployed)
if should_run_step "kms-connector"; then
    ${RUN_COMPOSE} "kms-connector" "KMS Connector Services" \
        "coprocessor-and-kms-db:running" \
        "kms-connector-db-migration:complete" \
        "kms-connector-gw-listener:running" \
        "kms-connector-kms-worker:running" \
        "kms-connector-tx-sender:running"
fi

# Build local relayer image if requested
if [ "$LOCAL_BUILD" = true ] && should_run_step "relayer"; then
    # Initialize database and run migrations BEFORE build
    # (sqlx needs live DB with schema to verify queries at compile time)
    log_info "Initializing relayer database for sqlx compile-time verification..."

    # Create relayer_db database
    docker exec coprocessor-and-kms-db psql -U postgres -tc \
        "SELECT 1 FROM pg_database WHERE datname = 'relayer_db'" | grep -q 1 || \
        docker exec coprocessor-and-kms-db psql -U postgres -c "CREATE DATABASE relayer_db"

    # Run migrations using the migration image
    log_info "Running relayer database migrations..."
    docker run --rm \
        --network "${PROJECT}_default" \
        -e DATABASE_URL="postgresql://postgres:postgres@coprocessor-and-kms-db:5432/relayer_db" \
        -e MAX_ATTEMPTS=1 \
        ghcr.io/zama-ai/console/relayer-migrate:${RELAYER_VERSION:-latest} || {
            # If pre-built image doesn't exist, build and run it
            log_warn "Pre-built migration image not found, building locally..."
            docker build -t relayer-migrate-local \
                -f "${ZAMA_ROOT:-$(dirname "$SCRIPT_DIR")/../../..}/console/docker/relayer-migrate/Dockerfile" \
                "${ZAMA_ROOT:-$(dirname "$SCRIPT_DIR")/../../..}/console"
            docker run --rm \
                --network "${PROJECT}_default" \
                -e DATABASE_URL="postgresql://postgres:postgres@coprocessor-and-kms-db:5432/relayer_db" \
                -e MAX_ATTEMPTS=1 \
                relayer-migrate-local
        }

    log_info "Building local relayer image..."
    "${SCRIPT_DIR}/build-relayer-local.sh"
    export RELAYER_VERSION=local
fi

# Run Relayer (External dependency)
if should_run_step "relayer"; then
    ${RUN_COMPOSE} "relayer" "Relayer Services" \
        "${PROJECT}-relayer-db-init:complete" \
        "${PROJECT}-relayer-db-migration:complete" \
        "${PROJECT}-relayer:running"
fi

# Run Test Suite container
if should_run_step "test-suite"; then
    ${RUN_COMPOSE} "test-suite" "Test Suite E2E Tests" "${PROJECT}-test-suite-e2e-debug:running"
fi

log_info "All services started successfully!"
