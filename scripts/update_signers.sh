#!/bin/bash

# Script to update .env file with Ethereum addresses from signer files
# Usage: ./update_env.sh [ENV_FILE_PATH] [SIGNER_FOLDER_PATH]

# Exit immediately on errors, treat unset variables as errors, and enable debugging of pipe commands
set -Eeuo pipefail

# Default values
DEFAULT_ENV_FILE="work_dir/fhevm/.env.example.deployment"
DEFAULT_SIGNER_FOLDER="network-fhe-keys"

# Accept optional arguments with default fallbacks
ENV_FILE="${1:-$DEFAULT_ENV_FILE}"
SIGNER_FOLDER="${2:-$DEFAULT_SIGNER_FOLDER}"

# Array of signer files
signer_files=("signer1" "signer2" "signer3" "signer4")

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

# Create a backup of the original .env file
cp "$ENV_FILE" "${ENV_FILE}.bak"
echo "Backup of .env file created at ${ENV_FILE}.bak"

# Update the number of signers in the .env file
sed -i '' 's/^export NUM_KMS_SIGNERS=.*/export NUM_KMS_SIGNERS="4"/' "$ENV_FILE"

# Loop through each signer file and update the corresponding address in the .env file
for i in "${!signer_files[@]}"; do
    signer_file="$SIGNER_FOLDER/${signer_files[$i]}"
    
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
    sed -i '' "s/^export ADDRESS_KMS_SIGNER_$i=.*/export ADDRESS_KMS_SIGNER_$i=\"$signer_address\"/" "$ENV_FILE"
    echo "Updated ADDRESS_KMS_SIGNER_$i in $ENV_FILE"
done

echo "Successfully updated $ENV_FILE with new signer addresses."
