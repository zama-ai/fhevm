import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { TypedValue } from '../../types/primitives.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../../types/relayer.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { TransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import { decryptValuesFromPairs as decryptValuesFromPairs_ } from '../../kms/decryptValuesFromPairs.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptValuesParameters = {
  readonly encryptedValues: readonly EncryptedValueLike[];
  readonly contractAddress: string;
  readonly transportKeyPair: TransportKeyPair;
  readonly signedPermit: SignedDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | RelayerDelegatedUserDecryptOptions | undefined;
};

export type DecryptValuesReturnType = readonly TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptValues(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptValuesParameters,
): Promise<DecryptValuesReturnType> {
  const { encryptedValues, contractAddress, ...rest } = parameters;

  assertIsAddress(contractAddress, {});
  const sanitizedContractAddress = addressToChecksummedAddress(contractAddress);

  const ownerAddress = addressToChecksummedAddress(parameters.signedPermit.encryptedDataOwnerAddress);
  const sanitizedPairs = encryptedValues.map((ev) => {
    return {
      handle: toFhevmHandle(ev),
      contractAddress: sanitizedContractAddress,
      ownerAddress,
    };
  });

  // context is not needed
  const fhevmContext = await initPublicAction(fhevm);

  return decryptValuesFromPairs_(fhevm, {
    ...rest,
    pairs: sanitizedPairs,
    fhevmContext,
  });
}
