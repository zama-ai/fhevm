// Node adapter for the gateway — a connect-style middleware usable by both Vite
// (`server.middlewares.use`) and a plain `http.Server`. The Next platform uses a
// separate Web `Request`/`Response` adapter; both call the same `handleGateway`.

import type { IncomingMessage, ServerResponse } from 'node:http';
import { type GatewayConfig, handleGateway } from './gateway.js';

export type NodeGatewayMiddleware = (req: IncomingMessage, res: ServerResponse, next: () => void) => Promise<void>;

export function createNodeGateway(config: GatewayConfig): NodeGatewayMiddleware {
  return async (req, res, next) => {
    const url = new URL(req.url ?? '/', 'http://localhost');
    if (!url.pathname.startsWith(`${config.mountPrefix}/`)) {
      next();
      return;
    }

    const method = req.method ?? 'GET';
    const body = method === 'GET' || method === 'HEAD' ? undefined : await _readBody(req);
    // Behind a same-origin proxy (e.g. Next rewrites), the page origin arrives in
    // x-forwarded-* — Host is the proxy target. Emit byte URLs at the page origin
    // so the browser fetches them same-origin (not the gateway's real port).
    const publicOrigin = _publicOrigin(req);

    const result = await handleGateway(config, {
      method,
      pathname: url.pathname,
      search: url.search,
      body,
      publicOrigin,
    });

    if (result === undefined) {
      next();
      return;
    }

    res.statusCode = result.status;
    for (const [name, value] of Object.entries(result.headers)) {
      res.setHeader(name, value);
    }
    res.end(typeof result.body === 'string' ? result.body : Buffer.from(result.body));
  };
}

function _firstHeader(value: string | string[] | undefined): string | undefined {
  return Array.isArray(value) ? value[0] : value;
}

function _publicOrigin(req: IncomingMessage): string {
  const host = _firstHeader(req.headers['x-forwarded-host']) ?? req.headers.host ?? 'localhost';
  const proto = _firstHeader(req.headers['x-forwarded-proto']) ?? 'http';
  return `${proto}://${host}`;
}

function _readBody(req: IncomingMessage): Promise<Uint8Array> {
  return new Promise((resolve, reject) => {
    const chunks: Buffer[] = [];
    req.on('data', (chunk: Buffer | string) => {
      chunks.push(typeof chunk === 'string' ? Buffer.from(chunk) : chunk);
    });
    req.on('end', () => resolve(Uint8Array.from(Buffer.concat(chunks))));
    req.on('error', reject);
  });
}
