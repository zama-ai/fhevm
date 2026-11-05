import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type {
  DecryptValueWithGatewayParameters,
  DecryptValueWithGatewayReturnType,
} from './decryptValueWithGateway.js';
import { decryptValueWithGateway } from './decryptValueWithGateway.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptValueParameters = DecryptValueWithGatewayParameters;

export type DecryptValueReturnType = DecryptValueWithGatewayReturnType;

////////////////////////////////////////////////////////////////////////////////

export async function decryptValue(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptValueParameters,
): Promise<DecryptValueReturnType> {
  return decryptValueWithGateway(fhevm, parameters);
}
