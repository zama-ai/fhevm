import type { FetchPadSharesParameters, RelayerClientWithRuntime } from '../types.js';
import type {
  Bytes32Hex,
  BytesHex,
  ChecksummedAddress,
  Uint16Number,
  Uint64BigInt,
  Uint8Number,
} from '../../../types/primitives.js';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { fetchPadShares } from './fetchPadShares.js';
import { RelayerAbortError } from '../../../errors/RelayerAbortError.js';
import { RelayerFetchError } from '../../../errors/RelayerFetchError.js';
import { RelayerResponseInvalidBodyError } from '../../../errors/RelayerResponseInvalidBodyError.js';
import { RelayerResponseStatusError } from '../../../errors/RelayerResponseStatusError.js';
import { RelayerTimeoutError } from '../../../errors/RelayerTimeoutError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/fetchPadShares.test.ts
////////////////////////////////////////////////////////////////////////////////

const relayerClient = {
  chain: { fhevm: { relayerUrl: 'https://relayer.example.com' } },
} as unknown as RelayerClientWithRuntime;

const USER = '0x5555555555555555555555555555555555555555' as ChecksummedAddress;
const APP = '0x3333333333333333333333333333333333333333' as ChecksummedAddress;
const LABEL = `0x${'88'.repeat(32)}` as Bytes32Hex;

const parameters: FetchPadSharesParameters = {
  payload: {
    kmsDecryptEip712Message: {
      userAddress: USER,
      publicKey: '0x20002000' as BytesHex,
      allowedContracts: [APP],
      startTimestamp: '1799999900',
      durationSeconds: '600',
      extraData: `0x02${'00'.repeat(64)}` as BytesHex,
    },
    kmsDecryptEip712Signature: `0x${'66'.repeat(65)}` as BytesHex,
    witnessVersion: 0 as Uint8Number,
    prfId: 1 as Uint16Number,
    chainId: 31337n as Uint64BigInt,
    pads: [
      {
        label: LABEL,
        witness: {
          submitter: USER,
          consumer: APP,
          expirationTimestamp: 1800604800n as Uint64BigInt,
          decryptors: [],
          nonce: `0x${'77'.repeat(16)}` as BytesHex,
        },
        padLength: 64 as Uint16Number,
      },
    ],
  },
  fhevmContext: {} as never,
};

const NODE_RESPONSE = {
  decryptionId: `0x${'99'.repeat(32)}`,
  encryptedPayload: '0xabcdef',
  signature: `0x${'1b'.repeat(65)}`,
};

/** `NODE_RESPONSE` as `fetchPadShares` returns it: the connector's `decryptionId` is the KMS `requestId`. */
const PAD_RESPONSE = {
  requestId: NODE_RESPONSE.decryptionId,
  encryptedPayload: NODE_RESPONSE.encryptedPayload,
  signature: NODE_RESPONSE.signature,
};

/** relayer-http's request ids are UUIDv4. */
const REQUEST_ID = '018f4d2e-7c3a-4b1e-8c2d-abcdef123456';

const originalFetch = global.fetch;

function mockFetch(status: number, body: unknown): void {
  global.fetch = vi
    .fn()
    .mockImplementation(() =>
      Promise.resolve(new Response(typeof body === 'string' ? body : JSON.stringify(body), { status })),
    );
}

afterEach(() => {
  global.fetch = originalFetch;
  vi.restoreAllMocks();
  vi.useRealTimers();
});

describe('fetchPadShares request', () => {
  it('posts the wire body to v4/pad-decrypt and returns every node response as is', async () => {
    mockFetch(200, {
      status: 'succeeded',
      requestId: REQUEST_ID,
      result: { responses: [NODE_RESPONSE, NODE_RESPONSE] },
    });

    const responses = await fetchPadShares(relayerClient, {
      ...parameters,
      options: { query: { zama_sdk_version: '1.2.3' }, headers: { 'x-extra': 'yes' } },
    });

    expect(responses).toEqual([PAD_RESPONSE, PAD_RESPONSE]);
    expect(Object.isFrozen(responses)).toBe(true);

    const [url, init] = vi.mocked(global.fetch).mock.calls[0]!;
    expect(url).toBe('https://relayer.example.com/v4/pad-decrypt?zama_sdk_version=1.2.3');
    expect(init?.method).toBe('POST');
    expect((init?.headers as Record<string, string>)['content-type']).toBe('application/json');
    expect((init?.headers as Record<string, string>)['x-extra']).toBe('yes');
    expect(JSON.parse(init?.body as string)).toEqual({
      attestationType: 'eip712-unified-user-decrypt-v1',
      payload: {
        witnessVersion: 0,
        prfId: 1,
        chainId: '31337',
        pads: [
          {
            label: LABEL,
            witness: {
              submitter: USER,
              consumer: APP,
              expirationTimestamp: '1800604800',
              decryptors: [],
              nonce: `0x${'77'.repeat(16)}`,
            },
            padLength: 64,
          },
        ],
        userAddress: USER,
        publicKey: '0x20002000',
        allowedContracts: [APP],
        requestValidity: { startTimestamp: '1799999900', durationSeconds: '600' },
        extraData: `0x02${'00'.repeat(64)}`,
      },
      signature: `0x${'66'.repeat(65)}`,
    });
  });
});

