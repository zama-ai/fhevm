import type { WithDecrypt } from '../types/coreFhevmRuntime.js';
import type { KmsSigncryptedShares } from '../types/kms.js';
import type { ClearValue } from '../types/encryptedTypes-p.js';
import type { TransportKeyPair } from './TransportKeyPair-p.js';
import { transportKeyPairToTkmsPrivateKey } from './TransportKeyPair-p.js';

////////////////////////////////////////////////////////////////////////////////
// decryptKmsSignedcryptedShares (with privateKey)
////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: WithDecrypt;
};

type Parameters = {
  readonly kmsSigncryptedShares: KmsSigncryptedShares;
  readonly transportKeyPair: TransportKeyPair;
};

type ReturnType = readonly ClearValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptKmsSignedcryptedShares(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { transportKeyPair: transportKeyPair, kmsSigncryptedShares } = parameters;

  // also validates `transportKeyPair`
  const tkmsPrivateKey = await transportKeyPairToTkmsPrivateKey(context, transportKeyPair);

  // Using the `KmsSigncryptedShares` decrypt and reconstruct clear values
  const orderedDecryptedHandles: readonly ClearValue[] = await context.runtime.decrypt.decryptAndReconstruct({
    shares: kmsSigncryptedShares,
    tkmsPrivateKey,
  });

  return orderedDecryptedHandles;
}
