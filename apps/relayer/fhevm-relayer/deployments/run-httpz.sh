#!/bin/bash

# Exit on error
set -e

# Colors for output
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

# Function to check and install Git LFS files
setup_git_lfs() {
    log_info "Setting up Git LFS files..."

    # Check if Git LFS is installed
    if ! command -v git-lfs &> /dev/null; then
        log_error "Git LFS is not installed. Please install it first:"
        log_error "  brew install git-lfs"
        exit 1
    fi

    # Make sure we're in the repository root directory
    local repo_root=$(git rev-parse --show-toplevel)
    cd "$repo_root"

    # Initialize Git LFS
    log_info "Initializing Git LFS..."
    git lfs install

    # Fetch all LFS files
    log_info "Fetching LFS files..."
    if ! git lfs pull; then
        log_error "Failed to fetch LFS files"
        exit 1
    fi

    # Verify LFS files are present
    log_info "Checking LFS files existence..."
    local lfs_files=(
        "deployments/config/kms-keys/CRS/a5fedad3fd734a598fb67452099229445cb68447198fb56f29bb64d98953d002"
        "deployments/config/kms-keys/PublicKey/408d8cbaa51dece7f782fe04ba0b1c1d017b10880c538b7c72037468fe5c97ee"
        "deployments/config/kms-keys/ServerKey/408d8cbaa51dece7f782fe04ba0b1c1d017b10880c538b7c72037468fe5c97ee"
        "deployments/config/kms-keys/SnsKey/408d8cbaa51dece7f782fe04ba0b1c1d017b10880c538b7c72037468fe5c97ee"
    )

    local missing_files=false
    for file in "${lfs_files[@]}"; do
        if [ ! -f "$file" ]; then
            log_error "LFS file not found: $file"
            missing_files=true
        fi
    done

    if [ "$missing_files" = true ]; then
        log_error "Some LFS files are missing. Please check Git LFS setup."
        exit 1
    fi

    log_info "Git LFS files successfully installed."

    # Return to deployments directory
    cd "$(dirname "$0")"
}

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

# Function to start an entire docker-compose file and wait for specified services
run_compose() {
    local env_file=$1
    local compose_file=$2
    local service_desc=$3
    shift 3

    local services=("$@")
    local service_states=()
    local service_names=()

    # Parse service names and states
    for arg in "${services[@]}"; do
        IFS=':' read -r name state <<< "$arg"
        service_names+=("$name")
        service_states+=("$state")
    done

    log_info "Starting $service_desc..."

    # Start all services
    if ! docker compose --env-file "$env_file" -p zama -f "$compose_file" up -d; then
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

get_s3_mock_ip() {
    # Get IP address of s3-mock container and update AWS_ENDPOINT_URL
    # this is a workaround as sns-worker is not able to resolve the container name
    local s3_mock_container_name=$1
    local s3_mock_ip
    s3_mock_ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$s3_mock_container_name")
    local coprocessor_file="./config/env/.env.staging.coprocessor"
    if [ -n "$s3_mock_ip" ]; then
    echo "Found $s3_mock_container_name container IP: $s3_mock_ip"
    # Replace the AWS_ENDPOINT_URL with the container's IP address
    sed -i.bak "s|export AWS_ENDPOINT_URL=\"http://[^:]*:9000\"|export AWS_ENDPOINT_URL=\"http://$s3_mock_ip:9000\"|" \
            "$coprocessor_file"
    echo "Updated AWS_ENDPOINT_URL to http://$s3_mock_ip:9000"
    else
    echo "Error: Could not find IP address for $s3_mock_container_name container"
    exit 1
    fi
}

cleanup() {
    log_warn "Setup new environment, cleaning up..."
    docker compose -p zama down -v --remove-orphans

    # Only exit if requested
    if [ "$1" = "exit" ]; then
        exit 1
    fi
}

cleanup "$@"

cd "$(git rev-parse --show-toplevel)/deployments"

run_compose "./config/env/.env.staging.layer1" "minio-docker-compose.yml" "S3 Mock Services" \
    "s3-mock:running" \
    "s3-mock-setup:complete"

run_compose "./config/env/.env.staging.core" "core-docker-compose.yml" "Core Services" \
    "kms-core:running" \
    "generate-fhe-keys:complete" \
    "update-kms-keys:complete"

run_compose "./config/env/.env.staging.connector" "connector-docker-compose.yml" "Connector Services" \
    "kms-connector:running"

run_compose "./config/env/.env.staging.layer2" "layer2-docker-compose.yml" "Layer 2 Services" \
    "layer2-node:running" \
    "layer2-sc-deploy:complete"

run_compose "./config/env/.env.staging.layer1" "layer1-docker-compose.yml" "Layer 1 Services" \
    "layer1-node:running" \
    "layer1-sc-deploy:complete"

get_s3_mock_ip "s3-mock"

run_compose "./config/env/.env.staging.coprocessor" "coprocessor-docker-compose.yml" "Coprocessor Services" \
    "db:running" \
    "db-migration:complete" \
    "httpz-listener:running" \
    "gw-listener:running" \
    "tfhe-worker:running" \
    "zkproof-worker:running" \
    "sns-worker:running" \
    "transaction-sender:running"

run_compose "./config/env/.env.staging.relayer" "relayer-docker-compose.yml" "Relayer Services" \
    "httpz-relayer:running" \
    "e2e-test-debug:running"

log_info "All services started successfully!"