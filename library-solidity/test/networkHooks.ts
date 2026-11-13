// Network hook used by the tests, the HH3 port of HH2's CustomProvider.ts (registered by the inline plugin in
// hardhat.config.ts).
//
// The mocked coprocessor (coprocessorUtils.ts) replays the FHEVMExecutor events from the last block it processed.
// After `evm_revert`, the chain is back at an earlier block: it must replay from there, so the hook records the block
// number reached after every revert and serves it through two custom JSON-RPC methods:
//   - `get_lastBlockSnapshot`: returns that block number;
//   - `set_lastBlockSnapshot` [blockNumber]: sets it (after the coprocessor processed the events up to it).
// The state is kept per network connection.
//
// Not ported (unused): HH2's +20% on eth_estimateGas (for an Ethermint chain), the FHEVMExecutor random counter read
// after a revert, and the `…ForDecrypt` methods.
import type { NetworkHooks } from 'hardhat/types/hooks';
import type { NetworkConnection } from 'hardhat/types/network';
import type { JsonRpcRequest, JsonRpcResponse } from 'hardhat/types/providers';

const lastBlockSnapshots = new WeakMap<NetworkConnection, number>();

function result(request: JsonRpcRequest, value: unknown): JsonRpcResponse {
  return { jsonrpc: '2.0', id: request.id, result: value };
}

function firstNumberParam(request: JsonRpcRequest): number {
  const first: unknown = Array.isArray(request.params) ? request.params[0] : undefined;
  if (typeof first !== 'number' || !Number.isInteger(first)) {
    throw new Error(`${request.method}: expected an integer as first parameter, got ${String(first)}`);
  }
  return first;
}

export default async (): Promise<Partial<NetworkHooks>> => {
  const handlers: Partial<NetworkHooks> = {
    async onRequest(context, networkConnection, jsonRpcRequest, next) {
      // The state is keyed by connection, whatever its chain type.
      const connection = networkConnection as unknown as NetworkConnection;

      if (jsonRpcRequest.method === 'get_lastBlockSnapshot') {
        return result(jsonRpcRequest, lastBlockSnapshots.get(connection) ?? 0);
      }
      if (jsonRpcRequest.method === 'set_lastBlockSnapshot') {
        const blockNumber = firstNumberParam(jsonRpcRequest);
        lastBlockSnapshots.set(connection, blockNumber);
        return result(jsonRpcRequest, blockNumber);
      }

      const response = await next(context, networkConnection, jsonRpcRequest);
      if (jsonRpcRequest.method === 'evm_revert') {
        const blockNumberHex: unknown = await networkConnection.provider.request({ method: 'eth_blockNumber' });
        if (typeof blockNumberHex !== 'string' || !/^0x[0-9a-fA-F]+$/.test(blockNumberHex)) {
          throw new Error(`eth_blockNumber: expected a hex quantity, got ${String(blockNumberHex)}`);
        }
        lastBlockSnapshots.set(connection, parseInt(blockNumberHex, 16));
      }
      return response;
    },
  };
  return await Promise.resolve(handlers);
};
