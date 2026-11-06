import type { RelayerPublicDecryptOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import type { TypedValue } from '../../types/primitives.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';
import { publicDecryptNoGateway as publicDecryptNoGateway_ } from '../../kms/publicDecryptNoGateway.js';
import { clearValueToTypedValue } from '../../handle/ClearValue.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptPublicValuesNoGatewayParameters = {
  readonly encryptedValues: readonly EncryptedValueLike[];
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

export type DecryptPublicValuesNoGatewayReturnType = TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptPublicValuesNoGateway(
  fhevm: Fhevm<FhevmChain>,
  parameters: DecryptPublicValuesNoGatewayParameters,
): Promise<DecryptPublicValuesNoGatewayReturnType> {
  const handles = parameters.encryptedValues.map(toFhevmHandle);

  const fhevmContext = await initPublicAction(fhevm);

  const originToken = Symbol('decryptPublicValues');
  const res = await publicDecryptNoGateway_(fhevm, { ...parameters, handles, originToken, fhevmContext });

  return res.orderedClearValues.map((cv) => clearValueToTypedValue(cv, originToken));
}
