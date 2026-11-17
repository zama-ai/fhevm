#!/bin/sh
# Creates the role prf-service logs in as when Postgres initializes its data directory, before any
# coprocessor migration: each migration then grants its rights on `pads`
# (coprocessor/fhevm-engine/db-migration/db-scripts/grant_pad_roles.sql), as in preview. The name is
# fixed: that script grants only to `prf_service`.
set -eu
psql -U "${POSTGRES_USER}" -d postgres -qtA -v ON_ERROR_STOP=1 \
  -v password="${PRF_SERVICE_DATABASE_PASSWORD}" <<'SQL'
SELECT format('CREATE ROLE prf_service LOGIN PASSWORD %L', :'password') \gexec
SQL
