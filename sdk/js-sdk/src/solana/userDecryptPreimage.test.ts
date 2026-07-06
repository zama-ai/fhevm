import { describe, expect, it } from 'vitest';

import {
  buildSolanaUserDecryptMmrProofExtraData,
  solanaUserDecryptSigningPreimage,
  type SolanaUserDecryptInput,
} from '../core/coprocessor/SolanaUserDecrypt-p.js';
import { bytesToHex } from './proof.js';

const identity = new Uint8Array(32).fill(0x07);
const nonce = new Uint8Array(32).fill(0x09);
const contextId = (() => {
  const c = new Uint8Array(32);
  c[30] = 0x12;
  c[31] = 0x34;
  return c;
})();
const domainKeys = [new Uint8Array(32).fill(0x01), new Uint8Array(32).fill(0x02)];
const publicKey = new TextEncoder().encode('public-key-bytes');
const handles = [new Uint8Array(32).fill(0x03), new Uint8Array(32).fill(0xaa)];
const aclValueKey = new Uint8Array(32).fill(0x55);
const mmrProofBytes = new Uint8Array([0x01, 0x02, 0x03]);
const proofSlot = 42n;

const vector: SolanaUserDecryptInput = {
  contractsChainId: 0xcafen,
  publicKey,
  handles,
  identity,
  contextId,
  nonce,
  allowedAclDomainKeys: domainKeys,
  startTimestamp: 1000n,
  durationSeconds: 3600n,
  aclValueKey,
  mmrProofBytes,
  proofSlot,
};

const rustPreimage =
  '0x7a616d612d736f6c616e612d757365722d646563727970742d7632' +
  '000000000000cafe' +
  '00000010' +
  '7075626c69632d6b65792d6279746573' +
  '00000002' +
  '0303030303030303030303030303030303030303030303030303030303030303' +
  'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' +
  '0707070707070707070707070707070707070707070707070707070707070707' +
  '0000000000000000000000000000000000000000000000000000000000001234' +
  '0909090909090909090909090909090909090909090909090909090909090909' +
  '00000002' +
  '0101010101010101010101010101010101010101010101010101010101010101' +
  '0202020202020202020202020202020202020202020202020202020202020202' +
  '00000000000003e8' +
  '0000000000000e10' +
  '5555555555555555555555555555555555555555555555555555555555555555' +
  '000000000000002a' +
  '00000003' +
  '010203';

const rustExtraData =
  '0x02' +
  '0000000000000000000000000000000000000000000000000000000000001234' +
  '5555555555555555555555555555555555555555555555555555555555555555' +
  '000000000000002a' +
  '00000003' +
  '010203';

describe('Solana user decrypt v2 MMR-tail byte parity', () => {
  it('matches the Rust preimage and extraData vectors with a non-empty proof tail', () => {
    expect(bytesToHex(solanaUserDecryptSigningPreimage(vector))).toBe(rustPreimage);
    expect(bytesToHex(buildSolanaUserDecryptMmrProofExtraData(contextId, aclValueKey, proofSlot, mmrProofBytes))).toBe(
      rustExtraData,
    );
  });
});
