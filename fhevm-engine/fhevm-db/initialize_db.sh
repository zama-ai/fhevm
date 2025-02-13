#!/bin/bash
set -e  # Exit on error

# 1: Create Database
echo "Creating database..."
sqlx database create || { echo "Failed to create database."; exit 1; }

# 2: Run sqlx migrations
echo "Running migrations..."
sqlx migrate run --source /migrations || { echo "Failed to run migrations."; exit 1; }

# 3. Insert test tenant with keys
echo "Start preparing tenant query..."

# Settings
CHAIN_ID=${CHAIN_ID:-"12345"}
PKS_FILE=${PKS_FILE:-"/fhevm-keys/pks"}
SKS_FILE=${SKS_FILE:-"/fhevm-keys/sks"}
PUBLIC_PARAMS_FILE=${PUBLIC_PARAMS_FILE:-"/fhevm-keys/pp"}

# Verify key files
for file in "$PKS_FILE" "$SKS_FILE" "$PUBLIC_PARAMS_FILE"; do
    if [[ ! -f $file ]]; then
        echo "Error: Key file $file not found."; exit 1;
    fi
done

# Ensure environment variables are set
if [[ -z "$DATABASE_URL" || -z "$TENANT_API_KEY" || -z "$ACL_CONTRACT_ADDRESS" || -z "$INPUT_VERIFIER_ADDRESS" ]]; then
    echo "Error: One or more required environment variables are missing."; exit 1;
fi

# Check if tenant already exists
TENANT_EXISTS=$(psql "$DATABASE_URL" -tAc "SELECT 1 FROM tenants WHERE tenant_api_key = '$TENANT_API_KEY'")

if [ "$TENANT_EXISTS" = "1" ]; then
    echo "Tenant with API key $TENANT_API_KEY already exists. Skipping insertion."
    exit 0
fi

TMP_CSV="/tmp/tenant_data.csv"
echo "tenant_api_key,chain_id,acl_contract_address,verifying_contract_address,pks_key,sks_key,public_params" > $TMP_CSV

echo "$TENANT_API_KEY,$CHAIN_ID,$ACL_CONTRACT_ADDRESS,$INPUT_VERIFIER_ADDRESS,\"\\x$(< "$PKS_FILE" xxd -p | tr -d '\n')\",\"\\x$(< "$SKS_FILE" xxd -p | tr -d '\n')\",\"\\x$(< "$PUBLIC_PARAMS_FILE" xxd -p | tr -d '\n')\"" >> $TMP_CSV

echo "Inserting tenant data using \COPY..."
psql "$DATABASE_URL" -c "\COPY tenants (tenant_api_key, chain_id, acl_contract_address, verifying_contract_address, pks_key, sks_key, public_params) FROM '$TMP_CSV' CSV HEADER;" || {
    echo "Error: Failed to insert tenant data."; exit 1;
}

rm -f $TMP_CSV
echo "Database initialization complete."
