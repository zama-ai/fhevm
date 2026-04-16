import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  generateTransportKeypair as generateTransportKeypair_,
  type TransportKeypair,
} from '../../kms/TransportKeypair-p.js';

////////////////////////////////////////////////////////////////////////////////

export type GenerateTransportKeypairReturnType = TransportKeypair;

export async function generateTransportKeypair(
  fhevm: Fhevm<FhevmChain | undefined, WithDecrypt, OptionalNativeClient>,
): Promise<GenerateTransportKeypairReturnType> {
  return await generateTransportKeypair_(fhevm);
}

////////////////////////////////////////////////////////////////////////////////
