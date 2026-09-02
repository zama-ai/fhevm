// The per-connection fhevm object — hardhat 3 scopes networks to CONNECTIONS, so fhevm state lives
// on each one (v2 had a per-process singleton). This is the seed the public API grows on: the D-stage
// port fills it method group by method group; until then it only says what kind of network it is on.

import type { NetworkConnection } from 'hardhat/types/network';

export interface HardhatFhevm {
  /** True on an in-process or otherwise development-class network (stub until network detection lands). */
  readonly isMock: boolean;
  /** True when the connection targets a cleartext stack (stub until network detection lands). */
  readonly isCleartext: boolean;
}

// Takes the connection although the stub ignores it: the semantics become a function of the network
// when detection lands, and the call site must not change then.
export function createFhevmConnection(_connection: NetworkConnection<string>): HardhatFhevm {
  return Object.freeze({
    isMock: true,
    isCleartext: false,
  });
}
