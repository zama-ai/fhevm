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

# Get docker-compose component name for a step
# kms-signer has no compose file (it's just a script), returns empty
get_compose_for_step() {
    local step=$1
    case "$step" in
        minio|core|database|host-node|gateway-node|coprocessor|kms-connector|gateway-mocked-payment|gateway-sc|host-sc|relayer|test-suite)
            echo "$step"
            ;;
        *)
            echo ""
            ;;
    esac
}

# Helper to get index of step in DEPLOYMENT_STEPS array
get_step_index() {
    local step_name=$1
    local i=0
    for step in "${DEPLOYMENT_STEPS[@]}"; do
        if [[ "$step" == "$step_name" ]]; then
            echo "$i"
            return 0
        fi
        ((i++))
    done
    echo "-1"
    return 1
}

# Helper to check if step should be skipped (when resuming or targeting only one step)
should_skip_step() {
    local current_step=$1

    # --only mode: only run the specified step
    if [[ -n "$ONLY_STEP" ]]; then
        [[ "$current_step" != "$ONLY_STEP" ]]
        return
    fi

    # --resume mode: skip steps before the resume point
    if [[ -n "$RESUME_STEP" ]]; then
        local resume_index=$(get_step_index "$RESUME_STEP")
        local current_index=$(get_step_index "$current_step")
        [[ "$current_index" -lt "$resume_index" ]]
        return
    fi

    # Normal mode: don't skip anything
    return 1
}

# Argument Parsing
FORCE_BUILD=false
LOCAL_BUILD=false
MULTI_CHAIN=false
RESUME_STEP=""
ONLY_STEP=""
RESUME_FLAG_DETECTED=false
ONLY_FLAG_DETECTED=false
COPROCESSOR_COUNT=1
COPROCESSOR_THRESHOLD_OVERRIDE=""
COPROCESSOR_COUNT_FLAG_DETECTED=false
COPROCESSOR_THRESHOLD_FLAG_DETECTED=false
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
  elif [[ "$arg" == "--only" ]]; then
    ONLY_FLAG_DETECTED=true
  elif [[ "$arg" == "--multi-chain" ]]; then
    MULTI_CHAIN=true
    log_info "Multi-chain mode enabled. A second host chain (Chain B) will be deployed."
  elif [[ "$arg" == "--coprocessors" ]]; then
    COPROCESSOR_COUNT_FLAG_DETECTED=true
  elif [[ "$arg" == "--coprocessor-threshold" ]]; then
    COPROCESSOR_THRESHOLD_FLAG_DETECTED=true
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
  elif [[ "$ONLY_FLAG_DETECTED" == true ]]; then
    ONLY_STEP="$arg"
    ONLY_FLAG_DETECTED=false
    # Validate step name
    if [[ $(get_step_index "$ONLY_STEP") -eq -1 ]]; then
      log_error "Invalid step: $ONLY_STEP"
      log_error "Valid steps are: ${DEPLOYMENT_STEPS[*]}"
      exit 1
    fi
    log_info "Only mode: deploying only step '$ONLY_STEP'"
  elif [[ "$COPROCESSOR_COUNT_FLAG_DETECTED" == true ]]; then
    COPROCESSOR_COUNT="$arg"
    COPROCESSOR_COUNT_FLAG_DETECTED=false
    if ! [[ "$COPROCESSOR_COUNT" =~ ^[0-9]+$ ]] || [[ "$COPROCESSOR_COUNT" -lt 1 ]]; then
      log_error "--coprocessors expects a positive integer"
      exit 1
    fi
  elif [[ "$COPROCESSOR_THRESHOLD_FLAG_DETECTED" == true ]]; then
    COPROCESSOR_THRESHOLD_OVERRIDE="$arg"
    COPROCESSOR_THRESHOLD_FLAG_DETECTED=false
    if ! [[ "$COPROCESSOR_THRESHOLD_OVERRIDE" =~ ^[0-9]+$ ]] || [[ "$COPROCESSOR_THRESHOLD_OVERRIDE" -lt 1 ]]; then
      log_error "--coprocessor-threshold expects a positive integer"
      exit 1
    fi
  else
    NEW_ARGS+=("$arg")
  fi
