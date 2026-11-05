#!/bin/sh
# Creates the role prf-service logs in as (PRF_SERVICE_DATABASE_USER and _PASSWORD of database.env),
# when Postgres initializes its data directory. Each coprocessor migration then grants it its rights
# on `pads` (coprocessor/fhevm-engine/db-migration/db-scripts/grant_pad_roles.sql).
set -eu
psql -U "${POSTGRES_USER}" -d postgres -qtA -v ON_ERROR_STOP=1 \
  -v user="${PRF_SERVICE_DATABASE_USER}" -v password="${PRF_SERVICE_DATABASE_PASSWORD}" <<'SQL'
SELECT format('CREATE ROLE %I LOGIN PASSWORD %L', :'user', :'password')
 WHERE NOT EXISTS (SELECT FROM pg_roles WHERE rolname = :'user') \gexec
SQL
