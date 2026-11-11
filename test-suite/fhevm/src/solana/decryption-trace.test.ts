import { readFileSync } from 'node:fs';
import { afterEach, describe, expect, test } from 'bun:test';

import { nodeOutcomes, RELAYER_HTTP_LOG, recordDecryptionCalls, type RelayerHttpLine } from './decryption-trace';

const RELAYER_HTTP_SRC = new URL('../../../../relayer-http/crates/relayer-http/src/', import.meta.url);
/** A relayer-http source file, without its test module. */
const relayerHttpSource = (path: string): string =>
  readFileSync(new URL(path, RELAYER_HTTP_SRC), 'utf8').split('#[cfg(test)]')[0] ?? '';

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

  test('an answer from in front of relayer-http reaches the caller unchanged', async () => {
    globalThis.fetch = (async (_input: RequestInfo | URL) =>
      new Response('<html>502 Bad Gateway</html>', { status: 502 })) as typeof fetch;

    const { result, calls } = await recordDecryptionCalls(async () =>
      (await fetch('http://relayer/v4/user-decrypt', { method: 'POST', body: '{}' })).text(),
    );

    expect(result).toBe('<html>502 Bad Gateway</html>');
    expect(calls.map(({ status, code, requestId }) => ({ status, code, requestId }))).toEqual([
      { status: 502, code: undefined, requestId: undefined },
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
      line('response accepted', { kms_node: 'kms_1', attempts: 1, elapsed_ms: 40 }),
      line('attempt failed', { kms_node: 'kms_5', attempt: 1, elapsed_ms: 12, code: 'unprocessable', retry_in_ms: null }),
      line('call failed', { kms_node: 'kms_5', attempts: 1, elapsed_ms: 12, error: 'refused' }),
    ]);
    expect(outcomes).toEqual([
      { node: 'kms_1', message: 'response accepted', elapsedMs: 40, attempts: 1, code: undefined },
      { node: 'kms_5', message: 'call failed', elapsedMs: 12, attempts: 1, code: 'unprocessable' },
    ]);
  });

  // A rename in relayer-http would otherwise only show as a scenario waiting for a line that never comes.
  test('reads only the messages and fields relayer-http logs', () => {
    const summary = relayerHttpSource('endpoint/summary.rs');
    expect(/pub const MESSAGE: &str = "([^"]+)";/.exec(summary)?.[1]).toBe(RELAYER_HTTP_LOG.summaryMessage);
    const summaryFields = [...(/pub const FIELDS: &\[Field\] = &\[([\s\S]*?)\n\];/.exec(summary)?.[1] ?? '').matchAll(/name: "(\w+)"/g)].map(
      (match) => match[1],
    );
    const otherFields = [...(/pub const OTHER_LINE_FIELDS: \[&str; \d+\] = \[([^\]]*)\]/.exec(summary)?.[1] ?? '').matchAll(/"(\w+)"/g)].map(
      (match) => match[1],
    );
    for (const field of RELAYER_HTTP_LOG.summaryFields) expect(summaryFields).toContain(field);
    for (const field of RELAYER_HTTP_LOG.nodeFields) expect([...otherFields, ...summaryFields]).toContain(field);
    const aggregator = relayerHttpSource('kms_aggregator/aggregator.rs');
    for (const message of [RELAYER_HTTP_LOG.aggregationStarted, RELAYER_HTTP_LOG.callFailed]) {
      expect(aggregator).toContain(`"${message}"`);
    }
  });
});