done

# Check for incomplete flags
if [[ "$RESUME_FLAG_DETECTED" == true ]]; then
  log_error "--resume requires a step name"
  log_error "Valid steps are: ${DEPLOYMENT_STEPS[*]}"
  exit 1
fi

if [[ "$ONLY_FLAG_DETECTED" == true ]]; then
  log_error "--only requires a step name"
  log_error "Valid steps are: ${DEPLOYMENT_STEPS[*]}"
  exit 1
fi

if [[ "$COPROCESSOR_COUNT_FLAG_DETECTED" == true ]]; then
  log_error "--coprocessors requires a value"
  exit 1
fi

if [[ "$COPROCESSOR_THRESHOLD_FLAG_DETECTED" == true ]]; then
  log_error "--coprocessor-threshold requires a value"
  exit 1
fi

# Check for conflicting flags
if [[ -n "$RESUME_STEP" && -n "$ONLY_STEP" ]]; then
  log_error "Cannot use --resume and --only together"
  exit 1
fi

if [[ -n "$COPROCESSOR_THRESHOLD_OVERRIDE" ]] && [[ "$COPROCESSOR_THRESHOLD_OVERRIDE" -gt "$COPROCESSOR_COUNT" ]]; then
  log_error "Invalid coprocessor threshold: $COPROCESSOR_THRESHOLD_OVERRIDE (must be <= --coprocessors $COPROCESSOR_COUNT)"
  exit 1
fi

if [[ "$COPROCESSOR_COUNT" -gt 5 ]]; then
  log_error "This local multicoprocessor mode currently supports up to 5 coprocessors"
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
log_contains() {
    local pattern=$1
    if command -v rg >/dev/null 2>&1; then
        rg -q "$pattern"
    else
        grep -q "$pattern"
    fi
}

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

        # Some one-shot jobs may complete their work but keep a process alive.
        # For host-sc-deploy, treat the deployment completion log as success and stop it.
        if [[ "$expect_running" == "false" && "$service_name" == "host-sc-deploy" && "$status" == "running" ]]; then
            if docker logs "$container_id" 2>&1 | log_contains "Contract deployment done!"; then
                log_warn "$service_name reported completion marker while still running; stopping container to unblock flow"
                docker stop "$container_id" >/dev/null 2>&1 || true
                status=$(docker inspect --format "{{.State.Status}}" "$container_id")
                exit_code=$(docker inspect --format "{{.State.ExitCode}}" "$container_id")
            fi
        fi

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

wait_for_relayer_ready() {
    local max_retries=24
    local retry_interval=5
    local container_name="${PROJECT}-relayer"

    log_info "Waiting for $container_name readiness signal..."

    for ((i=1; i<=max_retries; i++)); do
        if docker logs --since=10m "$container_name" 2>&1 | log_contains "All servers are ready and responding"; then
            log_info "$container_name reported ready"
            return 0
        fi

        if [ "$i" -lt "$max_retries" ]; then
            log_warn "$container_name not ready yet, waiting ${retry_interval}s... (${i}/${max_retries})"
            sleep "$retry_interval"
        else
            log_error "$container_name did not report ready within expected time"
            docker logs --tail 200 "$container_name" || true
            return 1
        fi
    done
}
# Function to prepare the local environment file for a component
prepare_local_env_file() {
    local component=$1
    local base_env_file="$SCRIPT_DIR/../env/staging/.env.$component"
    local local_env_file="$SCRIPT_DIR/../env/staging/.env.$component.local"
    local preserved_otlp_endpoint=""

    # Keep locally overridden OTLP endpoint across deploy regenerations.
    if [[ "$component" == "coprocessor" && -f "$local_env_file" ]]; then
        preserved_otlp_endpoint=$(awk -F= '$1 == "OTEL_EXPORTER_OTLP_ENDPOINT" {print substr($0, index($0, "=") + 1); exit}' "$local_env_file")
    fi

    if [[ ! -f "$base_env_file" ]]; then
        echo -e "${RED}[ERROR]${NC} Base environment file for $component not found: $base_env_file" >&2
        return 1
    else
        echo -e "${GREEN}[INFO]${NC} Creating/updating local environment file for $component..." >&2
        cp "$base_env_file" "$local_env_file"
    fi

    if [[ "$component" == "coprocessor" ]]; then
        local otlp_endpoint="${preserved_otlp_endpoint:-http://jaeger:4317}"
        if grep -q '^OTEL_EXPORTER_OTLP_ENDPOINT=' "$local_env_file"; then
            sed -i.bak "s|^OTEL_EXPORTER_OTLP_ENDPOINT=.*|OTEL_EXPORTER_OTLP_ENDPOINT=${otlp_endpoint}|" "$local_env_file"
        else
            printf '\nOTEL_EXPORTER_OTLP_ENDPOINT=%s\n' "$otlp_endpoint" >> "$local_env_file"
        fi
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

    if [[ "$MULTI_CHAIN" == "true" ]]; then
        components+=("host-node-b" "host-sc-b" "coprocessor-b")
    fi

    for component in "${components[@]}"; do
        prepare_local_env_file "$component" > /dev/null
    done

    log_info "All local environment files prepared successfully"
}

