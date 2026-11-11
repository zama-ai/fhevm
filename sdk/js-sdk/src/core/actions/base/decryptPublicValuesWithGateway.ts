import type { RelayerPublicDecryptOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import type { TypedValue } from '../../types/primitives.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';
import { publicDecryptWithGateway as publicDecryptWithGateway_ } from '../../kms/publicDecryptWithGateway.js';
import { clearValueToTypedValue } from '../../handle/ClearValue.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptPublicValuesWithGatewayParameters = {
  readonly encryptedValues: readonly EncryptedValueLike[];
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

export type DecryptPublicValuesWithGatewayReturnType = TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptPublicValuesWithGateway(
  fhevm: Fhevm<FhevmChain>,
  parameters: DecryptPublicValuesWithGatewayParameters,
): Promise<DecryptPublicValuesWithGatewayReturnType> {
  const handles = parameters.encryptedValues.map(toFhevmHandle);

  const fhevmContext = await initPublicAction(fhevm);

  const originToken = Symbol('decryptPublicValues');
  const res = await publicDecryptWithGateway_(fhevm, { ...parameters, handles, originToken, fhevmContext });

  return res.orderedClearValues.map((cv) => clearValueToTypedValue(cv, originToken));
}
