import type { WithDecrypt } from '../types/coreFhevmRuntime.js';
import type { KmsSigncryptedShares } from '../types/kms.js';
import type { ClearValue } from '../types/encryptedTypes-p.js';
import type { TransportKeyPair } from './TransportKeyPair-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { transportKeyPairToTkmsPrivateKey } from './TransportKeyPair-p.js';

////////////////////////////////////////////////////////////////////////////////
// decryptKmsSigncryptedShares (with privateKey)
////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: WithDecrypt;
};

type Parameters = {
  readonly kmsSigncryptedShares: KmsSigncryptedShares;
  readonly transportKeyPair: TransportKeyPair;
  readonly fhevmContext: FhevmClientFrozenContext;
};

type ReturnType = readonly ClearValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptKmsSigncryptedShares(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { transportKeyPair: transportKeyPair, kmsSigncryptedShares, fhevmContext } = parameters;

  if (fhevmContext.tkmsVersion !== kmsSigncryptedShares.tkmsVersion) {
    throw new Error('TkmsVersion mismatch');
  }

  // Could check compatibility of transportKeyPair bytes and
  // context tkmsVersion

  // also validates `transportKeyPair`
  const tkmsPrivateKey = await transportKeyPairToTkmsPrivateKey(context, { transportKeyPair, fhevmContext });
  try {
    // Using the `KmsSigncryptedShares` decrypt and reconstruct clear values
    const orderedDecryptedHandles: readonly ClearValue[] = await context.runtime.decrypt.decryptAndReconstruct({
      shares: kmsSigncryptedShares,
      tkmsPrivateKey,
      tkmsVersion: fhevmContext.tkmsVersion,
    });

    return orderedDecryptedHandles;
  } finally {
    tkmsPrivateKey.free();
  }
}