get_env_value() {
    local file=$1
    local key=$2
    awk -F= -v k="$key" '$1 == k {print substr($0, index($0, "=") + 1); exit}' "$file"
}

set_env_value() {
    local file=$1
    local key=$2
    local value=$3
    local escaped_value
    escaped_value=$(printf '%s' "$value" | sed 's/[\/&]/\\&/g')
    if grep -q "^${key}=" "$file"; then
        sed -i.bak "s|^${key}=.*|${key}=${escaped_value}|" "$file"
    else
        printf '%s=%s\n' "$key" "$value" >> "$file"
    fi
}

# Build effective n/t topology config and per-instance coprocessor env files.
configure_multicoprocessor_envs() {
    local gateway_env="$SCRIPT_DIR/../env/staging/.env.gateway-sc.local"
    local host_env="$SCRIPT_DIR/../env/staging/.env.host-sc.local"
    local coprocessor_env="$SCRIPT_DIR/../env/staging/.env.coprocessor.local"

    local configured_threshold
    configured_threshold=$(get_env_value "$gateway_env" "COPROCESSOR_THRESHOLD")
    if [[ -z "$configured_threshold" ]]; then
        configured_threshold=1
    fi
    if [[ -n "$COPROCESSOR_THRESHOLD_OVERRIDE" ]]; then
        configured_threshold="$COPROCESSOR_THRESHOLD_OVERRIDE"
    fi

    if [[ "$configured_threshold" -gt "$COPROCESSOR_COUNT" ]]; then
        log_error "Configured coprocessor threshold ($configured_threshold) cannot exceed number of coprocessors ($COPROCESSOR_COUNT)"
        exit 1
    fi

    set_env_value "$gateway_env" "NUM_COPROCESSORS" "$COPROCESSOR_COUNT"
    set_env_value "$gateway_env" "COPROCESSOR_THRESHOLD" "$configured_threshold"
    set_env_value "$host_env" "NUM_COPROCESSORS" "$COPROCESSOR_COUNT"
    set_env_value "$host_env" "COPROCESSOR_THRESHOLD" "$configured_threshold"

    # Default 1/1 topology does not require deriving extra coprocessor keys.
    if [[ "$COPROCESSOR_COUNT" -eq 1 ]]; then
        return 0
    fi

    local gateway_mnemonic
    gateway_mnemonic=$(get_env_value "$gateway_env" "MNEMONIC")
    if [[ -z "$gateway_mnemonic" ]]; then
        log_error "Missing MNEMONIC in $gateway_env; cannot derive coprocessor accounts"
        exit 1
    fi

    if ! command -v cast >/dev/null 2>&1; then
        log_error "cast is required to derive coprocessor accounts from mnemonic"
        exit 1
    fi

    local -a account_indices=(5 8 9 10 11)
    if [[ "$COPROCESSOR_COUNT" -gt "${#account_indices[@]}" ]]; then
        log_error "Not enough predefined account indices for $COPROCESSOR_COUNT coprocessors"
        exit 1
    fi

    for ((idx=0; idx<COPROCESSOR_COUNT; idx++)); do
        local account_index="${account_indices[$idx]}"
        local cp_address
        local cp_private_key

        cp_address=$(cast wallet address --mnemonic "$gateway_mnemonic" --mnemonic-index "$account_index")
        cp_private_key=$(cast wallet private-key --mnemonic "$gateway_mnemonic" --mnemonic-index "$account_index")

        set_env_value "$gateway_env" "COPROCESSOR_TX_SENDER_ADDRESS_${idx}" "$cp_address"
        set_env_value "$gateway_env" "COPROCESSOR_SIGNER_ADDRESS_${idx}" "$cp_address"
        set_env_value "$gateway_env" "COPROCESSOR_S3_BUCKET_URL_${idx}" "http://minio:9000/ct128"
        set_env_value "$host_env" "COPROCESSOR_SIGNER_ADDRESS_${idx}" "$cp_address"

        if [[ "$idx" -eq 0 ]]; then
            set_env_value "$coprocessor_env" "TX_SENDER_PRIVATE_KEY" "$cp_private_key"
            continue
        fi

        local coprocessor_instance_env="$SCRIPT_DIR/../env/staging/.env.coprocessor.${idx}.local"
        cp "$coprocessor_env" "$coprocessor_instance_env"
        set_env_value "$coprocessor_instance_env" "DATABASE_URL" "postgresql://${POSTGRES_USER:-postgres}:${POSTGRES_PASSWORD:-postgres}@db:5432/coprocessor_${idx}"
        set_env_value "$coprocessor_instance_env" "TX_SENDER_PRIVATE_KEY" "$cp_private_key"
    done
}

