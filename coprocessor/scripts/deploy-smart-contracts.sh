#!/bin/bash

# Exit the script immediately if any command fails
set -e

# Enable debug mode for troubleshooting (uncomment if needed)
# set -x

# Define functions for clarity and reuse
function check_and_run() {
    local cmd="$1"
    echo "Running: $cmd"
    eval "$cmd"
}

# Provide a log message for clarity
echo "Starting deployment script..."

if [[ -f .env.example.deployment ]]; then
    check_and_run "cp .env.example.deployment .env"
    echo "Environment file copied successfully."
else
    echo "Error: .env.example.deployment not found!" >&2
    exit 1
fi


if command -v npm &>/dev/null; then
    check_and_run "npm install"
else
    echo "Error: npm is not installed. Please install Node.js and npm first." >&2
    exit 1
fi

if [[ -f ./precompute-addresses.sh ]]; then
    check_and_run "./precompute-addresses.sh"
else
    echo "Error: precompute-addresses.sh not found!" >&2
    exit 1
fi

if [[ -f ./launch-fhevm-coprocessor.sh ]]; then
    check_and_run "./launch-fhevm-coprocessor.sh"
else
    echo "Error: launch-fhevm-coprocessor.sh not found!" >&2
    exit 1
fi

# Final success message
echo "Deployment script completed successfully!"
