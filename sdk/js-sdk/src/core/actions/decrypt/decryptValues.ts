import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type {
  DecryptValuesWithGatewayParameters,
  DecryptValuesWithGatewayReturnType,
} from './decryptValuesWithGateway.js';
import { decryptValuesWithGateway } from './decryptValuesWithGateway.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptValuesParameters = DecryptValuesWithGatewayParameters;

export type DecryptValuesReturnType = DecryptValuesWithGatewayReturnType;

////////////////////////////////////////////////////////////////////////////////

export async function decryptValues(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptValuesParameters,
): Promise<DecryptValuesReturnType> {
  return decryptValuesWithGateway(fhevm, parameters);
}
