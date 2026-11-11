import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { TypedValue } from '../../types/primitives.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../../types/relayer.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { TransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import { decryptValuesFromPairsNoGateway as decryptValuesFromPairsNoGateway_ } from '../../kms/decryptValuesFromPairsNoGateway.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptValuesNoGatewayParameters = {
  readonly encryptedValues: readonly EncryptedValueLike[];
  readonly contractAddress: string;
  readonly transportKeyPair: TransportKeyPair;
  readonly signedPermit: SignedDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | RelayerDelegatedUserDecryptOptions | undefined;
};

export type DecryptValuesNoGatewayReturnType = readonly TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptValuesNoGateway(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptValuesNoGatewayParameters,
): Promise<DecryptValuesNoGatewayReturnType> {
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

  return decryptValuesFromPairsNoGateway_(fhevm, {
    ...rest,
    pairs: sanitizedPairs,
    fhevmContext,
  });
}
