////////////////////////////////////////////////////////////////////////////////

import type { WithDecrypt } from '../types/coreFhevmRuntime.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { KmsSigncryptedShares } from '../types/kms.js';
import type { ChecksummedAddress, TypedValue } from '../types/primitives.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../types/relayer.js';
import type { SignedDelegatedDecryptionPermit, SignedSelfDecryptionPermit } from '../types/signedDecryptionPermit.js';
import type { TransportKeypair } from './TransportKeypair-p.js';
import { decryptKmsSignedcryptedShares } from './decryptKmsSignedcryptedShares-p.js';
import { fetchKmsSignedcryptedShares } from './fetchKmsSignedcryptedShares-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: WithDecrypt;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters =
  | {
      readonly pairs: ReadonlyArray<{
        readonly handle: Handle;
        readonly contractAddress: ChecksummedAddress;
      }>;
      readonly signedPermit: SignedSelfDecryptionPermit;
      readonly transportKeypair: TransportKeypair;
      readonly options?: RelayerUserDecryptOptions | undefined;
    }
  | {
      readonly pairs: ReadonlyArray<{
        readonly handle: Handle;
        readonly contractAddress: ChecksummedAddress;
      }>;
      readonly signedPermit: SignedDelegatedDecryptionPermit;
      readonly transportKeypair: TransportKeypair;
      readonly options?: RelayerDelegatedUserDecryptOptions | undefined;
    };

export type ReturnType = readonly TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptValuesFromPairs(fhevm: Context, parameters: Parameters): Promise<ReturnType> {
  const { transportKeypair } = parameters;

  const kmsSigncryptedShares: KmsSigncryptedShares = await fetchKmsSignedcryptedShares(fhevm, parameters);

  // Using the `KmsSigncryptedShares` decrypt and reconstruct clear values
  return decryptKmsSignedcryptedShares(fhevm, {
    kmsSigncryptedShares,
    transportKeypair,
  });
}
