/**
 * A fake fhEVM environment for the preflight tests: a JSON-RPC endpoint (`eth_chainId`, `eth_getCode`), the
 * relayer's `GET /v2/keyurl`, and the FHE key object it points to (`HEAD /key`).
 */
import { type IncomingHttpHeaders, createServer } from 'node:http';
import type { AddressInfo } from 'node:net';

export interface FakeEnvironmentOptions {
  chainId?: number;
  /** Addresses (lower case) with no code; every other address has some. */
  emptyAddresses?: string[];
  keyUrlStatus?: number;
  /** Body of `/v2/keyurl` (default: one FHE public key URL pointing at `/key` on this server). */
  keyUrlBody?: (baseUrl: string) => unknown;
  keyUrlDelayMs?: number;
  keyStatus?: number;
}

export interface FakeEnvironment {
  url: string;
  requests: { method: string; path: string; headers: IncomingHttpHeaders }[];
  close(): Promise<void>;
}

export async function startFakeEnvironment(options: FakeEnvironmentOptions = {}): Promise<FakeEnvironment> {
  const requests: FakeEnvironment['requests'] = [];
  let baseUrl = '';
  const server = createServer((request, response) => {
    requests.push({ method: request.method ?? '', path: request.url ?? '', headers: request.headers });
    const chunks: Buffer[] = [];
    request.on('data', (chunk: Buffer) => chunks.push(chunk));
    request.on('end', () => {
      if (request.method === 'POST') {
        type Call = { id: number; method: string; params?: unknown[] };
        const body = JSON.parse(Buffer.concat(chunks).toString('utf8')) as Call | Call[];
        const result = (call: Call) => {
          if (call.method !== 'eth_getCode') return `0x${(options.chainId ?? 12345).toString(16)}`;
          const address = String(call.params?.[0]).toLowerCase();
          return options.emptyAddresses?.includes(address) ? '0x' : '0x6080';
        };
        const answer = (call: Call) => ({ jsonrpc: '2.0', id: call.id, result: result(call) });
        response.setHeader('content-type', 'application/json');
        response.end(JSON.stringify(Array.isArray(body) ? body.map(answer) : answer(body)));
      } else if (request.url === '/v2/keyurl') {
        setTimeout(() => {
          response.statusCode = options.keyUrlStatus ?? 200;
          response.setHeader('content-type', 'application/json');
          const body = options.keyUrlBody?.(baseUrl) ?? {
            status: 'succeeded',
            response: { fheKeyInfo: [{ fhePublicKey: { dataId: '1', urls: [`${baseUrl}/key`] } }] },
          };
          response.end(JSON.stringify(body));
        }, options.keyUrlDelayMs ?? 0);
      } else if (request.url === '/key') {
        response.statusCode = options.keyStatus ?? 200;
        response.end();
      } else {
        response.statusCode = 404;
        response.end();
      }
    });
  });
  await new Promise<void>((resolve) => server.listen(0, '127.0.0.1', resolve));
  baseUrl = `http://127.0.0.1:${(server.address() as AddressInfo).port}`;
  return {
    url: baseUrl,
    requests,
    close: () =>
      new Promise((resolve) => {
        server.closeAllConnections();
        server.close(() => resolve());
      }),
  };
}
