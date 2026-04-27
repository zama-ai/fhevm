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

REMOVE_TENANTS_PREVIOUS_VERSION=20260128095634

echo "-------------- Start database initilaization --------------"

echo "Creating database..."
sqlx database create || { echo "Failed to create database."; exit 1; }

echo "Running migrations..."
if [ "$RUN_MIGRATIONS_UNTIL_REMOVE_TENANTS" = "true" ]; then
  sqlx migrate run --source $MIGRATION_DIR --target-version $REMOVE_TENANTS_PREVIOUS_VERSION || { echo "Failed to run migrations."; exit 1; }
else
  sqlx migrate run --source $MIGRATION_DIR || { echo "Failed to run migrations."; exit 1; }
fi

echo "Database initialization completed successfully."
