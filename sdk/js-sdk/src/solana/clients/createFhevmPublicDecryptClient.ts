import {
  decryptPublicValue,
  decryptPublicValues,
  type SolanaDecryptPublicValueParameters,
} from '../actions/decryptPublicValue.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { SolanaPublicDecryptCertifier } from '../actions/publicDecryptCertificate.js';
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
  return createSolanaPublicDecryptClient(parameters);
}

/** The public decrypt client over a given runtime and certifier; the cleartext client supplies both. */
export function createSolanaPublicDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C>,
  runtime?: FhevmRuntime,
  certify?: SolanaPublicDecryptCertifier,
): FhevmSolanaPublicDecryptClient<C> {
  const core = createSolanaCore(parameters, runtime);
  return Object.assign(solanaClientSurface(core, parameters), {
    ...solanaPublicDecryptActions(parameters.chain, core.runtime, certify),
    decryptPublicValues: (input: Parameters<typeof decryptPublicValues>[1]) =>
      decryptPublicValues(parameters, input, certify),
    decryptPublicValue: (input: SolanaDecryptPublicValueParameters) => decryptPublicValue(parameters, input, certify),
  });
}
