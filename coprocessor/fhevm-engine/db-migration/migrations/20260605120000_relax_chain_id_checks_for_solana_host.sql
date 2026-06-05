-- RFC-021 encodes the host chain-type in the HIGH bit of the u64 chain id, so a Solana host
-- chain id is > i64::MAX and is stored as its negative two's-complement i64 bit pattern (see
-- fhevm-engine-common ChainId::from_canonical_u64). zama-host embeds that full u64 in derived
-- handles, and the coprocessor keys handle -> tenant/keys by the same value, so the chain_id /
-- host_chain_id columns must accept it.
--
-- These `>= 0` CHECKs were EVM-only assumptions (EVM chain ids never set the high bit). Relaxing
-- them lets a Solana host id round-trip. Genuinely-invalid ids are still rejected at the
-- application layer: ChainId::try_from stays strict for EVM inputs, and only the deliberate
-- from_canonical_u64 path (used for RFC-021 hosts) produces the high-bit pattern.

ALTER TABLE IF EXISTS host_chains DROP CONSTRAINT IF EXISTS host_chains_chain_id_check;
ALTER TABLE IF EXISTS verify_proofs DROP CONSTRAINT IF EXISTS verify_proofs_chain_id_check;
ALTER TABLE IF EXISTS host_chain_blocks_valid DROP CONSTRAINT IF EXISTS host_chain_blocks_valid_chain_id_check;
ALTER TABLE IF EXISTS kms_key_activation_events DROP CONSTRAINT IF EXISTS kms_key_activation_events_chain_id_check;
ALTER TABLE IF EXISTS kms_crs_activation_events DROP CONSTRAINT IF EXISTS kms_crs_activation_events_chain_id_check;
ALTER TABLE IF EXISTS computations DROP CONSTRAINT IF EXISTS computations_host_chain_id_positive;
ALTER TABLE IF EXISTS pbs_computations DROP CONSTRAINT IF EXISTS pbs_computations_host_chain_id_positive;
ALTER TABLE IF EXISTS ciphertext_digest DROP CONSTRAINT IF EXISTS ciphertext_digest_host_chain_id_positive;
ALTER TABLE IF EXISTS tenants DROP CONSTRAINT IF EXISTS tenants_chain_id_check;
