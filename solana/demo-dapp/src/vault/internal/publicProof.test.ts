import { afterEach, describe, expect, it, vi } from 'vitest';
import { address } from '@solana/kit';
import { base58 } from '@scure/base';
import { buildPublicLeafProof, bytesToHex, reconstructSolanaStoreHistory } from '@sdk-src/solana/proof.js';
import { publicProof } from './publicProof.js';

const fetchState = vi.hoisted(() => vi.fn());
vi.mock('@sdk-src/solana/encryptedStore.js', () => ({ fetchSolanaEncryptedStore: fetchState }));
afterEach(() => {
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

function fixture() {
  const state = address(base58.encode(new Uint8Array(32).fill(7)));
  const handle = new Uint8Array(32).fill(8);
  const key = new Uint8Array(32).fill(9);
  const events = [
    { kind: 'allowed' as const, handle: new Uint8Array(32).fill(3), key },
    { kind: 'allowed' as const, handle, key },
    { kind: 'markedPublic' as const, handle },
    { kind: 'allowed' as const, handle: new Uint8Array(32).fill(4), key },
  ];
  const live = reconstructSolanaStoreHistory(base58.decode(state), events);
  const proof = buildPublicLeafProof(base58.decode(state), live, events, 2n);
  fetchState.mockResolvedValue(live);
  return { state, handle, proof };
}
const service = { url: 'http://listener-proof-endpoint', apiKey: 'test' };

describe('publicProof', () => {
  it('verifies a burn leaf among balance updates in a shared history', async () => {
    const { state, handle, proof } = fixture();
    const request = vi.fn().mockResolvedValue(
      new Response(
        JSON.stringify({
          proofs: [
            {
              status: 'found',
              leafIndex: Number(proof.leafIndex),
              siblings: proof.siblings.map(bytesToHex),
            },
          ],
        }),
      ),
    );
    vi.stubGlobal('fetch', request);
    expect(await publicProof({} as never, service, state, handle)).toEqual(proof);
    expect(JSON.parse(request.mock.calls[0]![1].body).leaves[0]).toEqual({
      encryptedStore: bytesToHex(base58.decode(state)),
      handle: bytesToHex(handle),
      kind: 'public',
    });
  });

  it('rejects a proof for another handle rather than forwarding it to settlement', async () => {
    const { state, handle, proof } = fixture();
    const invalidSiblings = proof.siblings.map((s, index) => (index === 0 ? new Uint8Array(32).fill(99) : s));
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(
          JSON.stringify({
            proofs: [
              {
                status: 'found',
                leafIndex: Number(proof.leafIndex),
                siblings: invalidSiblings.map(bytesToHex),
              },
            ],
          }),
        ),
      ),
    );
    vi.spyOn(Date, 'now').mockReturnValueOnce(0).mockReturnValue(15_001);
    await expect(publicProof({} as never, service, state, handle)).rejects.toThrow('did not catch up');
  });

  it('fails immediately on incomplete retained history', async () => {
    const { state, handle } = fixture();
    const request = vi
      .fn()
      .mockResolvedValue(new Response(JSON.stringify({ proofs: [{ status: 'historyIncomplete' }] })));
    vi.stubGlobal('fetch', request);
    await expect(publicProof({} as never, service, state, handle)).rejects.toThrow('historyIncomplete');
    expect(request).toHaveBeenCalledTimes(1);
  });
});
