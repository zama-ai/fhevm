#!/bin/sh

echo "
INSERT INTO tenants(tenant_api_key, pks_key, sks_key, cks_key)
VALUES (
  'a1503fb6-d79b-4e9e-826d-44cf262f3e05',
  decode('$(cat ../fhevm-keys/pks | xxd -p | tr -d '\n')','hex'),
  decode('$(cat ../fhevm-keys/sks | xxd -p | tr -d '\n')','hex'),
  decode('$(cat ../fhevm-keys/cks | xxd -p | tr -d '\n')','hex')
)
" > 20240723111257_coprocessor.sql
