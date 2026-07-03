// CLI: run the standalone gateway server until Ctrl-C. Useful for local dev and
// for non-Playwright platforms (e.g. node-server). Playwright globalSetup starts
// the server in-process instead.
//
//   npx tsx test/infra/gateway/serve.ts

import { GATEWAY_PORT } from '../config.js';
import { gatewayConfig } from '../topology.js';
import { closeGatewayServer, createGatewayServer, listenGatewayServer } from './server.js';

const server = createGatewayServer(gatewayConfig());
await listenGatewayServer(server, GATEWAY_PORT);
console.log(`gateway listening on http://127.0.0.1:${String(GATEWAY_PORT)}`);

const shutdown = async (): Promise<void> => {
  await closeGatewayServer(server);
  process.exit(0);
};
process.on('SIGINT', () => void shutdown());
process.on('SIGTERM', () => void shutdown());
