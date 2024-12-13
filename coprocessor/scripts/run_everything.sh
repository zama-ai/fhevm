#!/usr/bin/env bash

set -euo pipefail  # Strict error handling: exit on error, unset variables, or pipe failures

# Trap errors to print a custom message with the failing command
trap 'echo "Error: Command \"$BASH_COMMAND\" failed. Exiting."' ERR

# Function for logging
log() {
    echo "##################################"
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $*"
    echo "##################################"
}

# Run KMS service
log "Starting KMS service..."
make run-kms
log "KMS service started successfully."
sleep 4  # Allow some time for the service to initialize

# Initialize the database
log "Initializing the database..."
make init-db
log "Database initialization completed."

# Deploy ACL, Gateway, etc.
log "Preparing for e2e tests. This may take some time..."
make prepare-e2e-test

# Check deployment logs
log "Deployment in progress. You can check the logs of zama-setup-fhevm-contracts if needed."
log "To monitor: docker logs zama-setup-fhevm-contracts"

timeout=140  # Timeout after 60 seconds
while ! docker logs zama-setup-fhevm-contracts 2>&1 | grep -q "Deployment script completed successfully"; do
    sleep 10
    log "Waiting for fhevm contract deployment..."
    log "To monitor: docker logs zama-setup-fhevm-contracts -f"
    timeout=$((timeout - 10))
    if [ "$timeout" -le 0 ]; then
        log "Timeout reached waiting for zama-setup-fhevm-contracts to finish. Exiting."
        exit 1
    fi
done
log "Deployment script completed successfully."

# Run an asynchronous test
log "Running a decryption test trivial + non trivial..."
make run-async-test
log "Test completed successfully."

# Final message
log "All tasks completed successfully!"
