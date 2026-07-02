// Standalone HTTP gateway server. Wraps the Node adapter in an http.Server so
// platforms can run it as a separate process (or in-process) and proxy to it
// same-origin (Next rewrites, a Vite proxy, etc.) — instead of bundling the
// gateway core into a framework, which collides with NodeNext `.js` extensions.

import { type Server, createServer } from 'node:http';
import { type GatewayConfig } from './gateway.js';
import { createNodeGateway } from './nodeAdapter.js';

export function createGatewayServer(config: GatewayConfig): Server {
  const middleware = createNodeGateway(config);
  return createServer((req, res) => {
    void middleware(req, res, () => {
      res.statusCode = 404;
      res.setHeader('content-type', 'text/plain');
      res.end('Not found');
    });
  });
}

export function listenGatewayServer(server: Server, port: number, host = '127.0.0.1'): Promise<void> {
  return new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(port, host, () => {
      server.off('error', reject);
      resolve();
    });
  });
}

export function closeGatewayServer(server: Server): Promise<void> {
  return new Promise((resolve, reject) => {
    server.close((err) => {
      if (err !== undefined) {
        reject(err);
      } else {
        resolve();
      }
    });
  });
}
