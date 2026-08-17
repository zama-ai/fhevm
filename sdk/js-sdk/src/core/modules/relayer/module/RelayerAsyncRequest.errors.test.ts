import { describe, it, expect, afterEach, vi } from 'vitest';
import { RelayerAsyncRequest } from './RelayerAsyncRequest.js';
import { RelayerResponseApiError } from '../../../errors/RelayerResponseApiError.js';
import { RelayerResponseStatusError } from '../../../errors/RelayerResponseStatusError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/RelayerAsyncRequest.errors.test.ts
////////////////////////////////////////////////////////////////////////////////

const RELAYER_URL = 'https://relayer.example.com/v2/public-decrypt';

function newRequest(): RelayerAsyncRequest {
  return new RelayerAsyncRequest({
    relayerOperation: 'PUBLIC_DECRYPT',
    url: RELAYER_URL,
    payload: {},
    options: { fetchRetries: 1 },
  });
}

// Resolve a fresh Response on every fetch call: a Response body can only be
// read once, so reusing one instance across calls would fail.
function mockFetchStatus(status: number, body: string): void {
  global.fetch = vi.fn().mockImplementation(() => Promise.resolve(new Response(body, { status })));
}

describe('RelayerAsyncRequest auth/edge error surfacing', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
  });

  //////////////////////////////////////////////////////////////////////////////
  // 401
  //////////////////////////////////////////////////////////////////////////////

  it('401 surfaces the message from { error: { message } }', async () => {
    mockFetchStatus(
      401,
      JSON.stringify({ error: { message: 'origin says: invalid x-api-key', label: 'unauthorized' } }),
    );

    try {
      await newRequest().run();
      expect.unreachable();
    } catch (e) {
      expect(e).toBeInstanceOf(RelayerResponseApiError);
      const err = e as RelayerResponseApiError;
      expect(err.status).toBe(401);
      expect(err.relayerApiError.label).toBe('unauthorized');
      expect(err.relayerApiError.message).toBe('origin says: invalid x-api-key');
      expect(err.message).toContain('origin says: invalid x-api-key');
    }
  });

  it('401 with an empty body falls back to the canned message', async () => {
    mockFetchStatus(401, '');

    try {
      await newRequest().run();
      expect.unreachable();
    } catch (e) {
      expect(e).toBeInstanceOf(RelayerResponseApiError);
      const err = e as RelayerResponseApiError;
      expect(err.relayerApiError.label).toBe('unauthorized');
      expect(err.relayerApiError.message).toBe('Unauthorized, missing or invalid Zama Fhevm API Key.');
    }
  });

  //////////////////////////////////////////////////////////////////////////////
  // 403 (edge block — not part of the typed contract)
  //////////////////////////////////////////////////////////////////////////////

  it('403 surfaces the message from a flat Cloudflare/Kong body', async () => {
    mockFetchStatus(403, JSON.stringify({ message: 'Missing or invalid Zama API Key', label: 'unauthorized' }));

    try {
      await newRequest().run();
      expect.unreachable();
    } catch (e) {
      expect(e).toBeInstanceOf(RelayerResponseStatusError);
      const err = e as RelayerResponseStatusError;
      expect(err.status).toBe(403);
      expect(err.message).toContain('Missing or invalid Zama API Key');
    }
  });

  it('403 with a non-JSON body keeps the generic unexpected-status message', async () => {
    mockFetchStatus(403, 'Forbidden');

    try {
      await newRequest().run();
      expect.unreachable();
    } catch (e) {
      expect(e).toBeInstanceOf(RelayerResponseStatusError);
      const err = e as RelayerResponseStatusError;
      expect(err.status).toBe(403);
      expect(err.message).toContain('unexpected response status: 403');
      expect(err.message).not.toContain('Forbidden');
    }
  });
});
