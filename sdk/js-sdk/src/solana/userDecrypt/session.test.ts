// One permit through several requests.
//
// Everything here is about what changes between attempts and what must not. The bytes stay identical
// across attempts: the service, not the request, is what a retry waits out. And the signature never
// changes at all: the runner is given a permit that is already signed and has no way to ask for
// another, which is what keeps a retry from turning into a second wallet prompt.

import type {
  SolanaUserDecryptHandleEntry,
  SolanaUserDecryptRejection,
  SolanaUserDecryptRequestJson,
  SolanaUserDecryptTransport,
  SolanaUserDecryptTransportOutcome,
} from './index.js';
import type { SolanaPermitFields, SolanaSignedPermit } from '../permit/index.js';
import { describe, expect, it, vi } from 'vitest';
import {
  MAX_SOLANA_USER_DECRYPT_HANDLES,
  SOLANA_USER_DECRYPT_DEFAULT_ATTEMPTS,
  SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS,
  SolanaUserDecryptRequestError,
  SolanaUserDecryptRunError,
  runSolanaUserDecrypt,
} from './index.js';
import {
  PERMIT_IDENTITY_LEN,
  PERMIT_KMS_ROUTING_LEN,
  PERMIT_KMS_ROUTING_VERSION,
  PERMIT_SIGNATURE_LEN,
  PERMIT_TRANSPORT_KEY_LEN,
  decodeSolanaPermitFields,
} from '../permit/index.js';
import { bytesToHex } from '../proof.js';

////////////////////////////////////////////////////////////////////////////////
// A permit, a handle, and the ports around them
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
    transportKey: new Uint8Array(PERMIT_TRANSPORT_KEY_LEN),
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

/** One ebool handle on the permit's host chain. */
const handle = (): Uint8Array => {
  const bytes = new Uint8Array(32).fill(0xa1);
  new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength).setBigUint64(22, PERMIT_CHAIN_ID, false);
  bytes[30] = 0;
  bytes[31] = 0;
  return bytes;
};

const ENTRIES: readonly SolanaUserDecryptHandleEntry[] = [
  { handle: handle(), allowedKey: identity(0x11), encryptedValueAccount: identity(0xea) },
];

/**
 * A transport that answers a scripted sequence of outcomes, recording every body it was given. The
 * last entry of the script answers every attempt after it, so a one-entry script is an outcome that
 * repeats.
 */
function scriptedTransport(script: readonly SolanaUserDecryptTransportOutcome<string>[]) {
  let attempt = 0;
  const submit = vi.fn((_request: SolanaUserDecryptRequestJson) => {
    const outcome = script[Math.min(attempt, script.length - 1)];
    attempt += 1;
    if (outcome === undefined) {
      throw new Error('the transport script is empty');
    }
    return Promise.resolve(outcome);
  });
  const transport: SolanaUserDecryptTransport<string> = { submit };
  return { submit, transport };
}

const rejectedWith = (rejection: SolanaUserDecryptRejection): SolanaUserDecryptTransportOutcome<string> => ({
  ok: false,
  rejection,
});
const answered: SolanaUserDecryptTransportOutcome<string> = { ok: true, response: 'shares' };

function recordingClock() {
  const delay = vi.fn(() => Promise.resolve());
  return { delay, clock: { delay } };
}

/** The run error a call produced, or a failure if it produced anything else. */
async function runErrorOf(call: () => Promise<unknown>): Promise<SolanaUserDecryptRunError> {
  try {
    await call();
  } catch (error) {
    if (error instanceof SolanaUserDecryptRunError) {
      return error;
    }
    throw error;
  }
  throw new Error('expected the run to fail, it resolved');
}

////////////////////////////////////////////////////////////////////////////////

describe('a request that is answered', () => {
  it('is submitted once, and the answer is returned as the transport gave it', async () => {
    const { submit, transport } = scriptedTransport([answered]);
    const { clock, delay } = recordingClock();

    const result = await runSolanaUserDecrypt({ signedPermit: signedPermit(), entries: ENTRIES, transport, clock });

    expect(result).toEqual({ response: 'shares', attempts: 1 });
    expect(submit).toHaveBeenCalledTimes(1);
    expect(delay).not.toHaveBeenCalled();
  });
});

describe('a rejection nothing repairs', () => {
  it('is reported after one submission, without waiting', async () => {
    const { submit, transport } = scriptedTransport([rejectedWith({ kind: 'refused', label: 'validation_failed' })]);
    const { clock, delay } = recordingClock();

    const error = await runErrorOf(() =>
      runSolanaUserDecrypt({ signedPermit: signedPermit(), entries: ENTRIES, transport, clock }),
    );

    expect(error.rejection).toEqual({ kind: 'refused', label: 'validation_failed' });
    expect(error.attempts).toBe(1);
    expect(submit).toHaveBeenCalledTimes(1);
    expect(delay).not.toHaveBeenCalled();
  });
});