describe('fetchPadShares errors', () => {
  it('a relayer-http error carries its status, code, message and request id', async () => {
    mockFetch(400, {
      code: 'malformed',
      message: 'payload.pads[0].label: is not the hash of the witness',
      requestId: 'r-2',
    });

    const error = await fetchPadShares(relayerClient, parameters).catch((e: unknown) => e);
    expect(error).toBeInstanceOf(RelayerResponseStatusError);
    expect((error as RelayerResponseStatusError).status).toBe(400);
    expect((error as RelayerResponseStatusError).operation).toBe('PAD_DECRYPT');
    expect((error as Error).message).toContain('Pad decryption');
    expect((error as RelayerResponseStatusError).details).toBe(
      '[malformed] payload.pads[0].label: is not the hash of the witness (requestId r-2)',
    );
  });

  it('a non-JSON error body still reports the status', async () => {
    mockFetch(502, '<html>Bad Gateway</html>');
    const error = await fetchPadShares(relayerClient, parameters).catch((e: unknown) => e);
    expect(error).toBeInstanceOf(RelayerResponseStatusError);
    expect((error as RelayerResponseStatusError).status).toBe(502);
  });

  it('a 200 that does not match the schema is an invalid body', async () => {
    mockFetch(200, {
      status: 'succeeded',
      requestId: REQUEST_ID,
      result: { responses: [{ ...NODE_RESPONSE, signature: '0x1234' }] },
    });
    await expect(fetchPadShares(relayerClient, parameters)).rejects.toThrow(RelayerResponseInvalidBodyError);

    // The envelope's id must be a UUIDv4.
    mockFetch(200, { status: 'succeeded', requestId: 'r-3', result: { responses: [NODE_RESPONSE] } });
    await expect(fetchPadShares(relayerClient, parameters)).rejects.toThrow(RelayerResponseInvalidBodyError);

    // The node's id is the connector's `decryptionId`, not the KMS core's `requestId`.
    mockFetch(200, { status: 'succeeded', requestId: REQUEST_ID, result: { responses: [PAD_RESPONSE] } });
    await expect(fetchPadShares(relayerClient, parameters)).rejects.toThrow(RelayerResponseInvalidBodyError);

    mockFetch(200, 'not json');
    await expect(fetchPadShares(relayerClient, parameters)).rejects.toThrow(RelayerFetchError);
  });

  it('a network failure is a fetch error', async () => {
    global.fetch = vi.fn().mockRejectedValue(new TypeError('fetch failed'));
    await expect(fetchPadShares(relayerClient, parameters)).rejects.toThrow(RelayerFetchError);
  });

  it('options.timeout bounds the call, options.signal aborts it', async () => {
    // A fetch that only ends when its signal aborts.
    global.fetch = vi.fn().mockImplementation(
      (_url: string, init: RequestInit) =>
        new Promise((_resolve, reject) => {
          init.signal?.addEventListener('abort', () => {
            reject(new DOMException('aborted', 'AbortError'));
          });
        }),
    );
    await expect(fetchPadShares(relayerClient, { ...parameters, options: { timeout: 10 } })).rejects.toThrow(
      RelayerTimeoutError,
    );

    const controller = new AbortController();
    const pending = fetchPadShares(relayerClient, { ...parameters, options: { signal: controller.signal } });
    controller.abort();
    await expect(pending).rejects.toThrow(RelayerAbortError);

    await expect(
      fetchPadShares(relayerClient, { ...parameters, options: { signal: AbortSignal.abort() } }),
    ).rejects.toThrow(RelayerAbortError);
  });
});
