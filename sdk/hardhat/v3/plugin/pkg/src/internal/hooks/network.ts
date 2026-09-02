// Network hook handlers: attach `connection.fhevm` on creation, release it on close, and shape the
// JSON-RPC stream (`onRequest`) the way v2's provider wrapper did.
//
// The factory runs at most once per HardhatRuntimeEnvironment, which makes it the documented home
// for per-connection bookkeeping. `newConnection` is a decorator chain: take what `next()` built,
// detect the network, prepare the chain (the cleartext stack, on a development node), attach fhevm,
// return the SAME connection object.
//
// The generic signatures repeat hardhat's own NetworkHooks declarations VERBATIM (`ChainTypeT
// extends ChainType | string`), which its custom chain-type strings require — the rule below cannot
// see that.
/* eslint-disable @typescript-eslint/no-redundant-type-constituents */

import type { Deployed } from '@fhevm/host-contracts-cleartext/ts';
import type { HookContext, NetworkHooks } from 'hardhat/types/hooks';
import type { ChainType, NetworkConnection } from 'hardhat/types/network';
import type { JsonRpcRequest, JsonRpcResponse } from 'hardhat/types/providers';

import { createFhevmConnection } from '../FhevmConnection.js';
import { resolveFhevmNetwork } from '../network.js';
import { prepareDevelopmentChain } from '../prepare.js';
import { handleRequest } from '../requests.js';

export default (): Promise<Partial<NetworkHooks>> => {
  // The stack each development connection runs against; absent for public networks.
  const stackByConnection = new WeakMap<NetworkConnection<ChainType | string>, Deployed>();

  return Promise.resolve({
    async newConnection<ChainTypeT extends ChainType | string>(
      context: HookContext,
      next: (nextContext: HookContext) => Promise<NetworkConnection<ChainTypeT>>,
    ): Promise<NetworkConnection<ChainTypeT>> {
      const connection = await next(context);
      const network = await resolveFhevmNetwork(connection);
      const stack = await prepareDevelopmentChain(connection, network);
      if (stack !== undefined) stackByConnection.set(connection, stack);
      connection.fhevm = createFhevmConnection(connection, network);
      return connection;
    },

    async onRequest<ChainTypeT extends ChainType | string>(
      context: HookContext,
      connection: NetworkConnection<ChainTypeT>,
      request: JsonRpcRequest,
      next: (
        nextContext: HookContext,
        nextConnection: NetworkConnection<ChainTypeT>,
        nextRequest: JsonRpcRequest,
      ) => Promise<JsonRpcResponse>,
    ): Promise<JsonRpcResponse> {
      return handleRequest(request, (forwarded) => next(context, connection, forwarded));
    },

    async closeConnection<ChainTypeT extends ChainType | string>(
      context: HookContext,
      connection: NetworkConnection<ChainTypeT>,
      next: (nextContext: HookContext, nextConnection: NetworkConnection<ChainTypeT>) => Promise<void>,
    ): Promise<void> {
      stackByConnection.delete(connection);
      await next(context, connection);
    },
  });
};
