import { describe, it, expect } from 'vitest';
import { readErrorMessage } from './readErrorMessage.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/readErrorMessage.test.ts
////////////////////////////////////////////////////////////////////////////////

describe('readErrorMessage', () => {
  //////////////////////////////////////////////////////////////////////////////
  // JSON error bodies are surfaced
  //////////////////////////////////////////////////////////////////////////////

  it('surfaces message + label from a relayer-shaped body { error: { message, label } }', async () => {
    const response = new Response(
      JSON.stringify({ error: { message: 'missing or invalid API key', label: 'unauthorized' } }),
      { status: 401 },
    );
    await expect(readErrorMessage(response)).resolves.toEqual({
      message: 'missing or invalid API key',
      label: 'unauthorized',
    });
  });

  it('surfaces message + label from a flat Cloudflare/Kong body { message, label }', async () => {
    const response = new Response(
      JSON.stringify({ message: 'Missing or invalid Zama API Key', label: 'unauthorized' }),
      { status: 403 },
    );
    await expect(readErrorMessage(response)).resolves.toEqual({
      message: 'Missing or invalid Zama API Key',
      label: 'unauthorized',
    });
  });

  it('surfaces message only when label is absent', async () => {
    const response = new Response(JSON.stringify({ message: 'forbidden by edge' }), { status: 403 });
    await expect(readErrorMessage(response)).resolves.toEqual({
      message: 'forbidden by edge',
      label: undefined,
    });
  });

  it('surfaces label only when message is absent', async () => {
    const response = new Response(JSON.stringify({ label: 'unauthorized' }), { status: 401 });
    await expect(readErrorMessage(response)).resolves.toEqual({
      message: undefined,
      label: 'unauthorized',
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // Nothing is surfaced for empty / non-JSON / irrelevant bodies
  //////////////////////////////////////////////////////////////////////////////

  it('returns {} for an empty body', async () => {
    const response = new Response('', { status: 401 });
    await expect(readErrorMessage(response)).resolves.toEqual({});
  });

  it('returns {} for a non-JSON text body (e.g. bare "Forbidden")', async () => {
    const response = new Response('Forbidden', { status: 403 });
    await expect(readErrorMessage(response)).resolves.toEqual({});
  });

  it('returns {} for an HTML error page', async () => {
    const response = new Response('<html><body>403 Forbidden</body></html>', { status: 403 });
    await expect(readErrorMessage(response)).resolves.toEqual({});
  });

  it('returns {} for JSON without message/label fields', async () => {
    const response = new Response(JSON.stringify({ foo: 'bar', code: 42 }), { status: 500 });
    await expect(readErrorMessage(response)).resolves.toEqual({});
  });

  it('returns {} when error.message/label are not strings', async () => {
    const response = new Response(JSON.stringify({ error: { message: 123, label: ['x'] } }), { status: 401 });
    await expect(readErrorMessage(response)).resolves.toEqual({});
  });

  it('treats empty-string message/label as absent', async () => {
    const response = new Response(JSON.stringify({ error: { message: '', label: '' } }), { status: 401 });
    await expect(readErrorMessage(response)).resolves.toEqual({});
  });

  it('never throws when the body cannot be read', async () => {
    const broken = { text: () => Promise.reject(new Error('body unusable')) } as unknown as Response;
    await expect(readErrorMessage(broken)).resolves.toEqual({});
  });
});
