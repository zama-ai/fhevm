import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type {
  DecryptValuesFromPairsWithGatewayParameters,
  DecryptValuesFromPairsWithGatewayReturnType,
} from './decryptValuesFromPairsWithGateway.js';
import { decryptValuesFromPairsWithGateway } from './decryptValuesFromPairsWithGateway.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptValuesFromPairsParameters = DecryptValuesFromPairsWithGatewayParameters;

export type DecryptValuesFromPairsReturnType = DecryptValuesFromPairsWithGatewayReturnType;

////////////////////////////////////////////////////////////////////////////////

export async function decryptValuesFromPairs(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptValuesFromPairsParameters,
): Promise<DecryptValuesFromPairsReturnType> {
  return decryptValuesFromPairsWithGateway(fhevm, parameters);
}
