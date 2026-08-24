// The relayer transport, pinned at the wire.
//
// The core async request owns the POST/GET job loop, so what these tests pin is the seam: which URL
// is posted, that the body goes through verbatim, and how each wire outcome comes back through the
// port — shares as shares, a refusal and a failed job as their labels, a completed job with nothing
// in it as `unanswered`, and everything that is not a relayer answer as a thrown error. A 429 never
// reaches the port at all: the core loop waits it out at the wire with the server's own delay.

import type { SolanaUserDecryptRequestJson, SolanaUserDecryptTransportOutcome } from './index.js';
import type { SolanaSigncryptedShare } from './index.js';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { createSolanaUserDecryptRelayerTransport } from './index.js';

////////////////////////////////////////////////////////////////////////////////
// A scripted wire
////////////////////////////////////////////////////////////////////////////////

/** The request body, as the builder would emit it. The transport must not look inside. */
const REQUEST = {
  attestationType: 'solana-srfc38-user-decrypt-v1',
  attestedPayload: { handles: [] },
  signature: '0x77',
} as unknown as SolanaUserDecryptRequestJson;

const SHARE = {
  payload: 'deadbeef',
  signature: 'ab'.repeat(65),
  extraData: '0x01',
};

const queued = { status: 'queued', requestId: 'req-1', result: { jobId: 'job-1' } };
const succeeded = (shares: readonly unknown[]) => ({
  status: 'succeeded',
  requestId: 'req-1',
  result: { result: shares },
});
// `validation_failed` travels with a `details` array on the wire; the core guard requires it.
const failed = (label: string, details?: readonly unknown[]) => ({
  status: 'failed',
  requestId: 'req-1',
  error: { label, message: 'because', ...(details === undefined ? {} : { details }) },
});

/** Answers fetches from a script; the last entry answers every fetch after it. */
function scriptedWire(script: readonly { status: number; body: unknown }[]) {
  const calls: { url: string; method: string; body: unknown }[] = [];
  const fetchMock = vi.fn((input: string | URL | Request, init?: RequestInit) => {
    const step = script[Math.min(calls.length, script.length - 1)];
    calls.push({
      url: String(input),
      method: init?.method ?? 'GET',
      body: typeof init?.body === 'string' ? JSON.parse(init.body) : undefined,
    });
    if (step === undefined) {
      throw new Error('the wire script is empty');
    }
    return Promise.resolve(
      new Response(JSON.stringify(step.body), {
        status: step.status,
        headers: { 'Content-Type': 'application/json', 'Retry-After': '0' },
      }),
    );
  });
  vi.stubGlobal('fetch', fetchMock);
  return { calls };
}

/** Runs one submission under fake timers, driving every internal wait to completion. */
async function submitted(script: readonly { status: number; body: unknown }[]): Promise<{
  outcome: SolanaUserDecryptTransportOutcome<readonly SolanaSigncryptedShare[]>;
  calls: readonly { url: string; method: string; body: unknown }[];
}> {
  vi.useFakeTimers();
  const { calls } = scriptedWire(script);
  const transport = createSolanaUserDecryptRelayerTransport({ relayerUrl: 'http://relayer.local' });

  const pending = transport.submit(REQUEST);
  // Enough simulated time for every Retry-After floor the core loop inserts between fetches.
  await vi.advanceTimersByTimeAsync(60_000);
  const outcome = await pending;
  return { outcome, calls };
}

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
});

////////////////////////////////////////////////////////////////////////////////

describe('the relayer transport', () => {
  it('posts the body verbatim to v3/user-decrypt, polls the job, and returns its shares', async () => {
    const { outcome, calls } = await submitted([
      { status: 202, body: queued },
      { status: 200, body: succeeded([SHARE]) },
    ]);

    expect(calls[0]?.method).toBe('POST');
    expect(calls[0]?.url).toBe('http://relayer.local/v3/user-decrypt');
    expect(calls[0]?.body).toEqual(REQUEST);
    expect(calls[1]?.method).toBe('GET');
    expect(calls[1]?.url).toBe('http://relayer.local/v3/user-decrypt/job-1');

    expect(outcome).toEqual({
      ok: true,
      response: [{ payload: SHARE.payload, signature: SHARE.signature, extraData: SHARE.extraData }],
    });
  });

  it('maps a refusal before the job exists to refused, with the relayer label', async () => {
    const { outcome } = await submitted([{ status: 400, body: failed('validation_failed', []) }]);
    expect(outcome).toEqual({
      ok: false,
      rejection: { kind: 'refused', label: 'validation_failed', message: 'because' },
    });
  });

  it('maps a failure of an accepted job to failed, with the relayer label', async () => {
    const { outcome } = await submitted([
      { status: 202, body: queued },
      { status: 500, body: failed('host_acl_failed') },
    ]);
    expect(outcome).toEqual({
      ok: false,
      rejection: { kind: 'failed', label: 'host_acl_failed', message: 'because' },
    });
  });

  // A completed job carrying nothing is not an answer to verify: it is the Connector refusing
  // without a wire class, and the session repairs it by resolving the evidence again.
  it('answers unanswered when the job completes with no shares', async () => {
    const { outcome } = await submitted([
      { status: 202, body: queued },
      { status: 200, body: succeeded([]) },
    ]);
    expect(outcome).toEqual({ ok: false, rejection: { kind: 'unanswered' } });
  });

  // A status outside the relayer's contract (an edge proxy speaking for it) is not a rejection the
  // session can classify — it is infrastructure, and it surfaces as the error it is.
  it('rethrows what is not a relayer answer at all', async () => {
    vi.useFakeTimers();
    scriptedWire([{ status: 403, body: { message: 'forbidden' } }]);
    const transport = createSolanaUserDecryptRelayerTransport({ relayerUrl: 'http://relayer.local' });

    await expect(transport.submit(REQUEST)).rejects.toThrow();
  });
});
