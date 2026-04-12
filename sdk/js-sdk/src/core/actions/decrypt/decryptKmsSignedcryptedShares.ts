import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  e2eTransportKeypairToTkmsPrivateKey,
  type E2eTransportKeypair,
} from '../../kms/E2eTransportKeypair-p.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { KmsSigncryptedShares } from '../../types/kms.js';
import type { ClearValue } from '../../types/encryptedTypes.js';

////////////////////////////////////////////////////////////////////////////////
// decryptKmsSignedcryptedShares (with privateKey)
////////////////////////////////////////////////////////////////////////////////

export type DecryptKmsSignedcryptedSharesParameters = {
  readonly kmsSigncryptedShares: KmsSigncryptedShares;
  readonly e2eTransportKeypair: E2eTransportKeypair;
};

export type DecryptKmsSignedcryptedSharesReturnType = readonly ClearValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptKmsSignedcryptedShares(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptKmsSignedcryptedSharesParameters,
): Promise<DecryptKmsSignedcryptedSharesReturnType> {
  const { e2eTransportKeypair, kmsSigncryptedShares } = parameters;

  // also validates `e2eTransportKeypair`
  const tkmsPrivateKey = await e2eTransportKeypairToTkmsPrivateKey(
    fhevm,
    e2eTransportKeypair,
  );

  // Using the `KmsSigncryptedShares` decrypt and reconstruct clear values
  const orderedDecryptedHandles: readonly ClearValue[] =
    await fhevm.runtime.decrypt.decryptAndReconstruct({
      shares: kmsSigncryptedShares,
      tkmsPrivateKey,
    });

  return orderedDecryptedHandles;
}
