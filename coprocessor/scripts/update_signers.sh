#!/bin/bash

# Script to update .env file with Ethereum addresses from signer files
# Usage: ./update_env.sh [ENV_FILE_PATH] [SIGNER_FOLDER_PATH] [NUM_SIGNERS]

# Exit immediately on errors, treat unset variables as errors, and enable debugging of pipe commands
set -Eeuo pipefail

# Default values
DEFAULT_ENV_FILE="work_dir/fhevm/.env.example.deployment"
DEFAULT_SIGNER_FOLDER="network-fhe-keys"
DEFAULT_NUM_SIGNERS=4

# Accept optional arguments with default fallbacks
ENV_FILE="${1:-$DEFAULT_ENV_FILE}"
SIGNER_FOLDER="${2:-$DEFAULT_SIGNER_FOLDER}"
NUM_SIGNERS="${3:-$DEFAULT_NUM_SIGNERS}"

# Detect the operating system for compatibility with sed
if [[ "$OSTYPE" == "darwin"* ]]; then
    SED_INPLACE="sed -i ''"  # macOS
else
    SED_INPLACE="sed -i"     # Linux
fi

# Ensure the ENV file exists
if [ ! -f "$ENV_FILE" ]; then
    echo "Error: .env file '$ENV_FILE' not found."
    exit 1
fi

# Ensure the signer folder exists
if [ ! -d "$SIGNER_FOLDER" ]; then
    echo "Error: Signer folder '$SIGNER_FOLDER' not found."
    exit 1
fi

# Ensure NUM_SIGNERS is a positive integer
if ! [[ "$NUM_SIGNERS" =~ ^[0-9]+$ ]] || [ "$NUM_SIGNERS" -le 0 ]; then
    echo "Error: NUM_SIGNERS must be a positive integer."
    exit 1
fi

# Create a backup of the original .env file
cp "$ENV_FILE" "${ENV_FILE}.bak"
echo "Backup of .env file created at ${ENV_FILE}.bak"

# Update the number of signers in the .env file
$SED_INPLACE "s/^export NUM_KMS_SIGNERS=.*/export NUM_KMS_SIGNERS=\"$NUM_SIGNERS\"/" "$ENV_FILE"

# Loop through each signer file up to the specified number and update the corresponding address in the .env file
for (( i=1; i<=NUM_SIGNERS; i++ )); do
    signer_file="$SIGNER_FOLDER/signer$i"
    
    # Check if signer file exists
    if [ ! -f "$signer_file" ]; then
        echo "Error: Signer file '$signer_file' not found."
        exit 1
    fi
    
    # Read the signer address from the file
    signer_address=$(<"$signer_file")
    
    # Validate that the signer address is in Ethereum address format (basic validation)
    if [[ ! "$signer_address" =~ ^0x[a-fA-F0-9]{40}$ ]]; then
        echo "Error: Invalid Ethereum address format in '$signer_file'."
        exit 1
    fi

    # Update the corresponding address in the .env file
    env_index=$((i - 1))
    $SED_INPLACE "s/^export ADDRESS_KMS_SIGNER_$env_index=.*/export ADDRESS_KMS_SIGNER_$env_index=\"$signer_address\"/" "$ENV_FILE"
        echo "Updated ADDRESS_KMS_SIGNER_$env_index in $ENV_FILE"
done

echo "Successfully updated $ENV_FILE with $NUM_SIGNERS signer addresses."
