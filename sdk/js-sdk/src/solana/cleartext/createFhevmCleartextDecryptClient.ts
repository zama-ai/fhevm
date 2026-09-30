import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaClientParameters } from '../clients/createFhevmBaseClient.js';
import type { FhevmSolanaDecryptClient } from '../clients/createFhevmDecryptClient.js';
import type { FhevmSolanaPublicDecryptClient } from '../clients/createFhevmPublicDecryptClient.js';
import type { SolanaDecryptTrust } from '../clients/decorators/permitDecrypt.js';
import { withPermitDecrypt } from '../clients/createFhevmDecryptClient.js';
import { createSolanaPublicDecryptClient } from '../clients/createFhevmPublicDecryptClient.js';
import { cleartextPublicDecryptCertifier, cleartextUserDecryptExecution } from './decrypt.js';
import { getCleartextSolanaRuntime } from './runtime.js';

/**
 * Creates a public decrypt client for a cleartext host: certificates are signed by the cleartext
 * KMS key over the plaintexts the host recorded. Same parameters and actions as
 * `createFhevmPublicDecryptClient`.
 */
export function createFhevmCleartextPublicDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C>,
): FhevmSolanaPublicDecryptClient<C> {
  return createSolanaPublicDecryptClient(
    parameters,
    getCleartextSolanaRuntime(),
    cleartextPublicDecryptCertifier(parameters.rpc, parameters.chain),
  );
}

/**
 * Creates the private and public decrypt client for a cleartext host: permits and requests are
 * built and admitted as in production, then answered with the plaintexts the host recorded. Same
 * parameters and actions as `createFhevmDecryptClient`.
 */
export function createFhevmCleartextDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C> & { readonly trust: SolanaDecryptTrust },
): FhevmSolanaDecryptClient<C> {
  return withPermitDecrypt(
    createFhevmCleartextPublicDecryptClient(parameters),
    parameters.trust,
    getCleartextSolanaRuntime(),
    cleartextUserDecryptExecution(parameters.rpc, parameters.chain, parameters.trust),
  );
}
