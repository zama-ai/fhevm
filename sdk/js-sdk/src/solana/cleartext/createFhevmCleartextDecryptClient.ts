import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaClientParameters } from '../clients/createFhevmBaseClient.js';
import type { FhevmSolanaDecryptClient } from '../clients/createFhevmDecryptClient.js';
import type { FhevmSolanaPublicDecryptClient } from '../clients/createFhevmPublicDecryptClient.js';
import type { SolanaLeafReader } from './leafRecord.js';
import { withPermitDecrypt } from '../clients/createFhevmDecryptClient.js';
import { solanaHostProgram } from '../clients/createFhevmBaseClient.js';
import { createSolanaPublicDecryptClient } from '../clients/createFhevmPublicDecryptClient.js';
import { cleartextPublicDecryptCertifier, cleartextUserDecryptExecution } from './decrypt.js';
import { createSolanaLeafRecord } from './leafRecord.js';
import { getCleartextSolanaRuntime } from './runtime.js';
import { createSolanaHostKmsReads, type SolanaHostKmsReads } from '../actions/hostKms.js';

/**
 * The leaf record a cleartext client reads, standing in for the coprocessors' Merkle proof route
 * that the Connector reads.
 * Pass one record from `createSolanaLeafRecord` to every client of a host, so each client reads
 * only the store writes the record has not seen yet. A client without one keeps its own.
 */
type CleartextLeafRecordParameters = { readonly readLeaves?: SolanaLeafReader };

/**
 * Creates a public decrypt client for a cleartext host: certificates are signed by the cleartext
 * KMS key over the plaintexts the host recorded. Same parameters and actions as
 * `createFhevmPublicDecryptClient`.
 */
export function createFhevmCleartextPublicDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C> & CleartextLeafRecordParameters,
): FhevmSolanaPublicDecryptClient<C> {
  return publicDecryptClient(
    parameters,
    createSolanaHostKmsReads(parameters, getCleartextSolanaRuntime()),
    leafRecordOf(parameters),
  );
}

/**
 * Creates the private and public decrypt client for a cleartext host: permits and requests are
 * built and admitted as in production, then answered with the plaintexts the host recorded. Same
 * actions as `createFhevmDecryptClient`; there is no KMS response, so no FHE parameter.
 */
export function createFhevmCleartextDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C> & CleartextLeafRecordParameters,
): FhevmSolanaDecryptClient<C> {
  const readLeaves = leafRecordOf(parameters);
  const host = createSolanaHostKmsReads(parameters, getCleartextSolanaRuntime());
  return withPermitDecrypt(
    publicDecryptClient(parameters, host, readLeaves),
    host,
    cleartextUserDecryptExecution(parameters.rpc, parameters.chain, readLeaves),
  );
}

function leafRecordOf({
  rpc,
  chain,
  readLeaves,
}: SolanaClientParameters & CleartextLeafRecordParameters): SolanaLeafReader {
  return readLeaves ?? createSolanaLeafRecord(rpc, solanaHostProgram(chain));
}

function publicDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C>,
  host: SolanaHostKmsReads,
  readLeaves: SolanaLeafReader,
): FhevmSolanaPublicDecryptClient<C> {
  return createSolanaPublicDecryptClient(
    parameters,
    getCleartextSolanaRuntime(),
    host,
    cleartextPublicDecryptCertifier(parameters.rpc, parameters.chain, readLeaves),
  );
}
