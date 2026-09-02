// Network hook handlers: attach `connection.fhevm` on creation, release it on close.
//
// The factory runs at most once per HardhatRuntimeEnvironment, which makes it the documented home
// for per-connection bookkeeping. `newConnection` is a decorator chain: take what `next()` built,
// attach fhevm, return it.
//
// The generic signatures repeat hardhat's own NetworkHooks declarations VERBATIM (`ChainTypeT
// extends ChainType | string`), which its custom chain-type strings require — the rule below cannot
// see that.
/* eslint-disable @typescript-eslint/no-redundant-type-constituents */

import type { HookContext, NetworkHooks } from 'hardhat/types/hooks';
import type { ChainType, NetworkConnection } from 'hardhat/types/network';

import { createFhevmConnection } from '../FhevmConnection.js';

export default (): Promise<Partial<NetworkHooks>> => {
  const fhevmByConnection = new WeakMap<NetworkConnection<ChainType | string>, boolean>();

  return Promise.resolve({
    async newConnection<ChainTypeT extends ChainType | string>(
      context: HookContext,
      next: (nextContext: HookContext) => Promise<NetworkConnection<ChainTypeT>>,
    ): Promise<NetworkConnection<ChainTypeT>> {
      const connection = await next(context);
      connection.fhevm = createFhevmConnection(connection);
      fhevmByConnection.set(connection, true);
      return connection;
    },

    async closeConnection<ChainTypeT extends ChainType | string>(
      context: HookContext,
      connection: NetworkConnection<ChainTypeT>,
      next: (nextContext: HookContext, nextConnection: NetworkConnection<ChainTypeT>) => Promise<void>,
    ): Promise<void> {
      fhevmByConnection.delete(connection);
      await next(context, connection);
    },
  });
};
