#!/bin/bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
workspace_dir="$(cd "${script_dir}/.." && pwd)"

database_iam_auth_enabled() {
  case "${DATABASE_IAM_AUTH_ENABLED:-}" in
    1|[Tt][Rr][Uu][Ee]|[Yy][Ee][Ss]|[Oo][Nn])
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

resolve_database_url_cmd() {
  if [[ -x /usr/local/bin/resolve_database_url ]]; then
    /usr/local/bin/resolve_database_url
  elif [[ -x "${workspace_dir}/target/release/resolve_database_url" ]]; then
    "${workspace_dir}/target/release/resolve_database_url"
  elif [[ -x "${workspace_dir}/target/debug/resolve_database_url" ]]; then
    "${workspace_dir}/target/debug/resolve_database_url"
  elif command -v cargo >/dev/null 2>&1; then
    (
      cd "${workspace_dir}"
      cargo run --quiet -p fhevm-engine-common --bin resolve_database_url
    )
  else
    echo "ERROR: resolve_database_url binary is unavailable. Build it locally or run inside the db-migration image." >&2
    return 1
  fi
}

if database_iam_auth_enabled; then
  _resolved_url="$(resolve_database_url_cmd)" || {
    echo "Failed to resolve DATABASE_URL" >&2
    exit 1
  }
  export DATABASE_URL="${_resolved_url}"
  export PGSSLMODE=verify-full
  if [[ -n "${DATABASE_SSL_ROOT_CERT_PATH:-}" ]]; then
    export PGSSLROOTCERT="${DATABASE_SSL_ROOT_CERT_PATH}"
  fi
fi
