import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  generateE2eTransportKeypair as generateE2eTransportKeypair_,
  type E2eTransportKeypair,
} from '../../kms/E2eTransportKeypair-p.js';

////////////////////////////////////////////////////////////////////////////////

export type GenerateE2eTransportKeypairReturnType = E2eTransportKeypair;

export async function generateE2eTransportKeypair(
  fhevm: Fhevm<FhevmChain | undefined, WithDecrypt, OptionalNativeClient>,
): Promise<GenerateE2eTransportKeypairReturnType> {
  return await generateE2eTransportKeypair_(fhevm);
}

////////////////////////////////////////////////////////////////////////////////
