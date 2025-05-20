# Gateway Listener

The **gw-listener** service listens for input proof verification events from the InputVerification contract and inserts them into the DB into the `verify_proofs` table: [verify_proofs table](../fhevm-db/migrations/20250207092623_verify_proofs.sql)

The following fields are insertion in by gw-listner:

 * zk_proof_id
 * chain_id
 * contract_address
 * user_address
 * input (the ciphertext + proof as a tfhe-rs serialized data structure)
 * created_at

At the time of insertion, the `verified` and `verified_at` fields are false and NULL, respectively.

The gw-listener will notify **zkproof-worker** services that work is available over the `verify_proof_requests` DB channel (configurable, but this is the default one).

Once a ZK proof request is verified, a zkproof-worker should set:
 * `verified = true`
 * `verified_at = NOW()` 
 * `handles = concatenated 32-byte handles` (s.t. the length of the handles field in bytes is a multiple of 32)

Then, zkproof-worker should notify the **transaction-sender** on the **verify_proof_responses** DB channel (configurable, but this is the default one).
