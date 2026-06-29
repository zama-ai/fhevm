// Launch the full test infra (2 cleartext anvils + the same-origin gateway) and
// keep it running until Ctrl-C. While it's up, run any number of tests against
// it — Playwright globalSetup reuses the live anvils and gateway instead of
// starting its own (and teardown leaves them running).
//
//   npx tsx test/infra/up.ts        (or: test/infra/up.sh [-d|--detach])

import { startAnvils, stopAnvils } from './anvil/anvils.js';
import { closeGatewayServer, createGatewayServer, listenGatewayServer } from './gateway/server.js';
import { GATEWAY_MOUNT_PREFIX, GATEWAY_PORT } from './config.js';
import { anvilSpecs, gatewayConfig } from './topology.js';

async function gatewayAlreadyUp(): Promise<boolean> {
  try {
    const res = await fetch(`http://127.0.0.1:${String(GATEWAY_PORT)}${GATEWAY_MOUNT_PREFIX}/v12/relayer/v2/keyurl`, {
      // Bounded: never block on a wedged/half-open listener.
      signal: AbortSignal.timeout(3_000),
    });
    return res.ok;
  } catch {
    return false;
  }
}

const anvils = await startAnvils(anvilSpecs());
console.log('anvils:', anvils.map((h) => `${h.spec.slot}@${h.rpcUrl}${h.reused ? ' (reused)' : ''}`).join(', '));

// Idempotent: if a gateway is already serving (a previous `up`), reuse it and
// exit — that process keeps it (and the anvils) alive. Avoids EADDRINUSE.
if (await gatewayAlreadyUp()) {
  console.log(`gateway: http://127.0.0.1:${String(GATEWAY_PORT)} (already running, reused)`);
  console.log('Infra ready (already running).');
  process.exit(0);
}

const gateway = createGatewayServer(gatewayConfig());
await listenGatewayServer(gateway, GATEWAY_PORT);
console.log(`gateway: http://127.0.0.1:${String(GATEWAY_PORT)}`);

console.log('\nInfra ready. Run tests against it (they reuse this infra), e.g.:');
console.log('  cd test/browser-next && npx playwright test specs/gw-skeleton.spec.ts --config playwright.config.ts');
console.log('\nPress Ctrl-C to stop.');

let stopping = false;
const shutdown = async (): Promise<void> => {
  if (stopping) {
    return;
  }
  stopping = true;
  console.log('\nStopping infra...');
  await closeGatewayServer(gateway);
  await stopAnvils(anvils);
  process.exit(0);
};

process.on('SIGINT', () => void shutdown());
process.on('SIGTERM', () => void shutdown());
