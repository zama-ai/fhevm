import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { toTransportKeypair, type TransportKeypair } from '../../kms/TransportKeypair-p.js';

////////////////////////////////////////////////////////////////////////////////

export type ParseTransportKeypairParameters = {
  readonly publicKey: string;
  readonly privateKey: string;
};

export type ParseTransportKeypairReturnType = TransportKeypair;

export async function parseTransportKeypair(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: ParseTransportKeypairParameters,
): Promise<ParseTransportKeypairReturnType> {
  return toTransportKeypair(fhevm, parameters);
}
