import { afterEach, describe, expect, it, vi } from 'vitest';
import { address, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import { fetchSolanaPublicDecryptProof } from './proofService.js';
import { decodeMmrProofTransportBlob, MMR_MODE_PUBLIC } from '../../proof.js';

const LINEAGE: Address = address(base58.encode(new Uint8Array(32).fill(7)));
const HANDLE = new Uint8Array(32).fill(0x92);
// bytesToHex emits a 0x-prefixed string; the service strips the prefix before hex-decoding.
const HANDLE_HEX = `0x${Array.from(HANDLE, (b) => b.toString(16).padStart(2, '0')).join('')}`;

function jsonResponse(status: number, body: unknown): Response {
  return { ok: status >= 200 && status < 300, status, json: async () => body } as unknown as Response;
}

afterEach(() => vi.restoreAllMocks());

describe('fetchSolanaPublicDecryptProof', () => {
  it('parses a verified single-leaf proof and builds the 0x02 transport blob', async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      jsonResponse(200, {
        mmr_proof: { leaf_index: 0, siblings: [] },
        leaf_index: 0,
        leaf_count: 1,
        rpc_context_slot: 1234,
        commitment: 'confirmed',
        proof_format_version: 'v1',
        verified: true,
        status: 'verified',
      }),
    );
    vi.stubGlobal('fetch', fetchMock);

    const result = await fetchSolanaPublicDecryptProof({ proofServiceUrl: 'http://proof:8080/' }, LINEAGE, HANDLE);
    // leafIndex is a service output the SDK reads back, never supplied.
    expect(result.proof).toEqual({ leafIndex: 0n, siblings: [] });
    expect(result.leafCount).toBe(1n);
    // The transport blob round-trips through the SDK decoder as a public-decrypt proof.
    const decoded = decodeMmrProofTransportBlob(result.mmrProofBytes);
    expect(decoded.mode).toBe(MMR_MODE_PUBLIC);
    expect(decoded.proof).toEqual({ leafIndex: 0n, siblings: [] });

    // The request keys on (encrypted_value, handle) — no leaf_index anywhere.
    const url = fetchMock.mock.calls[0]![0] as string;
    expect(url).toBe(`http://proof:8080/internal/solana/public-proof?encrypted_value=${LINEAGE}&handle=${HANDLE_HEX}`);
    expect(url).not.toContain('leaf_index');
  });

  it('reads back a non-zero resolved leaf index and decodes hex siblings', async () => {
    const sibling = 'ab'.repeat(32);
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        jsonResponse(200, {
          mmr_proof: { leaf_index: 2, siblings: [sibling] },
          leaf_index: 2,
          leaf_count: 4,
          verified: true,
          status: 'verified',
        }),
      ),
    );
    const result = await fetchSolanaPublicDecryptProof({ proofServiceUrl: 'http://proof:8080' }, LINEAGE, HANDLE);
    expect(result.proof.leafIndex).toBe(2n);
    expect(result.leafCount).toBe(4n);
    expect(result.proof.siblings).toHaveLength(1);
    expect(Array.from(result.proof.siblings[0]!)).toEqual(Array.from(new Uint8Array(32).fill(0xab)));
  });

  it('retries on a 503 lagging body, then succeeds', async () => {
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce(
        jsonResponse(503, { mmr_proof: null, leaf_count: 0, rpc_context_slot: 0, verified: false, status: 'lagging' }),
      )
      .mockResolvedValueOnce(
        jsonResponse(200, {
          mmr_proof: { leaf_index: 0, siblings: [] },
          leaf_index: 0,
          leaf_count: 1,
          verified: true,
          status: 'verified',
        }),
      );
    vi.stubGlobal('fetch', fetchMock);

    const result = await fetchSolanaPublicDecryptProof(
      { proofServiceUrl: 'http://proof:8080', retryDelayMs: 0, maxRetries: 3 },
      LINEAGE,
      HANDLE,
    );
    expect(result.leafCount).toBe(1n);
    expect(fetchMock).toHaveBeenCalledTimes(2);
  });

  it('treats a 404 leaf_not_found as terminal (no retry)', async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      jsonResponse(404, {
        mmr_proof: null,
        leaf_count: 1,
        rpc_context_slot: 1,
        verified: false,
        status: 'leaf_not_found',
      }),
    );
    vi.stubGlobal('fetch', fetchMock);
    await expect(
      fetchSolanaPublicDecryptProof({ proofServiceUrl: 'http://proof:8080', retryDelayMs: 0 }, LINEAGE, HANDLE),
    ).rejects.toThrow('proof-service request failed');
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });

  it('treats a corrupt-cache integrity failure as terminal (no retry)', async () => {
    const fetchMock = vi
      .fn()
      .mockResolvedValue(
        jsonResponse(500, { mmr_proof: null, leaf_count: 1, verified: false, status: 'corrupt_cache' }),
      );
    vi.stubGlobal('fetch', fetchMock);
    await expect(
      fetchSolanaPublicDecryptProof({ proofServiceUrl: 'http://proof:8080', retryDelayMs: 0 }, LINEAGE, HANDLE),
    ).rejects.toThrow('proof-service request failed');
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });

  it('rejects an unverified 200 body', async () => {
    vi.stubGlobal(
      'fetch',
      vi
        .fn()
        .mockResolvedValue(jsonResponse(200, { mmr_proof: null, leaf_count: 1, verified: false, status: 'verified' })),
    );
    await expect(
      fetchSolanaPublicDecryptProof({ proofServiceUrl: 'http://proof:8080' }, LINEAGE, HANDLE),
    ).rejects.toThrow('unverified proof');
  });
});
