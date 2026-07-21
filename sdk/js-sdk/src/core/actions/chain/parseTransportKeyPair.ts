import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { TransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import { toTransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type ParseTransportKeyPairParameters = {
  readonly publicKey: string;
  readonly privateKey: string;
  readonly tkmsVersion?: string | undefined;
};

export type ParseTransportKeyPairReturnType = TransportKeyPair;

export async function parseTransportKeyPair(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: ParseTransportKeyPairParameters,
): Promise<ParseTransportKeyPairReturnType> {
  const fhevmContext = await initPublicAction(fhevm);
  return toTransportKeyPair(fhevm, { value: parameters, fhevmContext });
}
