import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { TransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import { toTransportKeyPair } from '../../kms/TransportKeyPair-p.js';

////////////////////////////////////////////////////////////////////////////////

export type ParseTransportKeyPairParameters = {
  readonly publicKey: string;
  readonly privateKey: string;
  readonly tkmsVersion?: string | undefined;
};

export type ParseTransportKeyPairReturnType = TransportKeyPair;

// eslint-disable-next-line @typescript-eslint/require-await
export async function parseTransportKeyPair(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: ParseTransportKeyPairParameters,
): Promise<ParseTransportKeyPairReturnType> {
  return toTransportKeyPair(fhevm, parameters);
}
