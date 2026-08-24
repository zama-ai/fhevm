// The session orchestrator, and the one mapping it owns.
//
// `solanaUserDecryptLinkInputs` is the substantive piece: every link field must come from the
// signed permit itself — the KMS routing from the extraData the wallet signed, never from
// configuration — so the link this client computes can only disagree with the KMS if the permit
// does. The execute wiring around it is pinned to the extent real vectors allow: an answered
// transport feeds verification (which refuses garbage shares), and an unanswered one surfaces the
// session's own error untouched.

import type { SolanaAccessEvidence, SolanaHandleRequest, SolanaSigncryptedShare } from './index.js';
import type { SolanaPermitFields, SolanaSignedPermit } from '../permit/index.js';
import { describe, expect, it } from 'vitest';
import {
  PERMIT_IDENTITY_LEN,
  PERMIT_KMS_ROUTING_LEN,
  PERMIT_KMS_ROUTING_VERSION,
  PERMIT_SIGNATURE_LEN,
  PERMIT_TRANSPORT_KEY_LEN,
  decodeSolanaPermitFields,
} from '../permit/index.js';
import { SolanaUserDecryptRunError, executeSolanaUserDecrypt, solanaUserDecryptLinkInputs } from './index.js';

////////////////////////////////////////////////////////////////////////////////

const PERMIT_CHAIN_ID = 10_037_641_751_006_774_702n;

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
    userPubkey: identity(0x11),
    transportKey: new Uint8Array(PERMIT_TRANSPORT_KEY_LEN).fill(0x55),
    allowedAclDomainKeys: [],
    startTimestamp: 1_767_229_380n,
    durationSeconds: 604_800n,
    verifyingProgramId: identity(0x22),
    chainId: PERMIT_CHAIN_ID,
    extraData: routing(),
  });

const signedPermit = (): SolanaSignedPermit => ({
  fields: permitFields(),
  signature: new Uint8Array(PERMIT_SIGNATURE_LEN).fill(0x77),
});

const handle = (): Uint8Array => {
  const bytes = new Uint8Array(32).fill(0xa1);
  new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength).setBigUint64(22, PERMIT_CHAIN_ID, false);
  bytes[30] = 0;
  bytes[31] = 0;
  return bytes;
};

const REQUESTS: readonly SolanaHandleRequest[] = [
  { handle: handle(), subject: identity(0x11), encryptedValueId: identity(0xe1) },
];

const evidence = {
  resolve: (request: SolanaHandleRequest): Promise<SolanaAccessEvidence> =>
    Promise.resolve({
      handle: request.handle,
      subject: request.subject,
      encryptedValueId: identity(0xe1),
      encryptedValueAccount: identity(0xea),
      proofLeafCount: 0n,
      accessProof: new Uint8Array(0),
      peaks: [],
    }),
};

const clock = { delay: (): Promise<void> => Promise.resolve() };

const session = () => ({
  signedPermit: signedPermit(),
  keyPair: { secretKey: {}, publicKey: {}, publicKeyBytes: new Uint8Array(869) } as never,
  warnings: [],
});

const verification = {
  signers: [{ partyId: 1, address: '0x0000000000000000000000000000000000000001' }],
  fheParameter: 'test',
};

////////////////////////////////////////////////////////////////////////////////

describe('the link inputs a permit pins', () => {
  it('takes every field from the permit itself, routing included', () => {
    const fields = permitFields();
    const handles = [handle()];

    expect(solanaUserDecryptLinkInputs(fields, handles)).toEqual({
      userPubkey: fields.userPubkey,
      hostChainId: PERMIT_CHAIN_ID,
      verifyingProgramId: fields.verifyingProgramId,
      kmsContextId: identity(0x33),
      kmsEpochId: identity(0x44),
      handles,
      transportKey: fields.transportKey,
    });
  });
});

describe('executing one user decryption', () => {
  it('feeds an answered transport into verification, which refuses shares that prove nothing', async () => {
    const shares: readonly SolanaSigncryptedShare[] = [{ signature: '0x00', payload: '0x00', extraData: '0x' }];
    const transport = { submit: () => Promise.resolve({ ok: true as const, response: shares }) };

    await expect(
      executeSolanaUserDecrypt({ session: session(), requests: REQUESTS, evidence, transport, clock, verification }),
    ).rejects.toThrow();
  });

  it('surfaces the session error of a run that was never answered, untouched', async () => {
    const transport = {
      submit: () =>
        Promise.resolve({ ok: false as const, rejection: { kind: 'refused', label: 'validation_failed' } as const }),
    };

    await expect(
      executeSolanaUserDecrypt({
        session: session(),
        requests: REQUESTS,
        evidence,
        transport,
        clock,
        attempts: 1,
        verification,
      }),
    ).rejects.toThrow(SolanaUserDecryptRunError);
  });
});
