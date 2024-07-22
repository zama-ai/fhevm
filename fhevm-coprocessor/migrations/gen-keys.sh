#!/bin/sh

echo "
INSERT INTO tenants(tenant_api_key, pks_key, sks_key, cks_key)
VALUES (
  'a1503fb6-d79b-4e9e-826d-44cf262f3e05',
  decode('$(cat pks | xxd -p | tr -d '\n')','hex'),
  decode('$(cat sks | xxd -p | tr -d '\n')','hex'),
  decode('$(cat cks | xxd -p | tr -d '\n')','hex')
)
"
