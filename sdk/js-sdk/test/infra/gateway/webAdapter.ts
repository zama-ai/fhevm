// Web (Fetch API) adapter for the gateway — for Next.js route handlers, which
// receive a `Request` and return a `Response`. Mirrors nodeAdapter.ts; both call
// the same `handleGateway`.

import { type GatewayConfig, handleGateway } from './gateway.js';

export type WebGatewayHandler = (request: Request) => Promise<Response>;

export function createWebGateway(config: GatewayConfig): WebGatewayHandler {
  return async (request) => {
    const url = new URL(request.url);
    const method = request.method;
    const body = method === 'GET' || method === 'HEAD' ? undefined : new Uint8Array(await request.arrayBuffer());

    const result = await handleGateway(config, {
      method,
      pathname: url.pathname,
      search: url.search,
      body,
      publicOrigin: url.origin,
    });

    if (result === undefined) {
      return new Response('Not found', { status: 404 });
    }

    // A Uint8Array is a valid BodyInit at runtime; cast around TS 5.7's
    // ArrayBufferLike width.
    const responseBody = typeof result.body === 'string' ? result.body : (result.body as unknown as BodyInit);
    return new Response(responseBody, { status: result.status, headers: { ...result.headers } });
  };
}
