import { describe, it, expect, afterEach, vi } from 'vitest';
import { RelayerHttpRequest } from '../RelayerHttpRequest.js';
import { RelayerHttpFetchError } from '../../../../errors/RelayerHttpFetchError.js';
import { RelayerHttpResponseError } from '../../../../errors/RelayerHttpResponseError.js';
import { RelayerHttpUnexpectedResponseError } from '../../../../errors/RelayerHttpUnexpectedResponseError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/test/RelayerHttpRequest.errors.test.ts
////////////////////////////////////////////////////////////////////////////////

const RELAYER_URL = 'https://relayer-http.example.com/v4/user-decrypt';
const VALID_REQUEST_ID = '018f4d2e-7c3a-7b1e-8c2d-abcdef123456';

describe('RelayerHttpRequest error responses', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
  });

  // A 403 with a valid relayer-http error body: RelayerHttpResponseError keeping status, code, message, requestId.
  it('throws RelayerHttpResponseError for a valid relayer-http error body, keeping its fields', async () => {
    const body = { code: 'acl_denied', message: 'not allowed', requestId: VALID_REQUEST_ID };
    global.fetch = vi.fn().mockResolvedValue(new Response(JSON.stringify(body), { status: 403 }));

    const request = new RelayerHttpRequest({ relayerOperation: 'USER_DECRYPT', url: RELAYER_URL, payload: {} });
    const error = await request.run().catch((e: unknown) => e);

    expect(error).toBeInstanceOf(RelayerHttpResponseError);
    const responseError = error as RelayerHttpResponseError;
    expect(responseError.url).toBe(RELAYER_URL);
    expect(responseError.status).toBe(403);
    expect(responseError.code).toBe('acl_denied');
    expect(responseError.responseMessage).toBe('not allowed');
    expect(responseError.requestId).toBe(VALID_REQUEST_ID);
  });

  // A 403 body breaking the { code, message, requestId } contract: RelayerHttpUnexpectedResponseError, raw body kept.
  it.each([
    ['code missing', { message: 'not allowed', requestId: VALID_REQUEST_ID }],
    ['code empty', { code: '', message: 'not allowed', requestId: VALID_REQUEST_ID }],
    ['code blank (whitespace only)', { code: '   ', message: 'not allowed', requestId: VALID_REQUEST_ID }],
    ['message missing', { code: 'acl_denied', requestId: VALID_REQUEST_ID }],
    ['message blank (whitespace only)', { code: 'acl_denied', message: '\t\n ', requestId: VALID_REQUEST_ID }],
    ['requestId missing', { code: 'acl_denied', message: 'not allowed' }],
    ['requestId not a UUIDv7', { code: 'acl_denied', message: 'not allowed', requestId: 'req-123' }],
    [
      'requestId a UUIDv4',
      { code: 'acl_denied', message: 'not allowed', requestId: '9b2c1f4e-3a5d-4e6f-8a7b-1c2d3e4f5a6b' },
    ],
  ])('throws RelayerHttpUnexpectedResponseError for an error body with %s', async (_label, body) => {
    const text = JSON.stringify(body);
    global.fetch = vi.fn().mockResolvedValue(new Response(text, { status: 403 }));

    const request = new RelayerHttpRequest({ relayerOperation: 'USER_DECRYPT', url: RELAYER_URL, payload: {} });
    const error = await request.run().catch((e: unknown) => e);

    expect(error).toBeInstanceOf(RelayerHttpUnexpectedResponseError);
    const unexpectedError = error as RelayerHttpUnexpectedResponseError;
    expect(unexpectedError.status).toBe(403);
    expect(unexpectedError.body).toBe(text);
  });

  // A 502 whose body is JSON null: RelayerHttpUnexpectedResponseError, not a raw TypeError.
  it('throws RelayerHttpUnexpectedResponseError for a non-2xx response whose body is JSON null', async () => {
    global.fetch = vi.fn().mockResolvedValue(new Response('null', { status: 502 }));

    const request = new RelayerHttpRequest({ relayerOperation: 'USER_DECRYPT', url: RELAYER_URL, payload: {} });

    await expect(request.run()).rejects.toBeInstanceOf(RelayerHttpUnexpectedResponseError);
  });

  // Bug: the body is parsed whatever its size; JSON.parse can take several times the
  // string's memory. Bodies over 10 Mi characters must be rejected before parsing.
  // Each body below is a valid JSON string, so without the check it gets parsed.
  const MAX_BODY_TEXT_LENGTH = 10 * 1024 * 1024;

  // A 200 body one character over 10 Mi: RelayerHttpFetchError 'too large', never parsed.
  it('throws RelayerHttpFetchError without parsing a body over the size limit', async () => {
    const body = `"${'a'.repeat(MAX_BODY_TEXT_LENGTH - 1)}"`;
    global.fetch = vi.fn().mockResolvedValue(new Response(body, { status: 200 }));
    const parseSpy = vi.spyOn(JSON, 'parse');

    const request = new RelayerHttpRequest({ relayerOperation: 'USER_DECRYPT', url: RELAYER_URL, payload: {} });
    const error = await request.run().catch((e: unknown) => e);

    expect(error).toBeInstanceOf(RelayerHttpFetchError);
    expect((error as Error).message).toContain('too large');
    expect((error as RelayerHttpFetchError).status).toBe(200);
    expect(parseSpy).not.toHaveBeenCalled();
  });

  // A 200 body of exactly 10 Mi characters: parsed normally.
  it('parses a body exactly at the size limit', async () => {
    const body = `"${'a'.repeat(MAX_BODY_TEXT_LENGTH - 2)}"`;
    global.fetch = vi.fn().mockResolvedValue(new Response(body, { status: 200 }));
    const parseSpy = vi.spyOn(JSON, 'parse');

    const request = new RelayerHttpRequest({ relayerOperation: 'USER_DECRYPT', url: RELAYER_URL, payload: {} });
    await request.run().catch((e: unknown) => e);

    expect(parseSpy).toHaveBeenCalledTimes(1);
  });

  // Body stream fails during the read: RelayerHttpFetchError keeping the 200 status and the stream error as cause.
  it('throws RelayerHttpFetchError keeping the status and cause when reading the body fails', async () => {
    const streamError = new TypeError('terminated');
    const body = new ReadableStream<Uint8Array>({
      pull(controller) {
        controller.error(streamError);
      },
    });
    global.fetch = vi.fn().mockResolvedValue(new Response(body, { status: 200 }));

    const request = new RelayerHttpRequest({ relayerOperation: 'USER_DECRYPT', url: RELAYER_URL, payload: {} });
    const error = await request.run().catch((e: unknown) => e);

    expect(error).toBeInstanceOf(RelayerHttpFetchError);
    expect((error as RelayerHttpFetchError).status).toBe(200);
    expect((error as Error).cause).toBe(streamError);
  });

  // A custom fetch rejecting with null (no abort): still a RelayerHttpFetchError, not a TypeError from the SDK.
  it.each([
    ['fetch', () => vi.fn().mockRejectedValue(null)],
    [
      'the body read',
      () =>
        vi
          .fn()
          .mockResolvedValue(Object.assign(new Response('{}', { status: 200 }), { text: () => Promise.reject(null) })),
    ],
  ])('throws RelayerHttpFetchError when %s rejects with null', async (_label, makeFetch) => {
    global.fetch = makeFetch();

    const request = new RelayerHttpRequest({
      relayerOperation: 'USER_DECRYPT',
      url: RELAYER_URL,
      payload: {},
      options: { fetchRetries: 0 },
    });
    const error = await request.run().catch((e: unknown) => e);

    expect(error).toBeInstanceOf(RelayerHttpFetchError);
  });
});
