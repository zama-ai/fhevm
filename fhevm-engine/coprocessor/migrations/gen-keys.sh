#!/bin/sh

echo "
INSERT INTO tenants(tenant_api_key, chain_id, verifying_contract_address, pks_key, sks_key, cks_key)
VALUES (
  'a1503fb6-d79b-4e9e-826d-44cf262f3e05',
  12345,
  '0x6819e3aDc437fAf9D533490eD3a7552493fCE3B1',
  decode('$(cat ../../fhevm-keys/pks | xxd -p | tr -d '\n')','hex'),
  decode('$(cat ../../fhevm-keys/sks | xxd -p | tr -d '\n')','hex'),
  decode('$(cat ../../fhevm-keys/cks | xxd -p | tr -d '\n')','hex')
)
" > 20240723111257_coprocessor.sql
