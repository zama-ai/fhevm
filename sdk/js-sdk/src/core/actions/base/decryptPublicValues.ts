import type { RelayerPublicDecryptOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import type { TypedValue } from '../../types/primitives.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';
import { publicDecrypt as publicDecrypt_ } from '../../kms/publicDecrypt.js';
import { clearValueToTypedValue } from '../../handle/ClearValue.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptPublicValuesParameters = {
  readonly encryptedValues: readonly EncryptedValueLike[];
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

export type DecryptPublicValuesReturnType = TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptPublicValues(
  fhevm: Fhevm<FhevmChain>,
  parameters: DecryptPublicValuesParameters,
): Promise<DecryptPublicValuesReturnType> {
  const handles = parameters.encryptedValues.map(toFhevmHandle);

  const fhevmContext = await initPublicAction(fhevm);

  const originToken = Symbol('decryptPublicValues');
  const res = await publicDecrypt_(fhevm, { ...parameters, handles, originToken, fhevmContext });

  return res.orderedClearValues.map((cv) => clearValueToTypedValue(cv, originToken));
}
