#!/bin/bash
set -euo pipefail

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

# Parse the connection target out of a PostgreSQL URL into DB_USER/DB_HOST/DB_PORT.
# These feed `aws rds generate-db-auth-token`, so they MUST match the host/user we
# actually connect with — otherwise the signed token is rejected. Falls back to the
# discrete DATABASE_USER / DATABASE_ENDPOINT env vars the chart also injects.
parse_database_target() {
  local url="${DATABASE_URL:-}"
  local authority userinfo hostport=""

  if [[ -n "$url" ]]; then
    authority="${url#*://}"      # strip "scheme://"
    authority="${authority%%/*}" # drop "/dbname?query"
    if [[ "$authority" == *@* ]]; then
      userinfo="${authority%@*}"
      hostport="${authority##*@}"
    else
      userinfo=""
      hostport="$authority"
    fi
    DB_USER="${userinfo%%:*}"     # strip any ":password"
  fi

  : "${DB_USER:=${DATABASE_USER:-}}"
  if [[ -z "$hostport" ]]; then
    hostport="${DATABASE_ENDPOINT:-}"
  fi

  if [[ "$hostport" == *:* ]]; then
    DB_HOST="${hostport%%:*}"
    DB_PORT="${hostport##*:}"
  else
    DB_HOST="$hostport"
    DB_PORT="5432"
  fi
}

# Remove the password component ("user:pass@" -> "user@") from a PostgreSQL URL,
# preserving the rest (scheme, host, dbname, query). Ensures PGPASSWORD governs.
strip_url_password() {
  local url="$1"
  local scheme rest authority remainder userinfo hostpart

  scheme="${url%%://*}"
  rest="${url#*://}"
  authority="${rest%%/*}"
  if [[ "$authority" == "$rest" ]]; then
    remainder=""
  else
    remainder="/${rest#*/}"
  fi

  if [[ "$authority" == *@* ]]; then
    userinfo="${authority%@*}"
    hostpart="${authority##*@}"
    userinfo="${userinfo%%:*}"   # drop ":password"
    authority="${userinfo}@${hostpart}"
  fi

  printf '%s://%s%s' "$scheme" "$authority" "$remainder"
}

if database_iam_auth_enabled; then
  if ! command -v aws >/dev/null 2>&1; then
    echo "ERROR: aws CLI is required for RDS IAM auth but was not found on PATH." >&2
    exit 1
  fi

  parse_database_target
  if [[ -z "${DB_HOST:-}" || -z "${DB_USER:-}" ]]; then
    echo "ERROR: RDS IAM auth needs a host and user; set DATABASE_URL (or DATABASE_ENDPOINT/DATABASE_USER)." >&2
    exit 1
  fi

  # Mint the RDS IAM auth token. It is a SigV4 presigned string whose query values
  # are already percent-encoded (e.g. %2F in X-Amz-Credential). It MUST be handed to
  # psql/sqlx verbatim via PGPASSWORD: embedding it in DATABASE_URL lets the consumer
  # percent-decode those %2F a second time, corrupting the signature so RDS rejects it
  # with "password authentication failed". The token is resolved with the pod's AWS
  # identity (IRSA / web identity) via the standard credential chain.
  if [[ -n "${DATABASE_IAM_REGION:-}" ]]; then
    _token="$(aws rds generate-db-auth-token \
      --hostname "$DB_HOST" --port "$DB_PORT" --username "$DB_USER" \
      --region "$DATABASE_IAM_REGION")"
  else
    _token="$(aws rds generate-db-auth-token \
      --hostname "$DB_HOST" --port "$DB_PORT" --username "$DB_USER")"
  fi
  if [[ -z "$_token" ]]; then
    echo "ERROR: failed to generate an RDS IAM auth token via aws CLI." >&2
    exit 1
  fi
  export PGPASSWORD="$_token"

  # TLS is configured purely through PG* env vars, which both psql (libpq) and
  # sqlx-cli honor (sqlx 0.7.x reads PGSSLMODE/PGSSLROOTCERT in PgConnectOptions).
  # RDS IAM auth requires SSL; verify-full needs the RDS CA bundle on disk.
  export PGSSLMODE=verify-full
  if [[ -n "${DATABASE_SSL_ROOT_CERT_PATH:-}" ]]; then
    export PGSSLROOTCERT="${DATABASE_SSL_ROOT_CERT_PATH}"
  fi

  # Ensure DATABASE_URL carries no stale password so PGPASSWORD governs the token.
  # (The chart omits the password in iam mode; this is defensive.)
  if [[ -n "${DATABASE_URL:-}" ]]; then
    export DATABASE_URL="$(strip_url_password "${DATABASE_URL}")"
  fi
fi
