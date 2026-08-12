import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  generateTransportKeyPair as generateTransportKeyPair_,
  type TransportKeyPair,
} from '../../kms/TransportKeyPair-p.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type GenerateTransportKeyPairReturnType = TransportKeyPair & { readonly tkmsVersion: string };

export async function generateTransportKeyPair(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
): Promise<GenerateTransportKeyPairReturnType> {
  const fhevmContext = await initPublicAction(fhevm);
  return await generateTransportKeyPair_(fhevm, { fhevmContext });
}

////////////////////////////////////////////////////////////////////////////////
