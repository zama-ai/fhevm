import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  generateTransportKeyPair as generateTransportKeyPair_,
  type TransportKeyPair,
} from '../../kms/TransportKeyPair-p.js';
import { asFhevmWithTkmsVersion } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type GenerateTransportKeyPairReturnType = TransportKeyPair & { readonly tkmsVersion: string };

export async function generateTransportKeyPair(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
): Promise<GenerateTransportKeyPairReturnType> {
  const f = asFhevmWithTkmsVersion(fhevm);
  return await generateTransportKeyPair_(f);
}

////////////////////////////////////////////////////////////////////////////////
