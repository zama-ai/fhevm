import type { RelayerPublicDecryptOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import type { BytesHex, TypedValue } from '../../types/primitives.js';
import type { HandleBytes32Hex } from '../../types/encryptedTypes-p.js';
import type { NonEmptyReadonlyArray } from '../../types/utils.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';
import { publicDecryptNoGateway as publicDecryptNoGateway_ } from '../../kms/publicDecryptNoGateway.js';
import { clearValueToTypedValue } from '../../handle/ClearValue.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptPublicValuesWithSignaturesNoGatewayParameters = {
  readonly encryptedValues: readonly EncryptedValueLike[];
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

export type DecryptPublicValuesWithSignaturesNoGatewayReturnType = {
  readonly clearValues: NonEmptyReadonlyArray<TypedValue>;
  readonly checkSignaturesArgs: {
    readonly handlesList: NonEmptyReadonlyArray<HandleBytes32Hex>;
    readonly abiEncodedCleartexts: BytesHex;
    readonly decryptionProof: BytesHex;
  };
};

////////////////////////////////////////////////////////////////////////////////

export async function decryptPublicValuesWithSignaturesNoGateway(
  fhevm: Fhevm<FhevmChain>,
  parameters: DecryptPublicValuesWithSignaturesNoGatewayParameters,
): Promise<DecryptPublicValuesWithSignaturesNoGatewayReturnType> {
  const handles = parameters.encryptedValues.map(toFhevmHandle);

  const fhevmContext = await initPublicAction(fhevm);

  const originToken = Symbol('decryptPublicValues');
  const res = await publicDecryptNoGateway_(fhevm, { ...parameters, handles, originToken, fhevmContext });

  const typedValues = res.orderedClearValues.map((cv) =>
    clearValueToTypedValue(cv, originToken),
  ) as unknown as NonEmptyReadonlyArray<TypedValue>;

  return Object.freeze({
    clearValues: typedValues,
    checkSignaturesArgs: Object.freeze({
      handlesList: Object.freeze(
        handles.map((h) => h.bytes32Hex),
      ) as unknown as NonEmptyReadonlyArray<HandleBytes32Hex>,
      abiEncodedCleartexts: res.orderedAbiEncodedClearValues,
      decryptionProof: res.decryptionProof,
    }),
  });
}
