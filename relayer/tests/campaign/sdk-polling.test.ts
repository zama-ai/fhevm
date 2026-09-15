import { test, expect } from 'bun:test';
import { RelayerAsyncRequest } from '../../../sdk/js-sdk/src/core/modules/relayer/module/RelayerAsyncRequest.ts';

for (const [retryAfter, minimumMs] of [
  [undefined, 2500],
  ['0', 1000],
] as const) {
  test(`SDK polls 202 with Retry-After=${retryAfter ?? 'absent'} and stops on 503`, async () => {
    const calls: Array<{ method: string; at: number }> = [];
    const server = Bun.serve({
      hostname: '127.0.0.1',
      port: 0,
      fetch(request) {
        calls.push({ method: request.method, at: performance.now() });
        const headers = retryAfter === undefined ? {} : { 'Retry-After': retryAfter };
        if (request.method === 'POST') {
          return Response.json(
            { status: 'queued', requestId: 'request', result: { jobId: 'job' } },
            { status: 202, headers },
          );
        }
        if (calls.length === 2) {
          return Response.json({ status: 'queued', requestId: 'request' }, { status: 202, headers });
        }
        return Response.json(
          { status: 'failed', error: { label: 'readiness_check_timed_out', message: 'Readiness exhausted' } },
          { status: 503 },
        );
      },
    });
    try {
      const request = new RelayerAsyncRequest({
        relayerOperation: 'USER_DECRYPT',
        url: `${server.url}v2/user-decrypt`,
        payload: {},
      });
      let failure: any;
      try {
        await request.run();
      } catch (error) {
        failure = error;
      }
      expect(failure?.status).toBe(503);
      expect(failure?.relayerApiError?.label).toBe('readiness_check_timed_out');
      expect(calls.map((c) => c.method)).toEqual(['POST', 'GET', 'GET']);
      for (let i = 1; i < calls.length; i++) {
        expect(calls[i].at - calls[i - 1].at).toBeGreaterThanOrEqual(minimumMs - 25);
      }
      await Bun.sleep(1100);
      expect(calls.length).toBe(3);
    } finally {
      server.stop(true);
    }
  }, 15000);
}
