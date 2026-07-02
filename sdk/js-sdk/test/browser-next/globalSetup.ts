// Playwright globalSetup: bring up the cleartext anvils (one per WASM version)
// and the standalone same-origin gateway server. The Next dev server proxies
// `/gw/*` to the gateway via rewrites (see next.config.mjs). Requires foundry
// (anvil/cast/forge) on PATH.

import type { Server } from 'node:http';
import { startAnvils } from '../infra/anvil/anvils.js';
import { createGatewayServer, listenGatewayServer } from '../infra/gateway/server.js';
import { GATEWAY_MOUNT_PREFIX, GATEWAY_PORT, LEGACY_SLOT } from '../infra/config.js';
import { anvilSpecs, gatewayConfig } from '../infra/topology.js';
import { setSetupState } from './setupState.js';

/* eslint-disable no-console */

async function _gatewayAlreadyUp(): Promise<boolean> {
  try {
    const res = await fetch(
      `http://127.0.0.1:${String(GATEWAY_PORT)}${GATEWAY_MOUNT_PREFIX}/${LEGACY_SLOT}/relayer/v2/keyurl`,
      {
        // Bounded: never block on a wedged/half-open listener.
        signal: AbortSignal.timeout(3_000),
      },
    );
    return res.ok;
  } catch {
    return false;
  }
}

export default async function globalSetup(): Promise<void> {
  const anvils = await startAnvils(anvilSpecs());
  console.log(
    '[globalSetup] anvils ready:',
    anvils.map((h) => `${h.spec.slot}@${h.rpcUrl}${h.reused ? ' (reused)' : ''}`).join(', '),
  );

  // Reuse an externally launched gateway (test/infra/up.sh) if present, so the
  // infra can stay up across many test runs. Otherwise start (and own) one.
  let gatewayServer: Server | undefined;
  if (await _gatewayAlreadyUp()) {
    console.log(`[globalSetup] reusing gateway already on http://127.0.0.1:${String(GATEWAY_PORT)}`);
  } else {
    gatewayServer = createGatewayServer(gatewayConfig());
    await listenGatewayServer(gatewayServer, GATEWAY_PORT);
    console.log(`[globalSetup] gateway listening on http://127.0.0.1:${String(GATEWAY_PORT)}`);
  }

  setSetupState({ anvils, gatewayServer });
}
