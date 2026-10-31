import { describe, it, expect, afterEach, vi } from 'vitest';
import { RelayerHttpRequest } from './RelayerHttpRequest.js';
import { RelayerHttpResultError } from '../../../errors/RelayerHttpResultError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/RelayerHttpRequest.publicDecrypt.test.ts
////////////////////////////////////////////////////////////////////////////////

const RELAYER_URL = 'https://relayer-http.example.com/v4/public-decrypt';
const VALID_REQUEST_ID = '018f4d2e-7c3a-7b1e-8c2d-abcdef123456';
const SIGNATURE = 'ab'.repeat(65);

const VALID_RESULT = {
  decryptedValue: '00'.repeat(31) + '2a',
  signatures: [SIGNATURE, 'cd'.repeat(65)],
  extraData: '0x00',
};

function _successBody(result: unknown): { status: string; requestId: string; result: unknown } {
  return { status: 'succeeded', requestId: VALID_REQUEST_ID, result };
}

function _run(body: unknown): Promise<unknown> {
  global.fetch = vi.fn().mockResolvedValue(new Response(JSON.stringify(body), { status: 200 }));
  const request = new RelayerHttpRequest({ relayerOperation: 'PUBLIC_DECRYPT', url: RELAYER_URL, payload: {} });
  return request.run();
}

describe('RelayerHttpRequest public-decrypt success body', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
  });

  it('returns the result object of a valid body', async () => {
    await expect(_run(_successBody(VALID_RESULT))).resolves.toEqual(VALID_RESULT);
  });

  it('accepts an empty extraData (0x)', async () => {
    const result = { ...VALID_RESULT, extraData: '0x' };
    await expect(_run(_successBody(result))).resolves.toEqual(result);
  });

  it.each([
    ['status is not succeeded', { ..._successBody(VALID_RESULT), status: 'failed' }],
    ['requestId is not a UUIDv7', { ..._successBody(VALID_RESULT), requestId: 'abc' }],
    ['result is missing', { status: 'succeeded', requestId: VALID_REQUEST_ID }],
    ['result is null', _successBody(null)],
    [
      'result is a user-decrypt share array',
      _successBody([{ payload: 'aa', signature: SIGNATURE, extraData: '0x00' }]),
    ],
    ['decryptedValue is missing', _successBody({ signatures: [SIGNATURE], extraData: '0x00' })],
    ['decryptedValue has a 0x prefix', _successBody({ ...VALID_RESULT, decryptedValue: '0x2a' })],
    ['signatures is missing', _successBody({ decryptedValue: '2a', extraData: '0x00' })],
    ['signatures is empty', _successBody({ ...VALID_RESULT, signatures: [] })],
    ['a signature is not 65 bytes', _successBody({ ...VALID_RESULT, signatures: [SIGNATURE, 'ab'.repeat(64)] })],
    ['a signature has a 0x prefix', _successBody({ ...VALID_RESULT, signatures: [`0x${SIGNATURE}`] })],
    ['extraData is missing', _successBody({ decryptedValue: '2a', signatures: [SIGNATURE] })],
    ['extraData has no 0x prefix', _successBody({ ...VALID_RESULT, extraData: '00' })],
  ])('throws RelayerHttpResultError when %s', async (_label, body) => {
    const error = await _run(body).catch((e: unknown) => e);

    expect(error).toBeInstanceOf(RelayerHttpResultError);
    expect((error as RelayerHttpResultError).url).toBe(RELAYER_URL);
  });
});
