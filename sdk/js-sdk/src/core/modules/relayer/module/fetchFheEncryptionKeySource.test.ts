import type { RelayerClient } from '../types.js';
import { describe, it, expect, afterEach, vi } from 'vitest';
import { fetchFheEncryptionKeySource } from './fetchFheEncryptionKeySource.js';
import { RelayerFetchError } from '../../../errors/RelayerFetchError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/fetchFheEncryptionKeySource.test.ts
////////////////////////////////////////////////////////////////////////////////

const relayerClient: RelayerClient = {
  relayerUrl: 'https://relayer.example.com',
  chainId: 11155111,
};

// Resolve a fresh Response on every fetch call (a Response body reads once).
function mockFetchStatus(status: number, body: string): void {
  global.fetch = vi.fn().mockImplementation(() => Promise.resolve(new Response(body, { status })));
}

describe('fetchFheEncryptionKeySource error surfacing', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
  });

  it('401 surfaces the message from { error: { message } }', async () => {
    mockFetchStatus(401, JSON.stringify({ error: { message: 'invalid x-api-key', label: 'unauthorized' } }));

    try {
      await fetchFheEncryptionKeySource(relayerClient, {});
      expect.unreachable();
    } catch (e) {
      expect(e).toBeInstanceOf(RelayerFetchError);
      expect((e as RelayerFetchError).message).toContain('401');
      expect((e as RelayerFetchError).message).toContain('invalid x-api-key');
    }
  });

  it('403 surfaces the message from a flat Cloudflare/Kong body', async () => {
    mockFetchStatus(403, JSON.stringify({ message: 'Missing or invalid Zama API Key', label: 'unauthorized' }));

    try {
      await fetchFheEncryptionKeySource(relayerClient, {});
      expect.unreachable();
    } catch (e) {
      expect(e).toBeInstanceOf(RelayerFetchError);
      expect((e as RelayerFetchError).message).toContain('Missing or invalid Zama API Key');
    }
  });

  it('403 with a non-JSON body keeps the generic HTTP-error message', async () => {
    mockFetchStatus(403, 'Forbidden');

    try {
      await fetchFheEncryptionKeySource(relayerClient, {});
      expect.unreachable();
    } catch (e) {
      expect(e).toBeInstanceOf(RelayerFetchError);
      const message = (e as RelayerFetchError).message;
      expect(message).toContain('HTTP error! status: 403');
      expect(message).not.toContain('Forbidden');
    }
  });
});
