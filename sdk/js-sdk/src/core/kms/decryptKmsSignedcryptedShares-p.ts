import type { WithDecrypt } from '../types/coreFhevmRuntime.js';
import type { KmsSigncryptedShares } from '../types/kms.js';
import type { ClearValue } from '../types/encryptedTypes-p.js';
import type { TransportKeypair } from './TransportKeypair-p.js';
import { transportKeypairToTkmsPrivateKey } from './TransportKeypair-p.js';

////////////////////////////////////////////////////////////////////////////////
// decryptKmsSignedcryptedShares (with privateKey)
////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: WithDecrypt;
};

type Parameters = {
  readonly kmsSigncryptedShares: KmsSigncryptedShares;
  readonly transportKeypair: TransportKeypair;
};

type ReturnType = readonly ClearValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptKmsSignedcryptedShares(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { transportKeypair, kmsSigncryptedShares } = parameters;

  // also validates `transportKeypair`
  const tkmsPrivateKey = await transportKeypairToTkmsPrivateKey(context, transportKeypair);

  // Using the `KmsSigncryptedShares` decrypt and reconstruct clear values
  const orderedDecryptedHandles: readonly ClearValue[] = await context.runtime.decrypt.decryptAndReconstruct({
    shares: kmsSigncryptedShares,
    tkmsPrivateKey,
  });

  return orderedDecryptedHandles;
}
