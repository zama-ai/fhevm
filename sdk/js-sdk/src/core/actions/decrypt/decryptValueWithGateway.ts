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

export type DecryptValueWithGatewayParameters = {
  readonly encryptedValue: EncryptedValueLike;
  readonly contractAddress: string;
  readonly transportKeyPair: TransportKeyPair;
  readonly signedPermit: SignedDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | RelayerDelegatedUserDecryptOptions | undefined;
};

export type DecryptValueWithGatewayReturnType = TypedValue;

////////////////////////////////////////////////////////////////////////////////

export async function decryptValueWithGateway(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptValueWithGatewayParameters,
): Promise<DecryptValueWithGatewayReturnType> {
  const { encryptedValue, contractAddress, ...rest } = parameters;

  assertIsAddress(contractAddress, {});
  const sanitizedContractAddress = addressToChecksummedAddress(contractAddress);

  const sanitizedPairs = [
    {
      handle: toFhevmHandle(encryptedValue),
      contractAddress: sanitizedContractAddress,
      ownerAddress: addressToChecksummedAddress(parameters.signedPermit.encryptedDataOwnerAddress),
    },
  ];

  const fhevmContext = await initPublicAction(fhevm);

  const typedValues = await decryptValuesFromPairsWithGateway_(fhevm, {
    ...rest,
    pairs: sanitizedPairs,
    fhevmContext,
  });

  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  return typedValues[0]!;
}