describe('an overloaded relayer', () => {
  it('is waited out for the delay it named, and given the very same bytes', async () => {
    const { submit, transport } = scriptedTransport([
      rejectedWith({ kind: 'overloaded', retryAfterSeconds: 3 }),
      answered,
    ]);
    const { clock, delay } = recordingClock();
    const permit = signedPermit();

    const result = await runSolanaUserDecrypt({ signedPermit: permit, entries: ENTRIES, transport, clock });

    expect(result.attempts).toBe(2);
    expect(delay).toHaveBeenCalledExactlyOnceWith(3);
    const [first, second] = submit.mock.calls.map((call) => call[0]);
    expect(second).toBe(first);
    expect((second as { signature: string }).signature).toBe(bytesToHex(permit.signature));
  });
});

describe('a request the Connector left unanswered', () => {
  // The leaf the Connector asked the coprocessors for may not have been indexed yet when it asked.
  // The request itself is right, so the same bytes are submitted again after a wait.
  it('is submitted again unchanged, after the default wait', async () => {
    const { submit, transport } = scriptedTransport([rejectedWith({ kind: 'unanswered' }), answered]);
    const { clock, delay } = recordingClock();

    const result = await runSolanaUserDecrypt({ signedPermit: signedPermit(), entries: ENTRIES, transport, clock });

    expect(result.attempts).toBe(2);
    expect(delay).toHaveBeenCalledExactlyOnceWith(SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS);
    const [first, second] = submit.mock.calls.map((call) => call[0]);
    expect(second).toBe(first);
  });
});

describe('the backoff between attempts', () => {
  // Constant-rate retries spend the whole budget in a burst against a service that needed time. The
  // wait doubles with each attempt made.
  it('doubles the relayer-named delay on each unchanged retry', async () => {
    const { transport } = scriptedTransport([
      rejectedWith({ kind: 'overloaded', retryAfterSeconds: 3 }),
      rejectedWith({ kind: 'overloaded', retryAfterSeconds: 3 }),
      answered,
    ]);
    const { clock, delay } = recordingClock();

    const result = await runSolanaUserDecrypt({ signedPermit: signedPermit(), entries: ENTRIES, transport, clock });

    expect(result.attempts).toBe(3);
    expect(delay.mock.calls).toEqual([[3], [6]]);
  });

  it('waits before every resubmission of an unanswered request, and the wait grows', async () => {
    const { transport } = scriptedTransport([rejectedWith({ kind: 'unanswered' })]);
    const { clock, delay } = recordingClock();

    await runErrorOf(() => runSolanaUserDecrypt({ signedPermit: signedPermit(), entries: ENTRIES, transport, clock }));

    // Four attempts and three waits between them: the last rejection is reported, not waited on.
    expect(delay.mock.calls).toEqual([
      [SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS],
      [SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS * 2],
      [SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS * 4],
    ]);
  });
});

describe('a request that is never answered', () => {
  it('stops at the attempt budget and reports what it last saw', async () => {
    const { submit, transport } = scriptedTransport([rejectedWith({ kind: 'unanswered' })]);
    const { clock } = recordingClock();

    const error = await runErrorOf(() =>
      runSolanaUserDecrypt({ signedPermit: signedPermit(), entries: ENTRIES, transport, clock }),
    );

    expect(submit).toHaveBeenCalledTimes(SOLANA_USER_DECRYPT_DEFAULT_ATTEMPTS);
    expect(error.attempts).toBe(SOLANA_USER_DECRYPT_DEFAULT_ATTEMPTS);
    expect(error.rejection).toEqual({ kind: 'unanswered' });
  });

  it('honors a smaller budget when one is given', async () => {
    const { submit, transport } = scriptedTransport([rejectedWith({ kind: 'unanswered' })]);
    const { clock } = recordingClock();

    const error = await runErrorOf(() =>
      runSolanaUserDecrypt({ signedPermit: signedPermit(), entries: ENTRIES, transport, clock, attempts: 2 }),
    );

    expect(submit).toHaveBeenCalledTimes(2);
    expect(error.attempts).toBe(2);
  });

  // The property the whole retry design exists for, stated once over every attempt this runner makes.
  it('never changes the signature, however many attempts it makes', async () => {
    const { submit, transport } = scriptedTransport([rejectedWith({ kind: 'unanswered' })]);
    const { clock } = recordingClock();
    const permit = signedPermit();

    await runErrorOf(() => runSolanaUserDecrypt({ signedPermit: permit, entries: ENTRIES, transport, clock }));

    const signatures = new Set(submit.mock.calls.map((call) => (call[0] as { signature: string }).signature));
    expect(signatures).toEqual(new Set([bytesToHex(permit.signature)]));
  });
});

describe('a request refused before the network', () => {
  // The stateless limits — the handle count, the bit budget, the host chain, the identity widths —
  // are properties of the request itself. Refusing here costs no submission.
  it('is refused on the handle cap without submitting', async () => {
    const { submit, transport } = scriptedTransport([answered]);
    const { clock, delay } = recordingClock();
    const overCap = Array.from({ length: MAX_SOLANA_USER_DECRYPT_HANDLES + 1 }, () => ({
      handle: handle(),
      allowedKey: identity(0x11),
      encryptedValueAccount: identity(0xea),
    }));

    await expect(
      runSolanaUserDecrypt({ signedPermit: signedPermit(), entries: overCap, transport, clock }),
    ).rejects.toThrow(SolanaUserDecryptRequestError);

    expect(submit).not.toHaveBeenCalled();
    expect(delay).not.toHaveBeenCalled();
  });
});