# Configure multi-chain environment files when --multi-chain is enabled.
configure_multichain_envs() {
    if [[ "$MULTI_CHAIN" != "true" ]]; then return 0; fi

    log_info "Configuring multi-chain environment..."

    local gateway_env="$SCRIPT_DIR/../env/staging/.env.gateway-sc.local"
    host_sc_b_env="$SCRIPT_DIR/../env/staging/.env.host-sc-b.local"
    coprocessor_b_env="$SCRIPT_DIR/../env/staging/.env.coprocessor-b.local"
    local test_suite_env="$SCRIPT_DIR/../env/staging/.env.test-suite.local"
    local relayer_config="$SCRIPT_DIR/../config/relayer/local.yaml"

    # Gateway: register Chain B as second host chain
    set_env_value "$gateway_env" "NUM_HOST_CHAINS" "2"
    local executor_addr_0
    executor_addr_0=$(get_env_value "$gateway_env" "HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_0")
    local acl_addr_0
    acl_addr_0=$(get_env_value "$gateway_env" "HOST_CHAIN_ACL_ADDRESS_0")
    set_env_value "$gateway_env" "HOST_CHAIN_CHAIN_ID_1" "67890"
    set_env_value "$gateway_env" "HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_1" "$executor_addr_0"
    set_env_value "$gateway_env" "HOST_CHAIN_ACL_ADDRESS_1" "$acl_addr_0"
    set_env_value "$gateway_env" "HOST_CHAIN_NAME_1" ""
    set_env_value "$gateway_env" "HOST_CHAIN_WEBSITE_1" ""

    # Host-node-b compose parameterization
    host_node_b_env="$SCRIPT_DIR/../env/staging/.env.host-node-b.local"
    set_env_value "$host_node_b_env" "HOST_NODE_CONTAINER_NAME" "host-node-b"
    set_env_value "$host_node_b_env" "HOST_NODE_PORT" "8547"
    set_env_value "$host_node_b_env" "HOST_NODE_CHAIN_ID" "67890"

    # Host-sc-b
    set_env_value "$host_sc_b_env" "RPC_URL" "http://host-node-b:8547"
    set_env_value "$host_sc_b_env" "HOST_SC_DEPLOY_CONTAINER_NAME" "host-sc-b-deploy"
    set_env_value "$host_sc_b_env" "HOST_SC_PAUSERS_CONTAINER_NAME" "host-sc-b-add-pausers"

    # Coprocessor-b
    set_env_value "$coprocessor_b_env" "RPC_HTTP_URL" "http://host-node-b:8547"
    set_env_value "$coprocessor_b_env" "RPC_WS_URL" "ws://host-node-b:8547"
    set_env_value "$coprocessor_b_env" "CHAIN_ID" "67890"

    # Test suite
    set_env_value "$test_suite_env" "RPC_URL_CHAIN_B" "http://host-node-b:8547"
    set_env_value "$test_suite_env" "CHAIN_ID_HOST_B" "67890"

    # Relayer: append Chain B to host_chains YAML
    local acl_address
    acl_address=$(get_env_value "$host_sc_b_env" "ACL_CONTRACT_ADDRESS")
    if [[ -z "$acl_address" ]]; then
        acl_address="$acl_addr_0"
    fi
    if ! grep -q "chain_id: 67890" "$relayer_config" 2>/dev/null; then
        sed -i.bak "/acl_address:.*${acl_addr_0}/a\\
  - chain_id: 67890\\
    url: \"http://host-node-b:8547\"\\
    acl_address: \"${acl_address}\"" "$relayer_config"
    fi

    # KMS Connector: replace host_chains with both chains
    local kms_connector_env="$SCRIPT_DIR/../env/staging/.env.kms-connector.local"
    if ! grep -q "chain_id.*67890" "$kms_connector_env" 2>/dev/null; then
        sed -i.bak "/^KMS_CONNECTOR_HOST_CHAINS=/,/^\]'/d" "$kms_connector_env"
        cat >> "$kms_connector_env" << KMSHC
KMS_CONNECTOR_HOST_CHAINS='[
    {"url": "http://host-node:8545", "chain_id": 12345, "acl_address": "${acl_addr_0}"},
    {"url": "http://host-node-b:8547", "chain_id": 67890, "acl_address": "${acl_address}"}
]'
KMSHC
    fi

    log_info "Multi-chain environment configured (Chain B: chain_id=67890, port=8547)"
}


