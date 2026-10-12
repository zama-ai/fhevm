import { afterEach, describe, expect, test } from 'bun:test';

import { nodeOutcomes, recordDecryptionCalls, type RelayerHttpLine } from './decryption-trace';

const realFetch = globalThis.fetch;
afterEach(() => {
  globalThis.fetch = realFetch;
});

const answer = (status: number, body: unknown, requestId: string) =>
  new Response(JSON.stringify(body), { status, headers: { 'x-request-id': requestId } });

describe('decryption trace', () => {
  test('records each v4 decryption POST with its status, code and request id, and nothing else', async () => {
    globalThis.fetch = (async (input: RequestInfo | URL) =>
      String(input).endsWith('/v4/user-decrypt')
        ? answer(403, { code: 'acl_denied', message: 'denied', requestId: 'r-1' }, 'r-1')
        : answer(200, { status: 'succeeded', requestId: 'r-2', result: {} }, 'r-2')) as typeof fetch;

    const { calls } = await recordDecryptionCalls(async () => {
      await fetch('http://relayer/v4/user-decrypt', { method: 'POST', body: '{"a":1}' });
      await fetch('http://relayer/v4/public-decrypt', { method: 'POST', body: '{"b":2}' });
      await fetch('http://relayer/healthz');
    });

    expect(calls.map(({ route, status, code, requestId, requestBody }) => ({ route, status, code, requestId, requestBody }))).toEqual([
      { route: 'user-decrypt', status: 403, code: 'acl_denied', requestId: 'r-1', requestBody: '{"a":1}' },
      { route: 'public-decrypt', status: 200, code: undefined, requestId: 'r-2', requestBody: '{"b":2}' },
    ]);
  });

  test('restores the previous fetch when the action throws', async () => {
    const before = globalThis.fetch;
    await expect(recordDecryptionCalls(async () => Promise.reject(new Error('boom')))).rejects.toThrow('boom');
    expect(globalThis.fetch).toBe(before);
  });

  test("a node's outcome is its last line, with the code its last failed attempt logged", () => {
    const line = (message: string, fields: Record<string, unknown>): RelayerHttpLine => ({ message, fields: { message, ...fields } });
    const outcomes = nodeOutcomes([
      line('aggregation started', { nodes: 5, threshold: 3, timeout_ms: 5000 }),
      line('response accepted', { node: 'kms_1', attempts: 1, elapsed_ms: 40 }),
      line('attempt failed', { node: 'kms_5', attempt: 1, elapsed_ms: 12, code: 'kms_context_invalid', retry_in_ms: null }),
      line('call failed', { node: 'kms_5', attempts: 1, elapsed_ms: 12, error: 'refused' }),
    ]);
    expect(outcomes).toEqual([
      { node: 'kms_1', message: 'response accepted', elapsedMs: 40, attempts: 1, code: undefined },
      { node: 'kms_5', message: 'call failed', elapsedMs: 12, attempts: 1, code: 'kms_context_invalid' },
    ]);
  });
});
