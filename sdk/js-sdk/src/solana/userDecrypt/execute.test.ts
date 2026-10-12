// The session orchestrator, and the one mapping it owns.
//
// `solanaUserDecryptRequestInputs` is the substantive piece: every permit-owned field must come from
// the signed permit itself — the KMS routing from the extraData the wallet signed, never from
// `HostConfig` again — and the one field the permit does not carry, the gateway domain, from
// `HostConfig`, handed in beside the signer set of the permit's `KmsContext`. So the link this
// client computes can only disagree with the KMS if the permit does, or if `HostConfig`'s domain is
// not the gateway's. The execute wiring around it: every attempt submits the same bytes under the
// permit's one signature, a request that can never be submitted is refused before the network, an
// answer feeds verification (which refuses garbage shares), and an unanswered run surfaces the retry
// loop's error untouched.

import type { SolanaSigncryptedShare, SolanaUserDecryptHandleEntry, SolanaUserDecryptRequestJson } from './index.js';
import type { SolanaPermitFields, SolanaSignedPermit } from '../permit/index.js';
import { beforeAll, describe, expect, it, vi } from 'vitest';
import {
  PERMIT_IDENTITY_LEN,
  PERMIT_KMS_ROUTING_LEN,
  PERMIT_KMS_ROUTING_VERSION,
  PERMIT_SIGNATURE_LEN,
  PERMIT_TRANSPORT_KEY_LEN,
  decodeSolanaPermitFields,
} from '../permit/index.js';
import { setFhevmRuntimeConfig } from '../internal/config.js';
import { getSolanaRuntime } from '../internal/runtime.js';
import { SolanaDecryptionRunError } from '../decryption/retry.js';
import { bytesToHex } from '../../core/base/bytes.js';
import {
  MAX_SOLANA_DECRYPT_HANDLES,
  SolanaUserDecryptRequestError,
  executeSolanaUserDecrypt,
  generateSolanaTransportKeyPair,
  solanaUserDecryptRequestInputs,
} from './index.js';

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
    userAddress: identity(0x11),
    transportKey: new Uint8Array(PERMIT_TRANSPORT_KEY_LEN).fill(0x55),
    allowedScopes: [],
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

const ENTRIES: readonly SolanaUserDecryptHandleEntry[] = [
  { handle: handle(), ownerAddress: identity(0x11), encryptedStore: identity(0xea) },
];

const clock = { delay: (): Promise<void> => Promise.resolve() };

const session = () => ({
  signedPermit: signedPermit(),
  keyPair: { secretKey: {}, publicKey: {}, publicKeyBytes: new Uint8Array(869) } as never,
  warnings: [],
});

const gatewayEip712Domain = {
  name: 'Decryption',
  version: '1',
  chainId: 31337n,
  verifyingContract: '0x0000000000000000000000000000000000000042',
};

const verification = {
  signers: [{ partyId: 1, address: '0x0000000000000000000000000000000000000001' }],
  fheParameter: 'test',
  gatewayEip712Domain,
};

////////////////////////////////////////////////////////////////////////////////

describe('the request inputs a permit pins', () => {
  it('takes every permit-owned field from the permit itself, routing included, and the domain from trust', () => {
    const fields = permitFields();
    const handles = [handle()];

    expect(solanaUserDecryptRequestInputs(fields, handles, gatewayEip712Domain)).toEqual({
      userAddress: fields.userAddress,
      handles,
      transportKey: fields.transportKey,
      gatewayEip712Domain,
      extraData: routing(),
    });
  });
});

describe('executing one user decryption', () => {
  beforeAll(() => {
    setFhevmRuntimeConfig({});
  });

  // The property the whole retry design exists for: a retry is never a second wallet prompt.
  it('submits the same bytes under the one signature on every attempt', async () => {
    const submit = vi.fn((_request: SolanaUserDecryptRequestJson) =>
      Promise.resolve({ ok: false as const, rejection: { code: 'acl_denied' } }),
    );
    const permit = session();

    await expect(
      executeSolanaUserDecrypt({
        runtime: getSolanaRuntime(),
        session: permit,
        entries: ENTRIES,
        submit,
        clock,
        verification,
      }),
    ).rejects.toThrow(SolanaDecryptionRunError);

    const [first, ...rest] = submit.mock.calls.map(([request]) => request);
    expect(rest.length).toBeGreaterThan(0);
    for (const request of rest) expect(request).toBe(first);
    expect(first?.signature).toBe(bytesToHex(permit.signedPermit.signature));
  });

  // The stateless limits are properties of the request itself; refusing here costs no submission.
  it('refuses a request over the handle cap without submitting', async () => {
    const submit = vi.fn();
    const overCap = Array.from({ length: MAX_SOLANA_DECRYPT_HANDLES + 1 }, () => ENTRIES[0]!);

    await expect(
      executeSolanaUserDecrypt({
        runtime: getSolanaRuntime(),
        session: session(),
        entries: overCap,
        submit,
        clock,
        verification,
      }),
    ).rejects.toThrow(SolanaUserDecryptRequestError);
    expect(submit).not.toHaveBeenCalled();
  });

  it('feeds an answer into verification, which refuses shares that prove nothing', async () => {
    const shares: readonly SolanaSigncryptedShare[] = [{ signature: '0x00', payload: '0x00', extraData: '0x' }];
    const submit = () => Promise.resolve({ ok: true as const, response: shares });
    // A real key pair, so the refusal comes from the KMS client reading the share rather than from
    // a malformed key rejected before any share is read.
    const keyPair = await generateSolanaTransportKeyPair(getSolanaRuntime());

    await expect(
      executeSolanaUserDecrypt({
        runtime: getSolanaRuntime(),
        session: { ...session(), keyPair },
        entries: ENTRIES,
        submit,
        clock,
        verification,
      }),
    ).rejects.toThrow(/response parsing failed/);
  });

  it('surfaces the retry loop error of a run that was never answered, untouched', async () => {
    const submit = () => Promise.resolve({ ok: false as const, rejection: { code: 'malformed' } });

    await expect(
      executeSolanaUserDecrypt({
        runtime: getSolanaRuntime(),
        session: session(),
        entries: ENTRIES,
        submit,
        clock,
        attempts: 1,
        verification,
      }),
    ).rejects.toThrow(SolanaDecryptionRunError);
  });
});
