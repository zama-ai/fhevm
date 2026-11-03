import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithDecrypt, WithEncryptTransci } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  generateTransportKeyPair as generateTransportKeyPair_,
  type TransportKeyPair,
  type TransportKeyPairGenerator,
} from '../../kms/TransportKeyPair-p.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';
import { hasFhevmRuntimeModule } from '../../runtime/CoreFhevmRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

export type GenerateTransportKeyPairReturnType = TransportKeyPair & { readonly tkmsVersion: string };

/**
 * Generates a fresh transport key pair. Works on a decrypt (or full) client and
 * on an encrypt-transciphering client; both load the TKMS WASM, and the key
 * pairs they produce are interchangeable on the same runtime.
 */
export async function generateTransportKeyPair(
  fhevm: Fhevm<FhevmChain, WithDecrypt> | Fhevm<FhevmChain, WithEncryptTransci>,
): Promise<GenerateTransportKeyPairReturnType> {
  const fhevmContext = await initPublicAction(fhevm);
  return await generateTransportKeyPair_(fhevm, { fhevmContext, generator: _resolveGenerator(fhevm) });
}

////////////////////////////////////////////////////////////////////////////////

function _resolveGenerator(
  fhevm: Fhevm<FhevmChain, WithDecrypt> | Fhevm<FhevmChain, WithEncryptTransci>,
): TransportKeyPairGenerator {
  const runtime = fhevm.runtime;
  if (hasFhevmRuntimeModule(runtime, 'decrypt')) {
    return runtime.decrypt;
  }
  if (hasFhevmRuntimeModule(runtime, 'encryptTransci')) {
    return runtime.encryptTransci;
  }
  throw new Error('generateTransportKeyPair requires a decrypt or encrypt-transciphering client.');
}
