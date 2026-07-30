import { describe, expect, it } from 'vitest';

import {
  buildSolanaUserDecryptMmrProofExtraData,
  solanaUserDecryptSigningMessage,
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
  '0x5a616d61206f6e652d74696d6520636f6e666964656e7469616c2076616c75652072657665616c' +
  '0a56657273696f6e3a20310a526571756573743a20' +
  '3330303864633633353236613063313831643436336335643332623635313934' +
  '3563646664636465306132653931396664383530396164656434646231363835';

const rustExtraData =
  '0x03' +
  '0000000000000000000000000000000000000000000000000000000000001234' +
  '5555555555555555555555555555555555555555555555555555555555555555' +
  '000000000000002a' +
  '00000003' +
  '010203';

describe('Solana user decrypt v3 wallet-message parity', () => {
  it('matches the Rust signing-message and extraData vectors with a non-empty proof tail', () => {
    expect(bytesToHex(solanaUserDecryptSigningMessage(vector))).toBe(rustPreimage);
    expect(bytesToHex(buildSolanaUserDecryptMmrProofExtraData(contextId, aclValueKey, proofSlot, mmrProofBytes))).toBe(
      rustExtraData,
    );
  });
});
