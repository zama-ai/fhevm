import {
  decryptPublicValue,
  decryptPublicValues,
  type SolanaDecryptPublicValueParameters,
} from '../actions/decryptPublicValue.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmSolanaBaseClient, SolanaClientParameters } from './createFhevmBaseClient.js';
import type { SolanaPublicDecryptActions } from './decorators/publicDecrypt.js';
import { createSolanaCore, solanaClientSurface } from './createFhevmBaseClient.js';
import { solanaPublicDecryptActions } from './decorators/publicDecrypt.js';

export type FhevmSolanaPublicDecryptClient<C extends FhevmSolanaChain = FhevmSolanaChain> = FhevmSolanaBaseClient<C> &
  SolanaPublicDecryptActions & {
    readonly decryptPublicValues: (
      parameters: Parameters<typeof decryptPublicValues>[1],
    ) => ReturnType<typeof decryptPublicValues>;
    readonly decryptPublicValue: (
      parameters: SolanaDecryptPublicValueParameters,
    ) => ReturnType<typeof decryptPublicValue>;
  };

/** Creates a signer-free public decrypt client. */
export function createFhevmPublicDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C>,
): FhevmSolanaPublicDecryptClient<C> {
  const core = createSolanaCore(parameters).extend(solanaPublicDecryptActions);
  return Object.assign(solanaClientSurface(core, parameters), {
    publicDecryptCertificate: core.publicDecryptCertificate,
    decryptPublicValues: (input: Parameters<typeof decryptPublicValues>[1]) => decryptPublicValues(parameters, input),
    decryptPublicValue: (input: SolanaDecryptPublicValueParameters) => decryptPublicValue(parameters, input),
  });
}
