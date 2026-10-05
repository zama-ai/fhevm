import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaClientParameters } from '../clients/createFhevmBaseClient.js';
import type { FhevmSolanaDecryptClient } from '../clients/createFhevmDecryptClient.js';
import type { FhevmSolanaPublicDecryptClient } from '../clients/createFhevmPublicDecryptClient.js';
import type { SolanaDecryptTrust } from '../clients/decorators/permitDecrypt.js';
import type { SolanaMerkleProofReader } from './merkleProofs.js';
import { withPermitDecrypt } from '../clients/createFhevmDecryptClient.js';
import { solanaHostProgram } from '../clients/createFhevmBaseClient.js';
import { createSolanaPublicDecryptClient } from '../clients/createFhevmPublicDecryptClient.js';
import { cleartextPublicDecryptCertifier, cleartextUserDecryptExecution } from './decrypt.js';
import { createSolanaLeafRecord } from './leafRecord.js';
import { getCleartextSolanaRuntime } from './runtime.js';

/**
 * The leaf record a cleartext client reads, standing in for the coprocessors' Merkle proof route
 * that the Connector reads.
 * Pass one record from `createSolanaLeafRecord` to every client of a host, so each client reads
 * only the store writes the record has not seen yet. A client without one keeps its own.
 */
type CleartextLeafRecordParameters = { readonly readMerkleProofs?: SolanaMerkleProofReader };

/**
 * Creates a public decrypt client for a cleartext host: certificates are signed by the cleartext
 * KMS key over the plaintexts the host recorded. Same parameters and actions as
 * `createFhevmPublicDecryptClient`.
 */
export function createFhevmCleartextPublicDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C> & CleartextLeafRecordParameters,
): FhevmSolanaPublicDecryptClient<C> {
  return publicDecryptClient(parameters, leafRecordOf(parameters));
}

/**
 * Creates the private and public decrypt client for a cleartext host: permits and requests are
 * built and admitted as in production, then answered with the plaintexts the host recorded. Same
 * parameters and actions as `createFhevmDecryptClient`.
 */
export function createFhevmCleartextDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C> & CleartextLeafRecordParameters & { readonly trust: SolanaDecryptTrust },
): FhevmSolanaDecryptClient<C> {
  const readMerkleProofs = leafRecordOf(parameters);
  return withPermitDecrypt(
    publicDecryptClient(parameters, readMerkleProofs),
    parameters.trust,
    getCleartextSolanaRuntime(),
    cleartextUserDecryptExecution(parameters.rpc, parameters.chain, parameters.trust, readMerkleProofs),
  );
}

function leafRecordOf({
  rpc,
  chain,
  readMerkleProofs,
}: SolanaClientParameters & CleartextLeafRecordParameters): SolanaMerkleProofReader {
  return readMerkleProofs ?? createSolanaLeafRecord(rpc, solanaHostProgram(chain));
}

function publicDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C>,
  readMerkleProofs: SolanaMerkleProofReader,
): FhevmSolanaPublicDecryptClient<C> {
  return createSolanaPublicDecryptClient(
    parameters,
    getCleartextSolanaRuntime(),
    cleartextPublicDecryptCertifier(parameters.rpc, parameters.chain, readMerkleProofs),
  );
}
