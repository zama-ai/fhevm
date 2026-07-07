import { describe, it, expect } from 'vitest';

import { ed25519 } from '@noble/curves/ed25519.js';
import { keccak_256 } from '@noble/hashes/sha3.js';

import { bytesToHex } from '../base/bytes.js';
import {
  buildSolanaUserDecryptRequest,
  solanaUserDecryptClientId,
  solanaUserDecryptSigningPreimage,
  SOLANA_USER_DECRYPT_ATTESTATION_TYPE,
  type SolanaUserDecryptInput,
} from './SolanaUserDecrypt-p.js';

// Fixed cross-impl vector. The expected hex below is the byte output of the Rust source of truth
// `kms-connector/crates/utils/src/types/solana_extra_data.rs`
// (`encode_solana_extra_data` / `solana_user_decrypt_signing_preimage`), captured by a standalone
// Rust program that copies that module's layout verbatim and prints hex for THIS exact vector.
// If the TS layout drifts from the Rust, these assertions fail.
const IDENTITY = new Uint8Array(32).fill(0x07);
const NONCE = new Uint8Array(32).fill(0x09);
const CONTEXT_ID = (() => {
  const c = new Uint8Array(32);
  c[30] = 0x12;
  c[31] = 0x34; // 0x1234 big-endian
  return c;
})();
const DOMAIN_KEYS = [new Uint8Array(32).fill(0x01), new Uint8Array(32).fill(0x02)];
const PUBLIC_KEY = new TextEncoder().encode('public-key-bytes');
const HANDLES = [new Uint8Array(32).fill(0x03), new Uint8Array(32).fill(0xaa)];
const ACL_VALUE_KEY = new Uint8Array(32).fill(0x55);

const VECTOR: SolanaUserDecryptInput = {
  contractsChainId: 0xcafen,
  publicKey: PUBLIC_KEY,
  handles: HANDLES,
  identity: IDENTITY,
  contextId: CONTEXT_ID,
  nonce: NONCE,
  allowedAclDomainKeys: DOMAIN_KEYS,
  startTimestamp: 1000n,
  durationSeconds: 3600n,
};

const RUST_PREIMAGE =
  '0x7a616d612d736f6c616e612d757365722d646563727970742d7632' + // "zama-solana-user-decrypt-v2"
  '000000000000cafe' + // contracts_chain_id u64 BE
  '00000010' +
  '7075626c69632d6b65792d6279746573' + // "public-key-bytes"
  '00000002' +
  '0303030303030303030303030303030303030303030303030303030303030303' +
  'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' +
  '0707070707070707070707070707070707070707070707070707070707070707' + // identity
  '0000000000000000000000000000000000000000000000000000000000001234' + // context_id
  '0909090909090909090909090909090909090909090909090909090909090909' + // nonce
  '00000002' +
  '0101010101010101010101010101010101010101010101010101010101010101' +
  '0202020202020202020202020202020202020202020202020202020202020202' +
  '00000000000003e8' + // start_timestamp 1000
  '0000000000000e10' + // duration_seconds 3600
  '0000000000000000000000000000000000000000000000000000000000000000' + // acl_value_key (absent)
  '0000000000000000' + // proof_slot (absent)
  '00000000'; // mmr_proof_len (absent, empty proof)

describe('SolanaUserDecrypt byte-parity with Rust source of truth', () => {
  it('builds the signing preimage byte-identically to solana_user_decrypt_signing_preimage', () => {
    expect(bytesToHex(solanaUserDecryptSigningPreimage(VECTOR))).toBe(RUST_PREIMAGE);
  });
});

describe('SolanaUserDecrypt signer and request builder', () => {
  // A fixed ed25519 seed; the matching identity must be used in the request.
  const SEED = new Uint8Array(32).fill(0x42);
  const PK = ed25519.getPublicKey(SEED);

  const signed: SolanaUserDecryptInput = { ...VECTOR, identity: PK };

  it('produces a request the connector can verify (sign -> verify round-trip)', () => {
    const req = buildSolanaUserDecryptRequest(signed, SEED);
    expect(req.attestationType).toBe(SOLANA_USER_DECRYPT_ATTESTATION_TYPE);

    const sig = Uint8Array.from(Buffer.from(req.signature.slice(2), 'hex'));
    expect(sig.length).toBe(64);

    const preimage = solanaUserDecryptSigningPreimage(signed);
    expect(ed25519.verify(sig, preimage, PK)).toBe(true);
  });

  it('binds the publicKey: a substituted key invalidates the signature', () => {
    const req = buildSolanaUserDecryptRequest(signed, SEED);
    const sig = Uint8Array.from(Buffer.from(req.signature.slice(2), 'hex'));

    const tampered = solanaUserDecryptSigningPreimage({
      ...signed,
      publicKey: new TextEncoder().encode('attacker-public-key'),
    });
    expect(ed25519.verify(sig, tampered, PK)).toBe(false);
  });

  it('rejects a secretKey that does not derive the identity', () => {
    expect(() => buildSolanaUserDecryptRequest({ ...signed, identity: IDENTITY }, SEED)).toThrow();
  });

  it('derives userAddress as keccak256(identity)[12..] lowercase', () => {
    const req = buildSolanaUserDecryptRequest(signed, SEED);
    const expected = bytesToHex(keccak_256(PK).subarray(12));
    expect(req.userAddress).toBe(expected);
    expect(req.userAddress).toBe(solanaUserDecryptClientId(PK));
    expect(req.userAddress).toBe(req.userAddress.toLowerCase());
  });

  it('carries the ed25519 auth fields as typed values, not packed into extraData (RFC-021)', () => {
    const req = buildSolanaUserDecryptRequest(signed, SEED);
    // With no aclValueKey, extraData falls back to context-only (v0x01 ‖ contextId).
    expect(req.extraData).toBe('0x01' + bytesToHex(CONTEXT_ID).slice(2));
    // The auth fields travel as typed gateway fields instead.
    expect(req.solanaUserIdentity).toBe(bytesToHex(PK));
    expect(req.solanaNonce).toBe(bytesToHex(NONCE));
    expect(req.solanaAllowedAclDomainKeys).toEqual(DOMAIN_KEYS.map((k) => bytesToHex(k)));
  });

  it('emits v0x03 extraData for a nonzero aclValueKey even with an empty proof', () => {
    const req = buildSolanaUserDecryptRequest({ ...signed, aclValueKey: ACL_VALUE_KEY }, SEED);
    expect(req.extraData).toBe(
      '0x03' + bytesToHex(CONTEXT_ID).slice(2) + bytesToHex(ACL_VALUE_KEY).slice(2) + '0000000000000000' + '00000000',
    );
  });
});
