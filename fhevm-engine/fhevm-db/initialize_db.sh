#!/bin/bash
set -e

# Default to using absolute paths needed for Docker containers
# Using arg --no-absolute-paths is needed for local DB initialization
USE_ABSOLUTE_PATHS=true

for arg in "$@"; do
  if [[ "$arg" == "--no-absolute-paths" ]]; then
    USE_ABSOLUTE_PATHS=false
  fi
done

if [ "$USE_ABSOLUTE_PATHS" = true ]; then
  MIGRATION_DIR="/migrations"
  KEY_DIR="/fhevm-keys"
else
  MIGRATION_DIR="./migrations"
  KEY_DIR="./../fhevm-keys"
fi

echo "-------------- Start database initilaization --------------"

echo "Creating database..."
sqlx database create || { echo "Failed to create database."; exit 1; }

echo "Running migrations..."
sqlx migrate run --source $MIGRATION_DIR || { echo "Failed to run migrations."; exit 1; }

echo "-------------- Start inserting keys for tenant: $TENANT_API_KEY --------------"

CHAIN_ID=${CHAIN_ID:-"12345"}
PKS_FILE=${PKS_FILE:-"$KEY_DIR/pks"}
SKS_FILE=${SKS_FILE:-"$KEY_DIR/sks"}
PUBLIC_PARAMS_FILE=${PUBLIC_PARAMS_FILE:-"$KEY_DIR/pp"}
SNS_PK_FILE=${SNS_PK_FILE:-"$KEY_DIR/sns_pk"}
KEY_ID=${KEY_ID:-"10f49fdf75a123370ce2e2b1c5cc0615fb6e78dd829d0d850470cdbc84f15c11"}
KEY_ID_HEX="\\x${KEY_ID}"


for file in "$PKS_FILE" "$SKS_FILE" "$PUBLIC_PARAMS_FILE" "$SNS_PK_FILE"; do
    if [[ ! -f $file ]]; then
        echo "Error: Key file $file not found."; exit 1;
    fi
done

if [[ -z "$DATABASE_URL" || -z "$TENANT_API_KEY" || -z "$ACL_CONTRACT_ADDRESS" || -z "$INPUT_VERIFIER_ADDRESS" ]]; then
    echo "Error: One or more required environment variables are missing."; exit 1;
fi

TENANT_EXISTS=$(psql "$DATABASE_URL" -tAc "SELECT 1 FROM tenants WHERE tenant_api_key = '$TENANT_API_KEY'")

if [ "$TENANT_EXISTS" = "1" ]; then
    echo "Tenant with API key $TENANT_API_KEY already exists. Skipping insertion."
    exit 0
fi

TMP_CSV="/tmp/tenant_data.csv"
echo "tenant_api_key,chain_id,acl_contract_address,verifying_contract_address,pks_key,sks_key,public_params,sns_pk,key_id" > $TMP_CSV


import_large_file() {
  local file="$1"
  local db_url="$2"
  local chunk_size=8388608  # 8MB chunks
  local total_size
  total_size=$(stat -c %s "$file")
  
  echo "Creating large object and importing file ($total_size bytes)..." >&2
  
  # Create temp file for sending commands
  local tmpfile
  tmpfile=$(mktemp)
  
  # Generate PostgreSQL script with all commands in a single session
  cat > "$tmpfile" <<EOF
BEGIN;
-- Create large object
SELECT lo_create(0) AS oid \gset
-- Open large object for writing
SELECT lo_open(:'oid', 131072) AS fd \gset
EOF
  
  # Split the file into chunks and add commands for each
  local bytes_read=0
  local chunk_file
  while [ "$bytes_read" -lt "$total_size" ]; do
    chunk_file=$(mktemp)
    dd if="$file" bs=$chunk_size skip=$((bytes_read / chunk_size)) count=1 status=none > "$chunk_file"
    
    # Encode chunk to hex to safely embed in SQL
    echo "SELECT lowrite(:'fd', decode('$(xxd -p -c 0 "$chunk_file")', 'hex'));" >> "$tmpfile"
    rm "$chunk_file"
    
    bytes_read=$((bytes_read + chunk_size))
    if [ $bytes_read -gt $total_size ]; then bytes_read=$total_size; fi
    echo "Processed: $bytes_read / $total_size bytes ($(( (bytes_read * 100) / total_size ))%)" >&2
  done
  
  # Finish the transaction
  cat >> "$tmpfile" <<EOF
-- Close the file descriptor
SELECT lo_close(:'fd');
COMMIT;
-- Print the OID separately AFTER the transaction is committed
\echo 'OID_MARKER:'
\echo :oid
EOF
  
  # Execute the entire script and extract just the OID
  local oid=$(psql "$db_url" -f "$tmpfile" -t | grep -A 1 'OID_MARKER:' | tail -n 1 | tr -d ' ')
  rm "$tmpfile"

  # Verify
  local size=$(psql "$db_url" -t -c "SELECT pg_size_pretty(SUM(octet_length(data))) FROM pg_largeobject WHERE loid = $oid" | tr -d ' ')
  echo "Imported file. Size: $size" >&2
  
  echo "$oid"
}

echo "Importing large object from $SNS_PK_FILE ($(du -h "$SNS_PK_FILE" | cut -f1))..."
SNS_PK_OID=$(import_large_file "$SNS_PK_FILE" "$DATABASE_URL")
 

echo "$TENANT_API_KEY,$CHAIN_ID,$ACL_CONTRACT_ADDRESS,$INPUT_VERIFIER_ADDRESS,\
\"\\x$(< "$PKS_FILE" xxd -p | tr -d '\n')\",\
\"\\x$(< "$SKS_FILE" xxd -p | tr -d '\n')\",\
\"\\x$(< "$PUBLIC_PARAMS_FILE" xxd -p | tr -d '\n')\",\
$SNS_PK_OID,\"$KEY_ID_HEX\"" >> $TMP_CSV

echo "----------- Tenant data prepared for insertion: $TMP_CSV -----------"
 


echo "Inserting tenant data from CSV using \COPY..."
psql "$DATABASE_URL" -c "\COPY tenants (tenant_api_key, chain_id, acl_contract_address, verifying_contract_address, pks_key, sks_key, public_params, sns_pk, key_id) FROM '$TMP_CSV' CSV HEADER;" || {
    echo "Error: Failed to insert tenant data."; exit 1;
}

echo "Checking large object creation..."
psql "$DATABASE_URL" -c "SELECT loid as oid,
                        pg_size_pretty(SUM(octet_length(data))) as size
                     FROM pg_largeobject
                     GROUP BY loid;"

echo "Checking tenant entry references correct OID..."
psql "$DATABASE_URL" -c "SELECT t.tenant_id,
                        t.tenant_api_key,
                        t.sns_pk,
                        pg_size_pretty((SELECT SUM(octet_length(lo.data))
                                      FROM pg_largeobject lo
                                      WHERE lo.loid = t.sns_pk)) as sns_pk_size
                      FROM tenants t
                      WHERE t.tenant_api_key = '$TENANT_API_KEY';"

rm -f $TMP_CSV
echo "Database initialization keys insertion complete successfully."
