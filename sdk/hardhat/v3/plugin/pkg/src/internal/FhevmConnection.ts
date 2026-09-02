// The per-connection fhevm object — hardhat 3 scopes networks to CONNECTIONS, so fhevm state lives
// on each one (v2 had a per-process singleton). This is the seed the public API grows on: the D-stage
// port fills it method group by method group; today it says what kind of network it is on.

import type { NetworkConnection } from 'hardhat/types/network';

import { type FhevmNetworkInfo, isCleartextNetwork, isDevelopmentNetwork } from './network.js';

export interface HardhatFhevm {
  /** The detected network: name, live chain id, kind, remote URL. */
  readonly network: FhevmNetworkInfo;
  /** @deprecated Same as {@link isCleartext}; kept for v2 call sites. */
  readonly isMock: boolean;
  /** True when the SDK talks to this node in cleartext mode (every development node). */
  readonly isCleartext: boolean;
  /** True on a development node the plugin may deploy the cleartext stack onto. */
  readonly isDevelopment: boolean;
}

export function createFhevmConnection(_connection: NetworkConnection<string>, network: FhevmNetworkInfo): HardhatFhevm {
  const isCleartext = isCleartextNetwork(network);
  return Object.freeze({
    network,
    isMock: isCleartext,
    isCleartext,
    isDevelopment: isDevelopmentNetwork(network),
  });
}
