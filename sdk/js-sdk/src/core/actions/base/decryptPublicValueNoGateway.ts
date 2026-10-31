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

export type DecryptPublicValueNoGatewayParameters = {
  readonly encryptedValue: EncryptedValueLike;
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

export type DecryptPublicValueNoGatewayReturnType = TypedValue;

////////////////////////////////////////////////////////////////////////////////

export async function decryptPublicValueNoGateway(
  fhevm: Fhevm<FhevmChain>,
  parameters: DecryptPublicValueNoGatewayParameters,
): Promise<DecryptPublicValueNoGatewayReturnType> {
  const handle = toFhevmHandle(parameters.encryptedValue);

  const fhevmContext = await initPublicAction(fhevm);

  const originToken = Symbol('decryptPublicValues');
  const res = await publicDecryptNoGateway_(fhevm, { ...parameters, handles: [handle], originToken, fhevmContext });

  return clearValueToTypedValue(res.orderedClearValues[0], originToken);
}
