import {
  decryptPublicValue,
  decryptPublicValues,
  type SolanaDecryptPublicValueParameters,
  type SolanaDecryptPublicValuesParameters,
} from '../actions/decryptPublicValue.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { SolanaPublicDecryptCertifier } from '../actions/publicDecryptCertificate.js';
import { publicDecryptCertificate, singlePublicDecryptCertificate } from '../actions/publicDecryptCertificate.js';
import type { FhevmSolanaBaseClient, SolanaClientParameters } from './createFhevmBaseClient.js';
import type { SolanaPublicDecryptActions } from './decorators/publicDecrypt.js';
import { createSolanaCore, solanaClientSurface } from './createFhevmBaseClient.js';
import { getSolanaRuntime } from '../internal/runtime.js';
import { createSolanaHostKmsReads, type SolanaHostKmsReads } from '../actions/hostKms.js';

export type FhevmSolanaPublicDecryptClient<C extends FhevmSolanaChain = FhevmSolanaChain> = FhevmSolanaBaseClient<C> &
  SolanaPublicDecryptActions & {
    readonly decryptPublicValues: (
      parameters: SolanaDecryptPublicValuesParameters,
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
  return createSolanaPublicDecryptClient(parameters, runtime, createSolanaHostKmsReads(parameters, runtime), (batch) =>
    publicDecryptCertificate({ chain: parameters.chain, runtime }, batch),
  );
}

/** The public decrypt client over a given runtime, host reads and certifier; the cleartext client supplies its own. */
export function createSolanaPublicDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C>,
  runtime: FhevmRuntime,
  host: SolanaHostKmsReads,
  certify: SolanaPublicDecryptCertifier,
): FhevmSolanaPublicDecryptClient<C> {
  const core = createSolanaCore(parameters, runtime);
  return Object.assign(solanaClientSurface(core, parameters), {
    publicDecryptCertificate: singlePublicDecryptCertificate(parameters, host, certify),
    decryptPublicValues: (input: SolanaDecryptPublicValuesParameters) =>
      decryptPublicValues(parameters, host, input, certify),
    decryptPublicValue: (input: SolanaDecryptPublicValueParameters) =>
      decryptPublicValue(parameters, host, input, certify),
  });
}
