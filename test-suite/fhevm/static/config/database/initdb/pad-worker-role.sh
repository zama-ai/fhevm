#!/bin/sh
# Creates the role the pad-worker logs in as, as prf-service-role.sh does for prf-service. The name
# is fixed: grant_pad_roles.sql grants only to `pad_worker`.
set -eu
psql -U "${POSTGRES_USER}" -d postgres -qtA -v ON_ERROR_STOP=1 \
  -v password="${PAD_WORKER_DATABASE_PASSWORD}" <<'SQL'
SELECT format('CREATE ROLE pad_worker LOGIN PASSWORD %L', :'password') \gexec
SQL
