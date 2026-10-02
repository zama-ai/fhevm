import {
  decryptPublicValue,
  decryptPublicValues,
  type SolanaDecryptPublicValueParameters,
} from '../actions/decryptPublicValue.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { SolanaPublicDecryptCertifier } from '../actions/publicDecryptCertificate.js';
import { publicDecryptCertificate } from '../actions/publicDecryptCertificate.js';
import type { FhevmSolanaBaseClient, SolanaClientParameters } from './createFhevmBaseClient.js';
import type { SolanaPublicDecryptActions } from './decorators/publicDecrypt.js';
import { createSolanaCore, solanaClientSurface } from './createFhevmBaseClient.js';
import { getSolanaRuntime } from '../internal/runtime.js';

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
  const runtime = getSolanaRuntime();
  return createSolanaPublicDecryptClient(parameters, runtime, (certificate) =>
    publicDecryptCertificate({ chain: parameters.chain, runtime }, certificate),
  );
}

/** The public decrypt client over a given runtime and certifier; the cleartext client supplies both. */
export function createSolanaPublicDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C>,
  runtime: FhevmRuntime,
  certify: SolanaPublicDecryptCertifier,
): FhevmSolanaPublicDecryptClient<C> {
  const core = createSolanaCore(parameters, runtime);
  return Object.assign(solanaClientSurface(core, parameters), {
    publicDecryptCertificate: certify,
    decryptPublicValues: (input: Parameters<typeof decryptPublicValues>[1]) =>
      decryptPublicValues(parameters, input, certify),
    decryptPublicValue: (input: SolanaDecryptPublicValueParameters) => decryptPublicValue(parameters, input, certify),
  });
}
