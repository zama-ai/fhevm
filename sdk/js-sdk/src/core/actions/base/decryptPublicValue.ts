import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type {
  DecryptPublicValueWithGatewayParameters,
  DecryptPublicValueWithGatewayReturnType,
} from './decryptPublicValueWithGateway.js';
import { decryptPublicValueWithGateway } from './decryptPublicValueWithGateway.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptPublicValueParameters = DecryptPublicValueWithGatewayParameters;

export type DecryptPublicValueReturnType = DecryptPublicValueWithGatewayReturnType;

////////////////////////////////////////////////////////////////////////////////

export async function decryptPublicValue(
  fhevm: Fhevm<FhevmChain>,
  parameters: DecryptPublicValueParameters,
): Promise<DecryptPublicValueReturnType> {
  return decryptPublicValueWithGateway(fhevm, parameters);
}
