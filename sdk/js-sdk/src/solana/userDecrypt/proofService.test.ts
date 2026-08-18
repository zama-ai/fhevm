// The access-proof client, pinned at the service's wire.
//
// Three things hold the seam: the query (base58 account and subject, bare-hex handle, no leaf
// index — the leaf is the service's answer, never the client's claim), the verified gate (only a
// proof the service checked against live peaks is accepted), and the retry rule (only `lagging`
// retries, bounded — every other answer is terminal, because retrying an integrity failure buries
// it under a timeout).

import { afterEach, describe, expect, it, vi } from 'vitest';
import { getAddressDecoder } from '@solana/kit';
import { decodeMmrProof } from '../proof.js';
import { fetchSolanaHistoricalAccessProof } from './index.js';

////////////////////////////////////////////////////////////////////////////////
// A scripted service
////////////////////////////////////////////////////////////////////////////////

// The account's own pubkey — the PDA the service indexes by, not the wire identity.
const ENCRYPTED_VALUE_ACCOUNT = new Uint8Array(32).fill(0xea);
const HANDLE = new Uint8Array(32).fill(0xa1);
const SUBJECT = new Uint8Array(32).fill(0xc1);
const base58 = (bytes: Uint8Array): string => getAddressDecoder().decode(bytes);

const SIBLING_HEX = '11'.repeat(32);

const verified = {
  mmr_proof: { leaf_index: 3, siblings: [SIBLING_HEX] },
  leaf_count: 8,
  rpc_context_slot: 100,
  verified: true,
  status: 'verified',
};

const lagging = { mmr_proof: null, leaf_count: 0, verified: false, status: 'lagging' };

function scriptedService(script: readonly { status: number; body: unknown }[]) {
  const urls: string[] = [];
  vi.stubGlobal(
    'fetch',
    vi.fn((input: string | URL | Request) => {
      const step = script[Math.min(urls.length, script.length - 1)];
      urls.push(String(input));
      if (step === undefined) {
        throw new Error('the service script is empty');
      }
      return Promise.resolve(new Response(JSON.stringify(step.body), { status: step.status }));
    }),
  );
  return { urls };
}

function fetched(config?: { laggingRetries?: number; laggingDelayMs?: number }) {
  return fetchSolanaHistoricalAccessProof(
    { proofServiceUrl: 'http://proofs.local/', ...config },
    { encryptedValueAccount: ENCRYPTED_VALUE_ACCOUNT, handle: HANDLE, subject: SUBJECT },
  );
}

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
});

////////////////////////////////////////////////////////////////////////////////

describe('the access-proof client', () => {
  it('asks by account, handle and subject — never by leaf index — and returns the bare borsh proof', async () => {
    const { urls } = scriptedService([{ status: 200, body: verified }]);

    const answer = await fetched();

    expect(urls[0]).toBe(
      `http://proofs.local/internal/solana/access-proof?encrypted_value=${base58(ENCRYPTED_VALUE_ACCOUNT)}` +
        `&handle=${'a1'.repeat(32)}&subject=${base58(SUBJECT)}`,
    );
    expect(answer.leafCount).toBe(8n);
    expect(answer.proof).toEqual({ leafIndex: 3n, siblings: [new Uint8Array(32).fill(0x11)] });
    // The wire form of a request entry: bare borsh, no mode byte in front of it.
    expect(decodeMmrProof(answer.accessProof)).toEqual(answer.proof);
  });

  // `verified: true` is the service's own live peak comparison; a proof without it proves nothing
  // this client should hand to a builder.
  it('refuses an answer the service itself did not verify', async () => {
    scriptedService([{ status: 200, body: { ...verified, verified: false, status: 'unverified' } }]);
    await expect(fetched()).rejects.toThrow('unverified');
  });

  it('waits out a lagging store, bounded, and picks up the answer when it lands', async () => {
    vi.useFakeTimers();
    const { urls } = scriptedService([
      { status: 503, body: lagging },
      { status: 503, body: lagging },
      { status: 200, body: verified },
    ]);

    const pending = fetched();
    await vi.advanceTimersByTimeAsync(10_000);
    const answer = await pending;

    expect(urls).toHaveLength(3);
    expect(answer.leafCount).toBe(8n);
  });

  it('gives up on a store that lags past the budget', async () => {
    vi.useFakeTimers();
    scriptedService([{ status: 503, body: lagging }]);

    const pending = fetched({ laggingRetries: 1 });
    const failure = expect(pending).rejects.toThrow('lagging');
    await vi.advanceTimersByTimeAsync(10_000);
    await failure;
  });

  // Anything that is not `lagging` is terminal on the spot: a missing leaf or a corrupt cache
  // retried into a timeout would be reported as slowness instead of what it is.
  it('fails a not-found answer immediately, without retrying', async () => {
    const { urls } = scriptedService([{ status: 404, body: { error: 'leaf not found', code: 'leaf_not_found' } }]);

    await expect(fetched()).rejects.toThrow('404');
    expect(urls).toHaveLength(1);
  });

  it('refuses a sibling that is not 32 bytes', async () => {
    scriptedService([{ status: 200, body: { ...verified, mmr_proof: { leaf_index: 3, siblings: ['1111'] } } }]);
    await expect(fetched()).rejects.toThrow('32');
  });
});
