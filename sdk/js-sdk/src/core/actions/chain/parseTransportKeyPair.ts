import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { toTransportKeyPair, type TransportKeyPair } from '../../kms/TransportKeyPair-p.js';

////////////////////////////////////////////////////////////////////////////////

export type ParseTransportKeyPairParameters = {
  readonly publicKey: string;
  readonly privateKey: string;
};

export type ParseTransportKeyPairReturnType = TransportKeyPair;

export async function parseTransportKeyPair(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: ParseTransportKeyPairParameters,
): Promise<ParseTransportKeyPairReturnType> {
  return toTransportKeyPair(fhevm, parameters);
}
