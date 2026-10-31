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

export type DecryptValuesFromPairsNoGatewayParameters = {
  readonly pairs: ReadonlyArray<{
    readonly encryptedValue: EncryptedValueLike;
    readonly contractAddress: string;
  }>;
  readonly transportKeyPair: TransportKeyPair;
  readonly signedPermit: SignedDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | RelayerDelegatedUserDecryptOptions | undefined;
};

export type DecryptValuesFromPairsNoGatewayReturnType = readonly TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptValuesFromPairsNoGateway(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptValuesFromPairsNoGatewayParameters,
): Promise<DecryptValuesFromPairsNoGatewayReturnType> {
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

  return decryptValuesFromPairsNoGateway_(fhevm, {
    ...rest,
    pairs: sanitizedPairs,
    fhevmContext,
  });
}
