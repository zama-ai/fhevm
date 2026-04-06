#!/bin/bash
set -e

CHAIN_ID=${CHAIN_ID:-"12345"}

if [[ -z "$DATABASE_URL" || -z "$ACL_CONTRACT_ADDRESS" ]]; then
    echo "Error: One or more required environment variables are missing."; exit 1;
fi

psql "$DATABASE_URL" -c \
  "INSERT INTO host_chains (chain_id, name, acl_contract_address) \
   VALUES ('$CHAIN_ID', 'test chain', '$ACL_CONTRACT_ADDRESS') \
   ON CONFLICT DO NOTHING;"