import { describe, it, expect, afterEach, vi } from 'vitest';
import { RelayerHttpRequest } from '../RelayerHttpRequest.js';
import { RelayerHttpResultError } from '../../../../errors/RelayerHttpResultError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/test/RelayerHttpRequest.userDecrypt.test.ts
////////////////////////////////////////////////////////////////////////////////

const RELAYER_URL = 'https://relayer-http.example.com/v4/user-decrypt';
const VALID_REQUEST_ID = '9b2c1f4e-3a5d-4e6f-8a7b-1c2d3e4f5a6b';
const SIGNATURE = 'ab'.repeat(65);

const VALID_SHARES = [
  { payload: 'a1b2', signature: SIGNATURE, extraData: '0x00' },
  { payload: 'c3d4', signature: 'cd'.repeat(65), extraData: '0x00' },
];

// relayer-http's user-decrypt `result` wraps the shares: `{ "result": [ ...shares ] }`
// (relayer-http/crates/relayer-http/src/endpoint/docs.md, section 4).
function _successBody(result: unknown): { status: string; requestId: string; result: unknown } {
  return { status: 'succeeded', requestId: VALID_REQUEST_ID, result };
}

function _run(body: unknown): Promise<unknown> {
  global.fetch = vi.fn().mockResolvedValue(new Response(JSON.stringify(body), { status: 200 }));
  const request = new RelayerHttpRequest({ relayerOperation: 'USER_DECRYPT', url: RELAYER_URL, payload: {} });
  return request.run();
}

describe('RelayerHttpRequest user-decrypt success body', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
  });

  // A valid body with wrapped shares: run() resolves to the share array.
  it('returns the share array of a valid body', async () => {
    await expect(_run(_successBody({ result: VALID_SHARES }))).resolves.toEqual(VALID_SHARES);
  });

  // A malformed success body (status, requestId, wrapping or share field): RelayerHttpResultError with the url.
  it.each([
    ['status is not succeeded', { ..._successBody({ result: VALID_SHARES }), status: 'failed' }],
    ['requestId is not a UUIDv4', { ..._successBody({ result: VALID_SHARES }), requestId: 'abc' }],
    ['result is missing', { status: 'succeeded', requestId: VALID_REQUEST_ID }],
    ['result is null', _successBody(null)],
    ['result is the share array itself, not wrapped', _successBody(VALID_SHARES)],
    ['result.result is missing', _successBody({})],
    ['result.result is empty', _successBody({ result: [] })],
    ['a payload has a 0x prefix', _successBody({ result: [{ ...VALID_SHARES[0], payload: '0xa1b2' }] })],
    ['a payload is empty', _successBody({ result: [VALID_SHARES[0], { ...VALID_SHARES[1], payload: '' }] })],
    ['a signature is not 65 bytes', _successBody({ result: [{ ...VALID_SHARES[0], signature: 'ab'.repeat(64) }] })],
    ['extraData is missing', _successBody({ result: [{ payload: 'a1b2', signature: SIGNATURE }] })],
  ])('throws RelayerHttpResultError when %s', async (_label, body) => {
    const error = await _run(body).catch((e: unknown) => e);

    expect(error).toBeInstanceOf(RelayerHttpResultError);
    expect((error as RelayerHttpResultError).url).toBe(RELAYER_URL);
  });
});
