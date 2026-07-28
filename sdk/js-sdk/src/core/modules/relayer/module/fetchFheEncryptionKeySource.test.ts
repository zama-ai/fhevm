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
const originalFetch = global.fetch;

// Resolve a fresh Response on every fetch call (a Response body reads once).
function mockFetchStatus(status: number, body: string): void {
  global.fetch = vi.fn().mockImplementation(() => Promise.resolve(new Response(body, { status })));
}

afterEach(() => {
  global.fetch = originalFetch;
  vi.restoreAllMocks();
});

describe('fetchFheEncryptionKeySource error surfacing', () => {
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

describe('_assertIsFetchKeyUrlResult accepted payloads', () => {
  const expectedPublicKeySource = {
    id: '0x0400000000000000000000000000000000000000000000000000000000000001',
    url: 'https://keys.example.com/PublicKey/0400000000000000000000000000000000000000000000000000000000000001',
  };
  const expectedCrsSource = {
    id: '0x0500000000000000000000000000000000000000000000000000000000000001',
    url: 'https://keys.example.com/CRS/0500000000000000000000000000000000000000000000000000000000000001',
    capacity: 2048,
  };

  it.each([
    [
      'with contextId and epochId',
      {
        status: 'succeeded',
        response: {
          fheKeyInfo: [{ fhePublicKey: { dataId: expectedPublicKeySource.id, urls: [expectedPublicKeySource.url] } }],
          crs: {
            '2048': { dataId: expectedCrsSource.id, urls: [expectedCrsSource.url] },
          },
          contextId: '0x0700000000000000000000000000000000000000000000000000000000000001',
          epochId: '0x01',
        },
      },
    ],
    [
      'without contextId and epochId',
      {
        status: 'succeeded',
        response: {
          fheKeyInfo: [{ fhePublicKey: { dataId: expectedPublicKeySource.id, urls: [expectedPublicKeySource.url] } }],
          crs: {
            '2048': { dataId: expectedCrsSource.id, urls: [expectedCrsSource.url] },
          },
        },
      },
    ],
  ])('accepts a succeeded keyurl response %s', async (_name, payload) => {
    mockFetchStatus(200, JSON.stringify(payload));

    const source = await fetchFheEncryptionKeySource(relayerClient, {});

    expect(source.publicKeySource).toStrictEqual(expectedPublicKeySource);
    expect(source.crsSource).toStrictEqual(expectedCrsSource);
  });
});

describe('_assertIsFetchKeyUrlResult rejected payloads', () => {
  const validFheKeyInfo = [
    {
      fhePublicKey: {
        dataId: '0x0400000000000000000000000000000000000000000000000000000000000001',
        urls: ['https://keys.example.com/PublicKey/0400000000000000000000000000000000000000000000000000000000000001'],
      },
    },
  ];
  const validCrs = {
    '2048': {
      dataId: '0x0500000000000000000000000000000000000000000000000000000000000001',
      urls: ['https://keys.example.com/CRS/0500000000000000000000000000000000000000000000000000000000000001'],
    },
  };

  it.each([
    ['fheKeyInfo', { crs: validCrs }, 'missing response.fheKeyInfo'],
    ['crs', { fheKeyInfo: validFheKeyInfo }, 'missing response.crs'],
  ])('rejects a succeeded keyurl response without %s', async (_field, response, expectedMessage) => {
    mockFetchStatus(200, JSON.stringify({ status: 'succeeded', response }));

    await expect(fetchFheEncryptionKeySource(relayerClient, {})).rejects.toThrow(expectedMessage);
  });
});
