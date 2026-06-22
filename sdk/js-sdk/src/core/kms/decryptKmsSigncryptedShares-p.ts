import type { WithDecrypt } from '../types/coreFhevmRuntime.js';
import type { KmsSigncryptedShares } from '../types/kms.js';
import type { ClearValue } from '../types/encryptedTypes-p.js';
import type { TransportKeyPair } from './TransportKeyPair-p.js';
import type { TkmsVersion } from '../../wasm/tkms/KmsLibApi.js';
import { transportKeyPairToTkmsPrivateKey } from './TransportKeyPair-p.js';

////////////////////////////////////////////////////////////////////////////////
// decryptKmsSigncryptedShares (with privateKey)
////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: WithDecrypt;
  readonly tkmsVersion: TkmsVersion;
};

type Parameters = {
  readonly kmsSigncryptedShares: KmsSigncryptedShares;
  readonly transportKeyPair: TransportKeyPair;
};

type ReturnType = readonly ClearValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptKmsSigncryptedShares(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { transportKeyPair: transportKeyPair, kmsSigncryptedShares } = parameters;

  if (context.tkmsVersion !== kmsSigncryptedShares.tkmsVersion) {
    throw new Error('TkmsVersion mismatch');
  }
  if (context.tkmsVersion !== transportKeyPair.tkmsVersion) {
    throw new Error('TkmsVersion mismatch');
  }

  // also validates `transportKeyPair`
  const tkmsPrivateKey = await transportKeyPairToTkmsPrivateKey(context, transportKeyPair);

  // Using the `KmsSigncryptedShares` decrypt and reconstruct clear values
  const orderedDecryptedHandles: readonly ClearValue[] = await context.runtime.decrypt.decryptAndReconstruct({
    shares: kmsSigncryptedShares,
    tkmsPrivateKey,
    tkmsVersion: context.tkmsVersion,
  });

  return orderedDecryptedHandles;
}
