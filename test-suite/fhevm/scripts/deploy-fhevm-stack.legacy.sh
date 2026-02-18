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

DEPLOY_MANIFEST_PATH="${SCRIPT_DIR}/legacy/lib/deploy-manifest.sh"
VERSION_MANIFEST_PATH="${SCRIPT_DIR}/legacy/lib/version-manifest.sh"

if [[ ! -f "$DEPLOY_MANIFEST_PATH" ]]; then
    log_error "Deploy manifest not found: $DEPLOY_MANIFEST_PATH"
    exit 1
fi
if [[ ! -f "$VERSION_MANIFEST_PATH" ]]; then
    log_error "Version manifest not found: $VERSION_MANIFEST_PATH"
    exit 1
fi

source "$DEPLOY_MANIFEST_PATH"
source "$VERSION_MANIFEST_PATH"
fhevm_export_default_versions

DEPLOYMENT_STEPS=()
while IFS= read -r step; do
    DEPLOYMENT_STEPS+=("$step")
done < <(fhevm_manifest_step_names)

if [[ ${#DEPLOYMENT_STEPS[@]} -eq 0 ]]; then
    log_error "Deployment manifest is empty"
    exit 1
fi

VALID_STEPS="$(fhevm_manifest_steps_string)"

# Get docker-compose component name for a step
# kms-signer has no compose file (it's just a script), returns empty
get_compose_for_step() {
    local step=$1
    fhevm_manifest_step_field "$step" component || true
}

# Helper to get index of step in DEPLOYMENT_STEPS array
get_step_index() {
    local step_name=$1
    fhevm_manifest_step_index "$step_name"
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
        local resume_index
        local current_index

        resume_index=$(get_step_index "$RESUME_STEP")
        current_index=$(get_step_index "$current_step")
        [[ "$current_index" -lt "$resume_index" ]]
        return
    fi

    # Normal mode: don't skip anything
    return 1
}

# Argument Parsing
FORCE_BUILD=false
LOCAL_BUILD=false
RESUME_STEP=""
ONLY_STEP=""
RESUME_FLAG_DETECTED=false
ONLY_FLAG_DETECTED=false
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
  elif [[ "$RESUME_FLAG_DETECTED" == true ]]; then
    RESUME_STEP="$arg"
    RESUME_FLAG_DETECTED=false
    # Validate step name
    if [[ $(get_step_index "$RESUME_STEP") -eq -1 ]]; then
      log_error "Invalid resume step: $RESUME_STEP"
      log_error "Valid steps are: ${VALID_STEPS}"
      exit 1
    fi
    log_info "Resume mode: starting from step '$RESUME_STEP'"
  elif [[ "$ONLY_FLAG_DETECTED" == true ]]; then
    ONLY_STEP="$arg"
    ONLY_FLAG_DETECTED=false
    # Validate step name
    if [[ $(get_step_index "$ONLY_STEP") -eq -1 ]]; then
      log_error "Invalid step: $ONLY_STEP"
      log_error "Valid steps are: ${VALID_STEPS}"
      exit 1
    fi
    log_info "Only mode: deploying only step '$ONLY_STEP'"
  else
    NEW_ARGS+=("$arg")
  fi
done

# Check for incomplete flags
if [[ "$RESUME_FLAG_DETECTED" == true ]]; then
  log_error "--resume requires a step name"
  log_error "Valid steps are: ${VALID_STEPS}"
  exit 1
fi

if [[ "$ONLY_FLAG_DETECTED" == true ]]; then
  log_error "--only requires a step name"
  log_error "Valid steps are: ${VALID_STEPS}"
  exit 1
fi

# Check for conflicting flags
if [[ -n "$RESUME_STEP" && -n "$ONLY_STEP" ]]; then
  log_error "Cannot use --resume and --only together"
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

logs_indicate_key_bootstrap_not_ready() {
    local logs=$1

    echo "$logs" | grep -Eiq "CrsNotGenerated|CrsgenNotRequested|KeygenNotRequested|PrepKeygenNotRequested|key bootstrap[^\n]*not ready|bootstrap key[^\n]*not ready|materials are not ready"
}

print_service_failure_hints() {
    local service_name=$1
    local step_hint=$2
    local exit_code=$3
    local oom_killed=$4
    local logs=$5

    if [[ "$exit_code" == "137" || "$oom_killed" == "true" ]]; then
        log_error "$service_name looks OOM-killed (exit code: ${exit_code}, OOMKilled: ${oom_killed})."
        log_error "Action: increase Docker memory and retry from this step: ./fhevm-cli deploy --resume ${step_hint}"
    fi

    if logs_indicate_key_bootstrap_not_ready "$logs"; then
        log_error "Detected key-bootstrap-not-ready state while starting $service_name."
        log_error "Action: wait for gateway keygen/CRS generation to settle, then retry: ./fhevm-cli deploy --resume gateway-sc"
    fi
}

print_container_logs() {
    local container_id=$1

    docker logs "$container_id" 2>&1 || true
}

# Function to check if services are ready based on expected state
wait_for_service() {
    local service_name=$1
    local step_hint=$2
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
        local container_id
        container_id=$(docker ps -a --filter name="${service_name}$" --format "{{.ID}}")

        if [[ -z "$container_id" ]]; then
            log_warn "Container for $service_name not found, waiting..."
            sleep "$retry_interval"
            continue
        fi

        local status
        local exit_code
        local oom_killed
        local service_logs

        status=$(docker inspect --format "{{.State.Status}}" "$container_id")
        exit_code=$(docker inspect --format "{{.State.ExitCode}}" "$container_id")
        oom_killed=$(docker inspect --format "{{.State.OOMKilled}}" "$container_id" 2>/dev/null || echo "false")

        # Check if service meets the expected state
        if [[ "$expect_running" == "true" && "$status" == "running" ]]; then
            log_info "$service_name is now running"
            return 0
        elif [[ "$expect_running" == "false" && "$status" == "exited" && "$exit_code" == "0" ]]; then
            log_info "$service_name completed successfully"
            return 0
        elif [[ "$status" == "exited" && "$exit_code" != "0" ]]; then
            log_error "$service_name failed with exit code $exit_code"
            service_logs=$(print_container_logs "$container_id")
            print_service_failure_hints "$service_name" "$step_hint" "$exit_code" "$oom_killed" "$service_logs"
            if [[ -n "$service_logs" ]]; then
                echo "$service_logs"
            fi
            return 1
        fi

        # Still waiting
        if [ "$i" -lt "$max_retries" ]; then
            log_warn "$service_name not ready yet (status: $status), waiting ${retry_interval}s... (${i}/${max_retries})"
            sleep "$retry_interval"
        else
            log_error "$service_name failed to reach desired state within the expected time"
            service_logs=$(print_container_logs "$container_id")
            print_service_failure_hints "$service_name" "$step_hint" "$exit_code" "$oom_killed" "$service_logs"
            if [[ -n "$service_logs" ]]; then
                echo "$service_logs"
            fi
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

prepare_all_env_files() {
    log_info "Preparing all local environment files..."

    local component
    while IFS= read -r component; do
        prepare_local_env_file "$component" > /dev/null
    done < <(fhevm_manifest_step_components)

    log_info "All local environment files prepared successfully"
}

# Function to start an entire docker-compose file and wait for specified services
run_compose() {
    local component=$1
    local service_desc=$2
    local use_build=$3
    local env_file="$SCRIPT_DIR/../env/staging/.env.$component.local"
    local compose_file="$SCRIPT_DIR/../docker-compose/$component-docker-compose.yml"
    shift 3

    local services=("$@")
    local service_states=()
    local service_names=()

    # Parse service names and states
    local arg
    for arg in "${services[@]}"; do
        IFS=':' read -r name state <<< "$arg"
        service_names+=("$name")
        service_states+=("$state")
    done

    if [[ "$use_build" == "true" ]]; then
        log_info "Building and starting $service_desc using local environment file..."
    else
        log_info "Starting $service_desc using local environment file..."
    fi
    log_info "Using environment file: $env_file"

    # Start all services
    if [[ "$use_build" == "true" ]]; then
        if ! docker compose -p "${PROJECT}" --env-file "$env_file" -f "$compose_file" up --build -d; then
            log_error "Failed to build and start $service_desc"
            return 1
        fi
    else
        if ! docker compose -p "${PROJECT}" --env-file "$env_file" -f "$compose_file" up -d; then
            log_error "Failed to start $service_desc"
            return 1
        fi
    fi

    # Wait for each specified service
    local i
    for i in "${!service_names[@]}"; do
        local name="${service_names[$i]}"
        local expect_running=true

        if [[ "${service_states[$i]}" == "complete" ]]; then
            expect_running=false
        fi

        wait_for_service "$name" "$component" "$expect_running"
        if [ $? -ne 0 ]; then
            return 1
        fi
    done
}

run_manifest_step() {
    local step=$1
    local component
    local description
    local services_raw
    local buildable
    local use_build=false

    component=$(fhevm_manifest_step_field "$step" component)
    description=$(fhevm_manifest_step_field "$step" description)
    services_raw=$(fhevm_manifest_step_field "$step" services)
    buildable=$(fhevm_manifest_step_field "$step" buildable)

    if [[ "$FORCE_BUILD" == true && "$buildable" == "true" ]]; then
        use_build=true
    fi

    local services=()
    if [[ -n "$services_raw" ]]; then
        # shellcheck disable=SC2206
        services=($services_raw)
    fi

    run_compose "$component" "$description" "$use_build" "${services[@]}"
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

resolve_component_env_file() {
    local component=$1
    local local_env_file="$SCRIPT_DIR/../env/staging/.env.$component.local"
    local base_env_file="$SCRIPT_DIR/../env/staging/.env.$component"

    if [[ -f "$local_env_file" ]]; then
        printf '%s\n' "$local_env_file"
        return 0
    fi

    if [[ -f "$base_env_file" ]]; then
        printf '%s\n' "$base_env_file"
        return 0
    fi

    return 1
}

cleanup_component() {
    local component=$1
    local compose_file="$SCRIPT_DIR/../docker-compose/$component-docker-compose.yml"

    if [[ ! -f "$compose_file" ]]; then
        return 0
    fi

    local env_file=""
    if env_file=$(resolve_component_env_file "$component"); then
        log_info "Stopping $component services..."
        docker compose -p "${PROJECT}" --env-file "$env_file" -f "$compose_file" down -v --remove-orphans 2>/dev/null || true
    else
        # Fallback for stale/orphaned containers when env files are unavailable.
        log_warn "Env file missing for $component, attempting cleanup without explicit env file"
        docker compose -p "${PROJECT}" -f "$compose_file" down -v --remove-orphans 2>/dev/null || true
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
    local start_index
    start_index=$(get_step_index "$start_step")

    log_warn "Resume mode: cleaning up services from '$start_step' onwards..."

    # Collect steps to cleanup (from start_step to end)
    local steps_to_cleanup=()
    local seen_components=" "
    local i

    for ((i=start_index; i<${#DEPLOYMENT_STEPS[@]}; i++)); do
        local step="${DEPLOYMENT_STEPS[$i]}"
        local component
        component=$(get_compose_for_step "$step")

        if [[ -n "$component" && "$seen_components" != *" $component "* ]]; then
            steps_to_cleanup+=("$component")
            seen_components+="$component "
        fi
    done

    # Tear down in reverse order (test-suite first, then relayer, etc.)
    for ((i=${#steps_to_cleanup[@]}-1; i>=0; i--)); do
        cleanup_component "${steps_to_cleanup[$i]}"
    done

    log_info "Cleanup complete. Services before '$start_step' preserved."
}

# Single step cleanup: tear down only the specified step's services
cleanup_single_step() {
    local step=$1
    local component
    component=$(get_compose_for_step "$step")

    if [[ -z "$component" ]]; then
        log_info "Step '$step' has no compose file to clean up"
        return 0
    fi

    log_warn "Only mode: cleaning up '$step' services..."
    cleanup_component "$component"
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

log_info "Deploying FHEVM Stack..."

BUILD_TAG=""
if [ "$FORCE_BUILD" = true ]; then
  BUILD_TAG=" (local build)"
fi

fhevm_print_versions log_info "$BUILD_TAG"

# Setup Gateway contracts triggers KMS materials generation.
# Key generation can still take a few seconds, which is why this step is
# sequenced before host contracts/relayer/tests.
for step in "${DEPLOYMENT_STEPS[@]}"; do
    if should_skip_step "$step"; then
        if [[ -n "$ONLY_STEP" ]]; then
            log_info "Skipping step: $step (only mode: $ONLY_STEP)"
        else
            log_info "Skipping step: $step (resuming from $RESUME_STEP)"
        fi

        # Still need minio IP for coprocessor env if container is running
        if [[ "$step" == "minio" ]]; then
            if docker ps --filter name=fhevm-minio --format "{{.Names}}" | grep -q fhevm-minio; then
                get_minio_ip "fhevm-minio"
            fi
        fi
        continue
    fi

    case "$step" in
        minio)
            run_manifest_step "$step"
            get_minio_ip "fhevm-minio"
            ;;
        kms-signer)
            sleep 5
            "${SCRIPT_DIR}/setup-kms-signer-address.sh"
            ;;
        *)
            run_manifest_step "$step"
            ;;
    esac
done

log_info "All services started successfully!"
