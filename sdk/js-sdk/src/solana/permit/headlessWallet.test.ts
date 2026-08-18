// The headless wallet, held to the same contract as a real one.
//
// What matters is not that it signs, but that it signs as a conforming wallet does: handed the
// canonical text, it builds the sRFC-38 envelope itself and signs those bytes — so a permit signed
// headlessly verifies under exactly the reconstruction every verifier runs, and a test or a
// server-side agent exercises the same channel a browser wallet would.

import type { SolanaPermitFields } from './index.js';
import { ed25519 } from '@noble/curves/ed25519.js';
import { describe, expect, it } from 'vitest';
import {
  PERMIT_IDENTITY_LEN,
  PERMIT_KMS_ROUTING_LEN,
  PERMIT_KMS_ROUTING_VERSION,
  PERMIT_TRANSPORT_KEY_LEN,
  buildSolanaPermitEnvelope,
  decodeSolanaPermitFields,
  signSolanaPermit,
  solanaPermitWalletFromSecretKey,
} from './index.js';

////////////////////////////////////////////////////////////////////////////////

const SEED = new Uint8Array(32).fill(0x07);
const PUBKEY = ed25519.getPublicKey(SEED);
const KEYPAIR_64 = new Uint8Array([...SEED, ...PUBKEY]);

const identity = (fill: number): Uint8Array => new Uint8Array(PERMIT_IDENTITY_LEN).fill(fill);

const routing = (): Uint8Array => {
  const bytes = new Uint8Array(PERMIT_KMS_ROUTING_LEN);
  bytes[0] = PERMIT_KMS_ROUTING_VERSION;
  bytes.set(identity(0x33), 1);
  bytes.set(identity(0x44), 1 + PERMIT_IDENTITY_LEN);
  return bytes;
};

const permitFields = (): SolanaPermitFields =>
  decodeSolanaPermitFields({
    userPubkey: PUBKEY,
    transportKey: new Uint8Array(PERMIT_TRANSPORT_KEY_LEN),
    allowedAclDomainKeys: [identity(0x01)],
    startTimestamp: 1_767_229_380n,
    durationSeconds: 604_800n,
    verifyingProgramId: identity(0x22),
    chainId: 10_037_641_751_006_774_702n,
    extraData: routing(),
  });

////////////////////////////////////////////////////////////////////////////////

describe('a headless wallet', () => {
  it('signs a permit that verifies under the same reconstruction as any wallet', async () => {
    const wallet = solanaPermitWalletFromSecretKey(SEED);
    const fields = permitFields();

    const signed = await signSolanaPermit(wallet, fields);

    // `signSolanaPermit` already verified; this restates the claim against the raw primitives.
    expect(ed25519.verify(signed.signature, buildSolanaPermitEnvelope(fields), PUBKEY)).toBe(true);
  });

  it('accepts the 64-byte Solana keypair form, and derives the same account', () => {
    expect(solanaPermitWalletFromSecretKey(KEYPAIR_64).publicKey).toEqual(PUBKEY);
    expect(solanaPermitWalletFromSecretKey(SEED).publicKey).toEqual(PUBKEY);
  });

  it('refuses a 64-byte key whose public half is not the seed`s', () => {
    const forged = new Uint8Array(KEYPAIR_64);
    forged[63] = (forged[63] ?? 0) ^ 0x01;
    expect(() => solanaPermitWalletFromSecretKey(forged)).toThrow('public half');
  });

  it('refuses a key of any other width', () => {
    expect(() => solanaPermitWalletFromSecretKey(new Uint8Array(31))).toThrow('32 or 64');
  });
});
