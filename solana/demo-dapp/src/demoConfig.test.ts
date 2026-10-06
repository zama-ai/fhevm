import { expect, test, vi } from 'vitest';
import { address } from '@solana/kit';

import { createFinalizedRpc } from './demoConfig';

test('defaults reads to finalized through the HTTP transport', async () => {
  const requests: { method: string; params: unknown[] }[] = [];
  const fetchMock = vi.spyOn(globalThis, 'fetch').mockImplementation(async (_input: unknown, init?: RequestInit) => {
    const { id, method, params } = JSON.parse(String(init?.body));
    requests.push({ method, params });
    return Response.json({
      jsonrpc: '2.0', id,
      result: method === 'getSlot' ? 42 : { context: { slot: 42 }, value: null },
    });
  });
  try {
    const rpc = createFinalizedRpc('http://localhost:8899');
    const account = address('11111111111111111111111111111111');
    await rpc.getSlot().send();
    await rpc.getAccountInfo(account, { encoding: 'base64' }).send();
    await rpc.getSlot({ commitment: 'finalized' }).send();
    // Kit omits finalized on the wire because it is the server default.
    expect(requests).toEqual([
      { method: 'getSlot', params: [] },
      { method: 'getAccountInfo', params: [account, { encoding: 'base64' }] },
      { method: 'getSlot', params: [] },
    ]);
  } finally {
    fetchMock.mockRestore();
  }
});
