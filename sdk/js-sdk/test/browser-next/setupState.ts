// Shares state between globalSetup and globalTeardown. Playwright runs both in
// the same Node process, so module-level state persists across them.

import type { Server } from 'node:http';
import type { AnvilHandle } from '../infra/anvil/anvils.js';

export type SetupState = {
  readonly anvils: readonly AnvilHandle[];
  /**
   * The gateway server this run started, or `undefined` when an externally
   * launched gateway (e.g. `test/infra/up.sh`) was reused — in which case
   * teardown must NOT close it.
   */
  readonly gatewayServer: Server | undefined;
};

let _state: SetupState | undefined;

export function setSetupState(state: SetupState): void {
  _state = state;
}

export function getSetupState(): SetupState | undefined {
  return _state;
}
