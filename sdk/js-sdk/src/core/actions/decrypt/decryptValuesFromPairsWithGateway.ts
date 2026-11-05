import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { TypedValue } from '../../types/primitives.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../../types/relayer.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { TransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import { decryptValuesFromPairsWithGateway as decryptValuesFromPairsWithGateway_ } from '../../kms/decryptValuesFromPairsWithGateway.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptValuesFromPairsWithGatewayParameters = {
  readonly pairs: ReadonlyArray<{
    readonly encryptedValue: EncryptedValueLike;
    readonly contractAddress: string;
  }>;
  readonly transportKeyPair: TransportKeyPair;
  readonly signedPermit: SignedDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | RelayerDelegatedUserDecryptOptions | undefined;
};

export type DecryptValuesFromPairsWithGatewayReturnType = readonly TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptValuesFromPairsWithGateway(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptValuesFromPairsWithGatewayParameters,
): Promise<DecryptValuesFromPairsWithGatewayReturnType> {
  const { pairs, ...rest } = parameters;

  const ownerAddress = addressToChecksummedAddress(parameters.signedPermit.encryptedDataOwnerAddress);
  const sanitizedPairs = pairs.map((pair) => {
    assertIsAddress(pair.contractAddress, {});
    return {
      handle: toFhevmHandle(pair.encryptedValue),
      contractAddress: addressToChecksummedAddress(pair.contractAddress),
      ownerAddress,
    };
  });

  const fhevmContext = await initPublicAction(fhevm);

  return decryptValuesFromPairsWithGateway_(fhevm, {
    ...rest,
    pairs: sanitizedPairs,
    fhevmContext,
  });
}
