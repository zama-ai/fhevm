import { afterEach, describe, expect, it, vi } from 'vitest';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import { solanaDecryptionRelayer } from './relayerHttp.js';

describe('solanaDecryptionRelayer', () => {
  const originalFetch = global.fetch;
  afterEach(() => {
    global.fetch = originalFetch;
  });

  // A relayer that is shutting down closes its listener: the request is refused, not answered.
  it('retries a refused connection, as the EVM relayer client does', async () => {
    const fetchMock = vi
      .fn()
      .mockRejectedValueOnce(new TypeError('fetch failed', { cause: { code: 'ECONNREFUSED' } }))
      .mockResolvedValue(new Response(JSON.stringify({ code: 'acl_denied', message: 'no leaf' }), { status: 403 }));
    global.fetch = fetchMock;
    const chain = { fhevm: { decryptionRelayerUrl: 'https://decryption.relayer.example.test' } } as FhevmSolanaChain;

    const outcome = await solanaDecryptionRelayer(chain, 'PUBLIC_DECRYPT', undefined).submit({});

    expect(outcome).toEqual({ ok: false, rejection: { code: 'acl_denied', message: 'no leaf' } });
    expect(fetchMock).toHaveBeenCalledTimes(2);
  });
});
