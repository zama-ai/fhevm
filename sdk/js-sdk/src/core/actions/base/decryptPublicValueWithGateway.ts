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

export type DecryptPublicValueWithGatewayParameters = {
  readonly encryptedValue: EncryptedValueLike;
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

export type DecryptPublicValueWithGatewayReturnType = TypedValue;

////////////////////////////////////////////////////////////////////////////////

export async function decryptPublicValueWithGateway(
  fhevm: Fhevm<FhevmChain>,
  parameters: DecryptPublicValueWithGatewayParameters,
): Promise<DecryptPublicValueWithGatewayReturnType> {
  const handle = toFhevmHandle(parameters.encryptedValue);

  const fhevmContext = await initPublicAction(fhevm);

  const originToken = Symbol('decryptPublicValues');
  const res = await publicDecryptWithGateway_(fhevm, { ...parameters, handles: [handle], originToken, fhevmContext });

  return clearValueToTypedValue(res.orderedClearValues[0], originToken);
}
