import { createFhevmEncryptClient, createFhevmDecryptClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptDecryptSlowTests } from '../ethers-common/clientEncrypt.encryptDecrypt.slow.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// Seeded ("deterministic") public encryption — full round-trip + verify-by-reproduction.
//
// This is the empirical check that a SEEDED proven compact list verifies and
// expands end-to-end on the deployed coprocessor (the serialization format is
// identical to the non-seeded path, but this proves it). Requires a real backend
// (tfhe-rs 1.6.1) — skipped on cleartext chains, like the non-seeded slow test.
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encryptDecrypt.seeded.slow.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encryptDecrypt.seeded.slow.test.ts
//
////////////////////////////////////////////////////////////////////////////////

// Fixed 32-byte seed: determinism is the whole point — a constant seed keeps the
// test reproducible. (In production the seed is generated per-encryption via
// generateEncryptionSeed() and kept secret, shared only with the verifier.)
const SEED = new Uint8Array(32).fill(0xa5);

defineClientEncryptDecryptSlowTests({
  runIf: !isCleartext(getEthersTestConfig().chainName),
  createFhevmEncryptClient: (params) => createFhevmEncryptClient(params),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
  seed: SEED,
});
