#!/bin/bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [[ -f /prepare_database_url.sh ]]; then
  source /prepare_database_url.sh
else
  source "${script_dir}/prepare_database_url.sh"
fi

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
  MIGRATION_DIR="${script_dir}/migrations"
fi

echo "-------------- Start database initilaization --------------"

echo "Creating database..."
sqlx database create || { echo "Failed to create database."; exit 1; }

echo "Running migrations..."
sqlx migrate run --source $MIGRATION_DIR || { echo "Failed to run migrations."; exit 1; }

echo "Database initialization completed successfully."
