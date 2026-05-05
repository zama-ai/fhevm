#!/bin/bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [[ -f /prepare_database_url.sh ]]; then
  source /prepare_database_url.sh
else
  source "${script_dir}/prepare_database_url.sh"
fi

TABLE_NAME="$1"

if [[ -z "${DATABASE_URL:-}" ]]; then
    echo "Error: DATABASE_URL is not set." >&2; exit 1;
fi

psql "$DATABASE_URL" -P pager=off -c "\d+ $TABLE_NAME"
