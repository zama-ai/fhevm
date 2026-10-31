import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type {
  DecryptPublicValuesWithSignaturesWithGatewayParameters,
  DecryptPublicValuesWithSignaturesWithGatewayReturnType,
} from './decryptPublicValuesWithSignaturesWithGateway.js';
import { decryptPublicValuesWithSignaturesWithGateway } from './decryptPublicValuesWithSignaturesWithGateway.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptPublicValuesWithSignaturesParameters = DecryptPublicValuesWithSignaturesWithGatewayParameters;

export type DecryptPublicValuesWithSignaturesReturnType = DecryptPublicValuesWithSignaturesWithGatewayReturnType;

////////////////////////////////////////////////////////////////////////////////

export async function decryptPublicValuesWithSignatures(
  fhevm: Fhevm<FhevmChain>,
  parameters: DecryptPublicValuesWithSignaturesParameters,
): Promise<DecryptPublicValuesWithSignaturesReturnType> {
  return decryptPublicValuesWithSignaturesWithGateway(fhevm, parameters);
}
