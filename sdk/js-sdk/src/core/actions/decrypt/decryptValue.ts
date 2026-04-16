/* eslint-disable @typescript-eslint/unified-signatures */
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { TypedValue } from '../../types/primitives.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../../types/relayer.js';
import type {
  SignedDelegatedDecryptionPermit,
  SignedSelfDecryptionPermit,
} from '../../types/signedDecryptionPermit.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { TransportKeypair } from '../../kms/TransportKeypair-p.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import { decryptValuesFromPairs as decryptValuesFromPairs_ } from '../../kms/decryptValuesFromPairs.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';

////////////////////////////////////////////////////////////////////////////////

type DecryptValueParametersBase = {
  readonly encryptedValue: EncryptedValueLike;
  readonly contractAddress: string;
  readonly transportKeypair: TransportKeypair;
};

export type DecryptSelfValueParameters = DecryptValueParametersBase & {
  readonly signedPermit: SignedSelfDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | undefined;
};

export type DecryptDelegatedValueParameters = DecryptValueParametersBase & {
  readonly signedPermit: SignedDelegatedDecryptionPermit;
  readonly options?: RelayerDelegatedUserDecryptOptions | undefined;
};

export type DecryptValueReturnType = TypedValue;

////////////////////////////////////////////////////////////////////////////////

export async function decryptValue(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptSelfValueParameters,
): Promise<DecryptValueReturnType>;

export async function decryptValue(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptDelegatedValueParameters,
): Promise<DecryptValueReturnType>;

export async function decryptValue(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptSelfValueParameters | DecryptDelegatedValueParameters,
): Promise<DecryptValueReturnType> {
  const { encryptedValue, contractAddress, ...rest } = parameters;

  assertIsAddress(contractAddress, {});
  const sanitizedContractAddress = addressToChecksummedAddress(contractAddress);

  const sanitizedPairs = [
    {
      handle: toFhevmHandle(encryptedValue),
      contractAddress: sanitizedContractAddress,
    },
  ];

  const typedValues = await decryptValuesFromPairs_(fhevm, { ...rest, pairs: sanitizedPairs });

  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  return typedValues[0]!;
}
