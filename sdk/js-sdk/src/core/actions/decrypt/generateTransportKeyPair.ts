import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  generateTransportKeyPair as generateTransportKeyPair_,
  type TransportKeyPair,
} from '../../kms/TransportKeyPair-p.js';

////////////////////////////////////////////////////////////////////////////////

export type GenerateTransportKeyPairReturnType = TransportKeyPair;

export async function generateTransportKeyPair(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
): Promise<GenerateTransportKeyPairReturnType> {
  return await generateTransportKeyPair_(fhevm);
}

////////////////////////////////////////////////////////////////////////////////
