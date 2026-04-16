import type { RelayerPublicDecryptOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import type { TypedValue } from '../../types/primitives.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';
import { publicDecrypt as publicDecrypt_ } from '../../kms/publicDecrypt.js';
import { clearValueToTypedValue } from '../../handle/ClearValue.js';

////////////////////////////////////////////////////////////////////////////////

export type ReadPublicValuesParameters = {
  readonly encryptedValues: readonly EncryptedValueLike[];
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

export type ReadPublicValuesReturnType = TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function readPublicValues(
  fhevm: Fhevm<FhevmChain>,
  parameters: ReadPublicValuesParameters,
): Promise<ReadPublicValuesReturnType> {
  const handles = parameters.encryptedValues.map(toFhevmHandle);

  const originToken = Symbol('readPublicValues');
  const res = await publicDecrypt_(fhevm, { ...parameters, handles, originToken });

  return res.orderedClearValues.map((cv) => clearValueToTypedValue(cv, originToken));
}
