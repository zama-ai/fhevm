import { describe, it, expect, afterEach, vi } from 'vitest';
import { RelayerHttpRequest } from './RelayerHttpRequest.js';
import { RelayerHttpFetchError } from '../../../errors/RelayerHttpFetchError.js';
import { RelayerHttpResponseError } from '../../../errors/RelayerHttpResponseError.js';
import { RelayerHttpUnexpectedResponseError } from '../../../errors/RelayerHttpUnexpectedResponseError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/RelayerHttpRequest.errors.test.ts
////////////////////////////////////////////////////////////////////////////////

const RELAYER_URL = 'https://relayer-http.example.com/v4/user-decrypt';
const VALID_REQUEST_ID = '018f4d2e-7c3a-7b1e-8c2d-abcdef123456';

describe('RelayerHttpRequest error responses', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
  });

  it('throws RelayerHttpResponseError for a valid relayer-http error body, keeping its fields', async () => {
    const body = { code: 'acl_denied', message: 'not allowed', requestId: VALID_REQUEST_ID };
    global.fetch = vi.fn().mockResolvedValue(new Response(JSON.stringify(body), { status: 403 }));

    const request = new RelayerHttpRequest({ url: RELAYER_URL, payload: {} });
    const error = await request.run().catch((e: unknown) => e);

    expect(error).toBeInstanceOf(RelayerHttpResponseError);
    const responseError = error as RelayerHttpResponseError;
    expect(responseError.url).toBe(RELAYER_URL);
    expect(responseError.status).toBe(403);
    expect(responseError.code).toBe('acl_denied');
    expect(responseError.responseMessage).toBe('not allowed');
    expect(responseError.requestId).toBe(VALID_REQUEST_ID);
  });

  // Any body that misses part of relayer-http's own { code, message, requestId }
  // contract did not come from relayer-http itself.
  it.each([
    ['code missing', { message: 'not allowed', requestId: VALID_REQUEST_ID }],
    ['code empty', { code: '', message: 'not allowed', requestId: VALID_REQUEST_ID }],
    ['message missing', { code: 'acl_denied', requestId: VALID_REQUEST_ID }],
    ['requestId missing', { code: 'acl_denied', message: 'not allowed' }],
    ['requestId not a UUIDv7', { code: 'acl_denied', message: 'not allowed', requestId: 'req-123' }],
    ['requestId a UUIDv4', { code: 'acl_denied', message: 'not allowed', requestId: '9b2c1f4e-3a5d-4e6f-8a7b-1c2d3e4f5a6b' }],
  ])('throws RelayerHttpUnexpectedResponseError for an error body with %s', async (_label, body) => {
    const text = JSON.stringify(body);
    global.fetch = vi.fn().mockResolvedValue(new Response(text, { status: 403 }));

    const request = new RelayerHttpRequest({ url: RELAYER_URL, payload: {} });
    const error = await request.run().catch((e: unknown) => e);

    expect(error).toBeInstanceOf(RelayerHttpUnexpectedResponseError);
    const unexpectedError = error as RelayerHttpUnexpectedResponseError;
    expect(unexpectedError.status).toBe(403);
    expect(unexpectedError.body).toBe(text);
  });

  // Bug: JSON.parse('null') returns null, and the non-2xx branch reads
  // bodyJson.code on it, throwing a raw TypeError instead of a typed error.
  it('throws RelayerHttpUnexpectedResponseError for a non-2xx response whose body is JSON null', async () => {
    global.fetch = vi.fn().mockResolvedValue(new Response('null', { status: 502 }));

    const request = new RelayerHttpRequest({ url: RELAYER_URL, payload: {} });

    await expect(request.run()).rejects.toBeInstanceOf(RelayerHttpUnexpectedResponseError);
  });

  it('throws RelayerHttpFetchError keeping the status and cause when reading the body fails', async () => {
    const streamError = new TypeError('terminated');
    const body = new ReadableStream<Uint8Array>({
      pull(controller) {
        controller.error(streamError);
      },
    });
    global.fetch = vi.fn().mockResolvedValue(new Response(body, { status: 200 }));

    const request = new RelayerHttpRequest({ url: RELAYER_URL, payload: {} });
    const error = await request.run().catch((e: unknown) => e);

    expect(error).toBeInstanceOf(RelayerHttpFetchError);
    expect((error as RelayerHttpFetchError).status).toBe(200);
    expect((error as Error).cause).toBe(streamError);
  });
});
