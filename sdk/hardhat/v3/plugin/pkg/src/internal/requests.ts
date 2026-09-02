// The request-level behaviours v2's provider wrapper kept once the mock relayer left: inflate
// `eth_estimateGas` (some nodes under-estimate FHE calls) and decorate a failed `eth_sendTransaction`
// with the decoded FHEVM error. Hardhat 3 hands us JSON-RPC RESPONSES here, so a failure arrives as
// `response.error`, not as a throw.

import type { JsonRpcRequest, JsonRpcResponse, SuccessfulJsonRpcResponse } from 'hardhat/types/providers';

const GAS_ESTIMATE_PERCENTAGE = 120n;

export type ForwardRequest = (request: JsonRpcRequest) => Promise<JsonRpcResponse>;

export async function handleRequest(request: JsonRpcRequest, forward: ForwardRequest): Promise<JsonRpcResponse> {
  switch (request.method) {
    case 'eth_estimateGas':
      return inflateGasEstimate(await forward(request));
    case 'eth_sendTransaction':
      return decorateSendError(await forward(request));
    default:
      return forward(request);
  }
}

export function inflateGasEstimate(response: JsonRpcResponse): JsonRpcResponse {
  if (!isSuccessful(response) || typeof response.result !== 'string') return response;
  const inflated = (BigInt(response.result) * GAS_ESTIMATE_PERCENTAGE) / 100n;
  return { ...response, result: `0x${inflated.toString(16)}` };
}

// The FHEVM error decoder lands with the error layer; until then a failed send passes through.
export function decorateSendError(response: JsonRpcResponse): JsonRpcResponse {
  return response;
}

function isSuccessful(response: JsonRpcResponse): response is SuccessfulJsonRpcResponse {
  return 'result' in response;
}
