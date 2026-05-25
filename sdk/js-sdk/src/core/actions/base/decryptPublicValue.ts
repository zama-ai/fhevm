import type { RelayerPublicDecryptOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import type { TypedValue } from '../../types/primitives.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';
import { publicDecrypt as publicDecrypt_ } from '../../kms/publicDecrypt.js';
import { clearValueToTypedValue } from '../../handle/ClearValue.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptPublicValueParameters = {
  readonly encryptedValue: EncryptedValueLike;
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

export type DecryptPublicValueReturnType = TypedValue;

////////////////////////////////////////////////////////////////////////////////

export async function decryptPublicValue(
  fhevm: Fhevm<FhevmChain>,
  parameters: DecryptPublicValueParameters,
): Promise<DecryptPublicValueReturnType> {
  const handle = toFhevmHandle(parameters.encryptedValue);

  const originToken = Symbol('decryptPublicValues');
  const res = await publicDecrypt_(fhevm, { ...parameters, handles: [handle], originToken });

  return clearValueToTypedValue(res.orderedClearValues[0], originToken);
}
