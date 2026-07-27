-- Flag ciphertexts that were materialised from a verified ZK proof input
-- (set by zkproof-worker), versus ciphertexts produced as computation
-- outputs by tfhe-worker. Defaults to FALSE so existing rows and any
-- non-zkproof-worker writer keep their previous semantics.

ALTER TABLE ciphertexts
ADD COLUMN IF NOT EXISTS is_input BOOLEAN NOT NULL DEFAULT FALSE;
