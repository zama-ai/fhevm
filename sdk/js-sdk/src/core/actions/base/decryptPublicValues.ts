import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type {
  DecryptPublicValuesWithGatewayParameters,
  DecryptPublicValuesWithGatewayReturnType,
} from './decryptPublicValuesWithGateway.js';
import { decryptPublicValuesWithGateway } from './decryptPublicValuesWithGateway.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptPublicValuesParameters = DecryptPublicValuesWithGatewayParameters;

export type DecryptPublicValuesReturnType = DecryptPublicValuesWithGatewayReturnType;

////////////////////////////////////////////////////////////////////////////////

export async function decryptPublicValues(
  fhevm: Fhevm<FhevmChain>,
  parameters: DecryptPublicValuesParameters,
): Promise<DecryptPublicValuesReturnType> {
  return decryptPublicValuesWithGateway(fhevm, parameters);
}
