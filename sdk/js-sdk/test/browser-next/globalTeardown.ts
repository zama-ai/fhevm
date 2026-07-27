// Playwright globalTeardown: close the gateway server and stop the anvils
// spawned in globalSetup (reused anvils are left running).

import { stopAnvils } from '../infra/anvil/anvils.js';
import { closeGatewayServer } from '../infra/gateway/server.js';
import { getSetupState } from './setupState.js';

export default async function globalTeardown(): Promise<void> {
  const state = getSetupState();
  if (state === undefined) {
    return;
  }
  // Only close the gateway if this run started it (undefined = externally
  // launched and reused → leave it running). stopAnvils already skips reused
  // anvils (proc === undefined).
  if (state.gatewayServer !== undefined) {
    await closeGatewayServer(state.gatewayServer);
  }
  await stopAnvils(state.anvils);
}
