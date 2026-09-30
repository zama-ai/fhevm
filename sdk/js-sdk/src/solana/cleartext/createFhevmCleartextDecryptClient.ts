import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaClientParameters } from '../clients/createFhevmBaseClient.js';
import type { FhevmSolanaDecryptClient } from '../clients/createFhevmDecryptClient.js';
import type { FhevmSolanaPublicDecryptClient } from '../clients/createFhevmPublicDecryptClient.js';
import type { SolanaDecryptTrust } from '../clients/decorators/permitDecrypt.js';
import type { SolanaLeafProofReader } from './leafProofs.js';
import { withPermitDecrypt } from '../clients/createFhevmDecryptClient.js';
import { solanaHostProgram } from '../clients/createFhevmBaseClient.js';
import { createSolanaPublicDecryptClient } from '../clients/createFhevmPublicDecryptClient.js';
import { cleartextPublicDecryptCertifier, cleartextUserDecryptExecution } from './decrypt.js';
import { createSolanaLeafRecord } from './leafRecord.js';
import { getCleartextSolanaRuntime } from './runtime.js';

/**
 * Creates a public decrypt client for a cleartext host: certificates are signed by the cleartext
 * KMS key over the plaintexts the host recorded. Same parameters and actions as
 * `createFhevmPublicDecryptClient`.
 */
export function createFhevmCleartextPublicDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C>,
): FhevmSolanaPublicDecryptClient<C> {
  return publicDecryptClient(parameters, leafRecordOf(parameters));
}

/**
 * Creates the private and public decrypt client for a cleartext host: permits and requests are
 * built and admitted as in production, then answered with the plaintexts the host recorded. Same
 * parameters and actions as `createFhevmDecryptClient`.
 */
export function createFhevmCleartextDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C> & { readonly trust: SolanaDecryptTrust },
): FhevmSolanaDecryptClient<C> {
  const readLeafProofs = leafRecordOf(parameters);
  return withPermitDecrypt(
    publicDecryptClient(parameters, readLeafProofs),
    parameters.trust,
    getCleartextSolanaRuntime(),
    cleartextUserDecryptExecution(parameters.rpc, parameters.chain, parameters.trust, readLeafProofs),
  );
}

/** The client's own leaf record, standing in for the coprocessors' the Connector reads. */
function leafRecordOf({ rpc, chain }: SolanaClientParameters): SolanaLeafProofReader {
  return createSolanaLeafRecord(rpc, solanaHostProgram(chain));
}

function publicDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C>,
  readLeafProofs: SolanaLeafProofReader,
): FhevmSolanaPublicDecryptClient<C> {
  return createSolanaPublicDecryptClient(
    parameters,
    getCleartextSolanaRuntime(),
    cleartextPublicDecryptCertifier(parameters.rpc, parameters.chain, readLeafProofs),
  );
}
