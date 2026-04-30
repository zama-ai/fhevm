import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  generateTransportKeyPair as generateTransportKeyPair_,
  type TransportKeyPair,
} from '../../kms/TransportKeyPair-p.js';

////////////////////////////////////////////////////////////////////////////////

export type GenerateTransportKeyPairReturnType = TransportKeyPair;

export async function generateTransportKeyPair(
  fhevm: Fhevm<FhevmChain | undefined, WithDecrypt, OptionalNativeClient>,
): Promise<GenerateTransportKeyPairReturnType> {
  return await generateTransportKeyPair_(fhevm);
}

////////////////////////////////////////////////////////////////////////////////
