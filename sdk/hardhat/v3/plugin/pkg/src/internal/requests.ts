// The request-level behaviours v2's provider wrapper kept once the mock relayer left: inflate
// `eth_estimateGas` (some nodes under-estimate FHE calls) and explain a revert inside an FHEVM
// contract. Hardhat 3 THROWS a failed request (EDR's SolidityError, a node's ProviderError), so the
// decoration catches, rewrites the error in place and rethrows the same object — chai matchers and
// ethers keep the `data` they inspect.

import type { JsonRpcRequest, JsonRpcResponse, SuccessfulJsonRpcResponse } from 'hardhat/types/providers';

import type { FhevmContractsRepository } from './contracts.js';
import { decorateRevertError } from './errors/decorate.js';
import type { TransactionParties } from './errors/messages.js';

const GAS_ESTIMATE_PERCENTAGE = 120n;

export type ForwardRequest = (request: JsonRpcRequest) => Promise<JsonRpcResponse>;

export async function handleRequest(
  request: JsonRpcRequest,
  forward: ForwardRequest,
  repository?: FhevmContractsRepository,
): Promise<JsonRpcResponse> {
  const response = await forwardDecorated(request, forward, repository);
  return request.method === 'eth_estimateGas' ? inflateGasEstimate(response) : response;
}

async function forwardDecorated(
  request: JsonRpcRequest,
  forward: ForwardRequest,
  repository: FhevmContractsRepository | undefined,
): Promise<JsonRpcResponse> {
  if (repository === undefined) return forward(request);
  try {
    return await forward(request);
  } catch (e) {
    decorateRevertError(repository, e, transactionParties(request));
    throw e;
  }
}

export function inflateGasEstimate(response: JsonRpcResponse): JsonRpcResponse {
  if (!isSuccessful(response) || typeof response.result !== 'string') return response;
  const inflated = (BigInt(response.result) * GAS_ESTIMATE_PERCENTAGE) / 100n;
  return { ...response, result: `0x${inflated.toString(16)}` };
}

// The `{ from, to }` of a call or send payload, for messages that name the transaction parties.
export function transactionParties(request: JsonRpcRequest): TransactionParties {
  const params: unknown = request.params;
  const tx: unknown = Array.isArray(params) ? params[0] : undefined;
  if (typeof tx !== 'object' || tx === null) return {};
  return {
    from: 'from' in tx && typeof tx.from === 'string' ? tx.from : undefined,
    to: 'to' in tx && typeof tx.to === 'string' ? tx.to : undefined,
  };
}

function isSuccessful(response: JsonRpcResponse): response is SuccessfulJsonRpcResponse {
  return 'result' in response;
}
