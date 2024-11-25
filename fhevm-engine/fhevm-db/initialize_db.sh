#!/bin/bash

# 1: Create Database
echo "Creating database..."
sqlx database create

# 2: Run sqlx migrations
echo "Running migrations..."
sqlx migrate run --source /migrations || { echo "Failed to run migrations."; exit 1; }

# 3. Insert test tenant with keys
echo "Start preparing tenant query..."
TENANT_API_KEY=a1503fb6-d79b-4e9e-826d-44cf262f3e05
CHAIN_ID=12345
ACL_CONTRACT_ADDRESS=0x339EcE85B9E11a3A3AA557582784a15d7F82AAf2
INPUT_VERIFIER_ADDRESS=0x69dE3158643e738a0724418b21a35FAA20CBb1c5
PKS_FILE="/fhevm-keys/pks"
SKS_FILE="/fhevm-keys/sks"
PUBLIC_PARAMS_FILE="/fhevm-keys/pp"

TMP_CSV="/tmp/tenant_data.csv"
echo "tenant_api_key,chain_id,acl_contract_address,verifying_contract_address,pks_key,sks_key,public_params" > $TMP_CSV

echo "$TENANT_API_KEY,$CHAIN_ID,$ACL_CONTRACT_ADDRESS,$INPUT_VERIFIER_ADDRESS,\"\\x$(cat $PKS_FILE | xxd -p | tr -d '\n')\",\"\\x$(cat $SKS_FILE | xxd -p | tr -d '\n')\",\"\\x$(cat $PUBLIC_PARAMS_FILE | xxd -p | tr -d '\n')\"" >> $TMP_CSV

echo "Inserting tenant data using \COPY..."
psql $DATABASE_URL -c "\COPY tenants (tenant_api_key, chain_id, acl_contract_address, verifying_contract_address, pks_key, sks_key, public_params) FROM '$TMP_CSV' CSV HEADER;"

rm -f $TMP_CSV

echo "Database initialization complete."