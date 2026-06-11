import type { EncryptionBits } from '../types/fheType.js';

import { describe, it, expect } from 'vitest';

import { toSolanaZkProof } from './SolanaZkProof-p.js';
import { SOLANA_CHAIN_TYPE_BIT } from './buildInputProofMetaData-p.js';

const CHAIN_ID = SOLANA_CHAIN_TYPE_BIT | 12345n;
const ACL = `0x${'33'.repeat(32)}`;
const CONTRACT = `0x${'11'.repeat(32)}`;
const USER = `0x${'22'.repeat(32)}`;
const CT = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8]);
// euint64 (64-bit). Branded EncryptionBits is a plain numeric value at runtime.
const BITS64 = [64] as unknown as readonly EncryptionBits[];

function mkProof(overrides: Partial<Record<'aclContractAddress', string>> = {}) {
  return toSolanaZkProof({
    chainId: CHAIN_ID,
    aclContractAddress: overrides.aclContractAddress ?? ACL,
    contractAddress: CONTRACT,
    userAddress: USER,
    ciphertextWithZkProof: CT,
    encryptionBits: BITS64,
  });
}

describe('SolanaZkProof', () => {
  it('builds with bytes32 identities and predicts one input handle per value', () => {
    const proof = mkProof();
    expect(proof.aclContractAddress).toBe(ACL);
    expect(proof.chainId).toBe(CHAIN_ID);

    const handles = proof.getInputHandles();
    expect(handles.length).toBe(1);

    const handle = handles[0]!;
    // Trailing handle metadata is what zama-host checks: the chain-type high bit
    // survives, fhe type is euint64, index 0, version 0.
    expect(handle.chainId).toBe(CHAIN_ID);
    expect(handle.fheTypeId).toBe(5);
    expect(handle.bytes32[21]).toBe(0);
    expect(handle.bytes32[31]).toBe(0);
  });

  it('is deterministic for identical inputs', () => {
    expect(mkProof().getInputHandles()[0]!.bytes32Hex).toBe(mkProof().getInputHandles()[0]!.bytes32Hex);
  });

  it('binds the bytes32 ACL identity into the handle prehash', () => {
    const base = mkProof().getInputHandles()[0]!;
    const other = mkProof({ aclContractAddress: `0x${'44'.repeat(32)}` }).getInputHandles()[0]!;
    expect(base.hash21).not.toBe(other.hash21);
  });

  it('rejects a non-bytes32 (20-byte) identity', () => {
    expect(() => mkProof({ aclContractAddress: `0x${'33'.repeat(20)}` })).toThrow();
  });
});
