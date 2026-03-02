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
else
  MIGRATION_DIR="./migrations"
fi

echo "-------------- Start database initilaization --------------"

echo "Creating database..."
sqlx database create || { echo "Failed to create database."; exit 1; }

echo "Running migrations..."
sqlx migrate run --source $MIGRATION_DIR || { echo "Failed to run migrations."; exit 1; }

echo "-------------- Start inserting a host chain --------------"

CHAIN_ID=${CHAIN_ID:-"12345"}

if [[ -z "$DATABASE_URL" || -z "$ACL_CONTRACT_ADDRESS" ]]; then
    echo "Error: One or more required environment variables are missing."; exit 1;
fi

psql "$DATABASE_URL" -c \
  "INSERT INTO host_chains (chain_id, name, acl_contract_address) \
   VALUES ('$CHAIN_ID', 'test chain', '$ACL_CONTRACT_ADDRESS');" || {
    echo "Error: Failed to insert host chain data."; exit 1;
}

echo "Database initialization completed successfully."