# Start one extra coprocessor instance (db-migration first, then runtime services).
run_additional_coprocessor_instance() {
    local instance_idx=$1
    local env_file="$SCRIPT_DIR/../env/staging/.env.coprocessor.${instance_idx}.local"
    local source_compose="$SCRIPT_DIR/../docker-compose/coprocessor-docker-compose.yml"
    local temp_compose
    temp_compose=$(mktemp "$SCRIPT_DIR/../docker-compose/coprocessor-${instance_idx}.generated.XXXXXX")

    sed -e "s#../env/staging/.env.coprocessor.local#../env/staging/.env.coprocessor.${instance_idx}.local#g" \
        -e "s/coprocessor-/coprocessor${instance_idx}-/g" \
        -e "s/--coprocessor${instance_idx}-fhe-threads/--coprocessor-fhe-threads/g" \
        "$source_compose" > "$temp_compose"

    local db_migration_service="coprocessor${instance_idx}-db-migration"
    local runtime_services=(
        "coprocessor${instance_idx}-host-listener"
        "coprocessor${instance_idx}-host-listener-poller"
        "coprocessor${instance_idx}-gw-listener"
        "coprocessor${instance_idx}-tfhe-worker"
        "coprocessor${instance_idx}-zkproof-worker"
        "coprocessor${instance_idx}-sns-worker"
        "coprocessor${instance_idx}-transaction-sender"
    )

    log_info "Starting additional coprocessor instance #$instance_idx (db migration phase)"
    if [[ "$FORCE_BUILD" == true ]]; then
        docker compose -p "${PROJECT}" --env-file "$env_file" -f "$temp_compose" up --build -d "$db_migration_service"
    else
        docker compose -p "${PROJECT}" --env-file "$env_file" -f "$temp_compose" up -d "$db_migration_service"
    fi

    wait_for_service "$temp_compose" "$db_migration_service" "false"

    log_info "Starting additional coprocessor instance #$instance_idx (runtime phase)"
    if [[ "$FORCE_BUILD" == true ]]; then
        docker compose -p "${PROJECT}" --env-file "$env_file" -f "$temp_compose" up --build --no-deps -d "${runtime_services[@]}"
    else
        docker compose -p "${PROJECT}" --env-file "$env_file" -f "$temp_compose" up --no-deps -d "${runtime_services[@]}"
    fi

    wait_for_service "$temp_compose" "coprocessor${instance_idx}-host-listener" "true"
    wait_for_service "$temp_compose" "coprocessor${instance_idx}-gw-listener" "true"
    wait_for_service "$temp_compose" "coprocessor${instance_idx}-tfhe-worker" "true"
    wait_for_service "$temp_compose" "coprocessor${instance_idx}-zkproof-worker" "true"
    wait_for_service "$temp_compose" "coprocessor${instance_idx}-sns-worker" "true"
    wait_for_service "$temp_compose" "coprocessor${instance_idx}-transaction-sender" "true"

    rm -f "$temp_compose"
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
    if [ -n "$minio_ip" ]; then
    echo "Found $minio_container_name container IP: $minio_ip"
    local coprocessor_files=("$SCRIPT_DIR/../env/staging/.env.coprocessor.local")
    for ((idx=1; idx<COPROCESSOR_COUNT; idx++)); do
        coprocessor_files+=("$SCRIPT_DIR/../env/staging/.env.coprocessor.${idx}.local")
    done
    for coprocessor_file in "${coprocessor_files[@]}"; do
        if [[ -f "$coprocessor_file" ]]; then
            sed -i.bak "s|AWS_ENDPOINT_URL=http://[^:]*:9000|AWS_ENDPOINT_URL=http://$minio_ip:9000|" \
                "$coprocessor_file"
        fi
    done
    echo "Updated AWS_ENDPOINT_URL to http://$minio_ip:9000 for ${#coprocessor_files[@]} coprocessor env file(s)"
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

# Selective cleanup: tear down services from a specific step onwards
# Preserves containers/volumes from earlier steps
cleanup_from_step() {
    local start_step=$1
    local start_index=$(get_step_index "$start_step")

    log_warn "Resume mode: cleaning up services from '$start_step' onwards..."

    # Collect steps to cleanup (from start_step to end)
    local steps_to_cleanup=()
    for ((i=start_index; i<${#DEPLOYMENT_STEPS[@]}; i++)); do
        local step="${DEPLOYMENT_STEPS[$i]}"
        local compose=$(get_compose_for_step "$step")
        if [[ -n "$compose" ]]; then
            steps_to_cleanup+=("$compose")
        fi
    done

    # Tear down in reverse order (test-suite first, then relayer, etc.)
    for ((i=${#steps_to_cleanup[@]}-1; i>=0; i--)); do
        local component="${steps_to_cleanup[$i]}"
        local env_file="$SCRIPT_DIR/../env/staging/.env.$component.local"
        local compose_file="$SCRIPT_DIR/../docker-compose/$component-docker-compose.yml"

        if [[ -f "$compose_file" ]]; then
            # Use base env file if local doesn't exist yet
            if [[ ! -f "$env_file" ]]; then
                env_file="$SCRIPT_DIR/../env/staging/.env.$component"
            fi
            if [[ -f "$env_file" ]]; then
                log_info "Stopping $component services..."
                docker compose -p "${PROJECT}" --env-file "$env_file" -f "$compose_file" down -v --remove-orphans 2>/dev/null || true
            fi
        fi
    done

    log_info "Cleanup complete. Services before '$start_step' preserved."
}

# Single step cleanup: tear down only the specified step's services
cleanup_single_step() {
    local step=$1
    local compose=$(get_compose_for_step "$step")

    if [[ -z "$compose" ]]; then
        log_info "Step '$step' has no compose file to clean up"
        return 0
    fi

    log_warn "Only mode: cleaning up '$step' services..."

    local env_file="$SCRIPT_DIR/../env/staging/.env.$compose.local"
    local compose_file="$SCRIPT_DIR/../docker-compose/$compose-docker-compose.yml"

    if [[ -f "$compose_file" ]]; then
        if [[ ! -f "$env_file" ]]; then
            env_file="$SCRIPT_DIR/../env/staging/.env.$compose"
        fi
        if [[ -f "$env_file" ]]; then
            log_info "Stopping $compose services..."
            docker compose -p "${PROJECT}" --env-file "$env_file" -f "$compose_file" down -v --remove-orphans 2>/dev/null || true
        fi
    fi

    log_info "Cleanup complete. Only '$step' was cleaned."
}

# Run cleanup based on mode
if [[ -n "$ONLY_STEP" ]]; then
    cleanup_single_step "$ONLY_STEP"
elif [[ -n "$RESUME_STEP" ]]; then
    cleanup_from_step "$RESUME_STEP"
else
    cleanup "$@"
fi

prepare_all_env_files
prepare_local_config_relayer
configure_multicoprocessor_envs
configure_multichain_envs

log_info "Deploying FHEVM Stack..."
log_info "Coprocessor topology: n=$COPROCESSOR_COUNT threshold=${COPROCESSOR_THRESHOLD_OVERRIDE:-auto}"
if [[ "$MULTI_CHAIN" == "true" ]]; then
    log_info "Multi-chain mode: Chain A (12345) + Chain B (67890)"
fi

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

# Step 5b: host-node-b (multi-chain only)
# Reuses host-node compose with sed for service key + env_file path;
# container name, port, and chain-id are parameterized and come from env vars.
if [[ "$MULTI_CHAIN" == "true" ]] && ! should_skip_step "host-node"; then
    log_info "Starting Host node B service (Chain B)..."
    temp_host_b=$(mktemp "$SCRIPT_DIR/../docker-compose/host-node-b.XXXXXX")
    sed -e 's/^  host-node:$/  host-node-b:/' \
        -e 's/\.env\.host-node\.local/.env.host-node-b.local/g' \
        "$SCRIPT_DIR/../docker-compose/host-node-docker-compose.yml" > "$temp_host_b"
    host_node_b_env="$SCRIPT_DIR/../env/staging/.env.host-node-b.local"
    if [[ "$FORCE_BUILD" == true ]]; then
        docker compose -p "${PROJECT}" --env-file "$host_node_b_env" -f "$temp_host_b" up --build -d
    else
        docker compose -p "${PROJECT}" --env-file "$host_node_b_env" -f "$temp_host_b" up -d
    fi
    wait_for_service "$temp_host_b" "host-node-b" "true"
    rm -f "$temp_host_b"
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
    for ((idx=1; idx<COPROCESSOR_COUNT; idx++)); do
        run_additional_coprocessor_instance "$idx"
    done
else
    log_info "Skipping step: coprocessor (resuming from $RESUME_STEP)"
fi

# Step 7b: coprocessor Chain B listeners (multi-chain only)
# Reuses coprocessor compose with sed to rename host-listener services + env_file path.
# Only starts the two host-listener services with --no-deps (db-migration already done).
if [[ "$MULTI_CHAIN" == "true" ]] && ! should_skip_step "coprocessor"; then
    # Register Chain B in the coprocessor database so workers (zkproof, etc.) can resolve its ACL address.
    # This must happen before any Chain B traffic arrives, and the zkproof-worker must be restarted
    # because HostChainsCache is loaded once at startup.
    log_info "Registering Chain B (67890) in coprocessor host_chains table..."
    chain_b_acl=$(get_env_value "$SCRIPT_DIR/../env/staging/.env.coprocessor-b.local" "ACL_CONTRACT_ADDRESS")
    docker exec coprocessor-and-kms-db psql -U postgres -d coprocessor -c \
        "INSERT INTO host_chains (chain_id, name, acl_contract_address) VALUES (67890, 'test chain b', '${chain_b_acl}') ON CONFLICT (chain_id) DO NOTHING;"

    log_info "Restarting zkproof-worker to pick up Chain B host_chains entry..."
    docker restart coprocessor-zkproof-worker
    wait_for_service "$SCRIPT_DIR/../docker-compose/coprocessor-docker-compose.yml" "coprocessor-zkproof-worker" "true"

    log_info "Starting coprocessor listeners for Chain B..."
    temp_coproc_b=$(mktemp "$SCRIPT_DIR/../docker-compose/coprocessor-host-b.XXXXXX")
    # Process poller pattern first (more specific) to avoid partial matches
    sed -e 's/^  coprocessor-host-listener-poller:$/  coprocessor-host-listener-poller-b:/' \
        -e 's/^  coprocessor-host-listener:$/  coprocessor-host-listener-b:/' \
        -e 's/container_name: coprocessor-host-listener-poller$/container_name: coprocessor-host-listener-poller-b/' \
        -e 's/container_name: coprocessor-host-listener$/container_name: coprocessor-host-listener-b/' \
        -e 's/\.env\.coprocessor\.local/.env.coprocessor-b.local/g' \
        "$SCRIPT_DIR/../docker-compose/coprocessor-docker-compose.yml" > "$temp_coproc_b"
    coprocessor_b_env="$SCRIPT_DIR/../env/staging/.env.coprocessor-b.local"
    if [[ "$FORCE_BUILD" == true ]]; then
        docker compose -p "${PROJECT}" --env-file "$coprocessor_b_env" -f "$temp_coproc_b" up --build --no-deps -d coprocessor-host-listener-b coprocessor-host-listener-poller-b
    else
        docker compose -p "${PROJECT}" --env-file "$coprocessor_b_env" -f "$temp_coproc_b" up --no-deps -d coprocessor-host-listener-b coprocessor-host-listener-poller-b
    fi
    wait_for_service "$temp_coproc_b" "coprocessor-host-listener-b" "true"
    wait_for_service "$temp_coproc_b" "coprocessor-host-listener-poller-b" "true"
    rm -f "$temp_coproc_b"
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

# Step 11b: host-sc-b (multi-chain only)
# Reuses host-sc compose with sed for service keys, depends_on, env_file path, and volume name;
# container names are parameterized and come from env vars.
if [[ "$MULTI_CHAIN" == "true" ]] && ! should_skip_step "host-sc"; then
    log_info "Starting Host contracts Chain B..."
    temp_sc_b=$(mktemp "$SCRIPT_DIR/../docker-compose/host-sc-b.XXXXXX")
    sed -e 's/host-sc-deploy/host-sc-b-deploy/g' \
        -e 's/host-sc-add-pausers/host-sc-b-add-pausers/g' \
        -e 's/\.env\.host-sc\.local/.env.host-sc-b.local/g' \
        -e 's/addresses-volume/addresses-volume-b/g' \
        "$SCRIPT_DIR/../docker-compose/host-sc-docker-compose.yml" > "$temp_sc_b"
    host_sc_b_env="$SCRIPT_DIR/../env/staging/.env.host-sc-b.local"
    if [[ "$FORCE_BUILD" == true ]]; then
        docker compose -p "${PROJECT}" --env-file "$host_sc_b_env" -f "$temp_sc_b" up --build -d
    else
        docker compose -p "${PROJECT}" --env-file "$host_sc_b_env" -f "$temp_sc_b" up -d
    fi
    wait_for_service "$temp_sc_b" "host-sc-b-deploy" "false"
    wait_for_service "$temp_sc_b" "host-sc-b-add-pausers" "false"
    rm -f "$temp_sc_b"
fi

# Step 12: relayer
if ! should_skip_step "relayer"; then
    ${RUN_COMPOSE} "relayer" "Relayer Services" \
        "${PROJECT}-relayer:running"
    wait_for_relayer_ready
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
