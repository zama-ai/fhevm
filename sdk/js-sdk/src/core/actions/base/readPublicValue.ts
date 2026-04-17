import type { RelayerPublicDecryptOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import type { TypedValue } from '../../types/primitives.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';
import { publicDecrypt as publicDecrypt_ } from '../../kms/publicDecrypt.js';
import { clearValueToTypedValue } from '../../handle/ClearValue.js';

////////////////////////////////////////////////////////////////////////////////

export type ReadPublicValueParameters = {
  readonly encryptedValue: EncryptedValueLike;
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

export type ReadPublicValueReturnType = TypedValue;

////////////////////////////////////////////////////////////////////////////////

export async function readPublicValue(
  fhevm: Fhevm<FhevmChain>,
  parameters: ReadPublicValueParameters,
): Promise<ReadPublicValueReturnType> {
  const handle = toFhevmHandle(parameters.encryptedValue);

  const originToken = Symbol('readPublicValues');
  const res = await publicDecrypt_(fhevm, { ...parameters, handles: [handle], originToken });

  return clearValueToTypedValue(res.orderedClearValues[0], originToken);
}
