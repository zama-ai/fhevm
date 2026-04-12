import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type {
  RelayerDelegatedUserDecryptOptions,
  RelayerUserDecryptOptions,
} from '../../types/relayer.js';
import {
  assertIsE2eTransportKeypair,
  type E2eTransportKeypair,
} from '../../kms/E2eTransportKeypair-p.js';
import type {
  SignedDelegatedDecryptionPermit,
  SignedSelfDecryptionPermit,
} from '../../types/signedDecryptionPermit.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { KmsSigncryptedShares } from '../../types/kms.js';
import { fetchKmsSignedcryptedShares } from '../base/fetchKmsSignedcryptedShares.js';
import { assertIsSignedDecryptionPermit } from '../../kms/SignedDecryptionPermit-p.js';
import { decryptKmsSignedcryptedShares } from './decryptKmsSignedcryptedShares.js';
import type {
  ClearValue,
  EncryptedValueLike,
} from '../../types/encryptedTypes.js';

////////////////////////////////////////////////////////////////////////////////
// decrypt (with privateKey)
////////////////////////////////////////////////////////////////////////////////

type EncryptedValueEntry = {
  readonly encryptedValue: EncryptedValueLike;
  readonly contractAddress: ChecksummedAddress;
};

type EncryptedValues = EncryptedValueEntry | readonly EncryptedValueEntry[];

export type DecryptParameters =
  | {
      readonly encryptedValues: EncryptedValues;
      readonly signedPermit: SignedSelfDecryptionPermit;
      readonly e2eTransportKeypair: E2eTransportKeypair;
      readonly options?: RelayerUserDecryptOptions | undefined;
    }
  | {
      readonly encryptedValues: EncryptedValues;
      readonly signedPermit: SignedDelegatedDecryptionPermit;
      readonly e2eTransportKeypair: E2eTransportKeypair;
      readonly options?: RelayerDelegatedUserDecryptOptions | undefined;
    };

export type DecryptReturnType = readonly ClearValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decrypt(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptParameters,
): Promise<DecryptReturnType> {
  const { e2eTransportKeypair } = parameters;

  assertIsSignedDecryptionPermit(parameters.signedPermit, {});
  assertIsE2eTransportKeypair(parameters.e2eTransportKeypair, {});

  const kmsSigncryptedShares: KmsSigncryptedShares =
    await fetchKmsSignedcryptedShares(fhevm, parameters);

  // Using the `KmsSigncryptedShares` decrypt and reconstruct clear values
  const orderedDecryptedHandles: readonly ClearValue[] =
    await decryptKmsSignedcryptedShares(fhevm, {
      kmsSigncryptedShares,
      e2eTransportKeypair,
    });

  return orderedDecryptedHandles;
}
