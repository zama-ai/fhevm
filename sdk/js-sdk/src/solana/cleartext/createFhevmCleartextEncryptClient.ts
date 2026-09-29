import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaClientParameters, SolanaEncryptOptions } from '../clients/createFhevmBaseClient.js';
import type { FhevmSolanaEncryptClient } from '../clients/createFhevmEncryptClient.js';
import { encryptModule } from '../../core/modules/encrypt/mock.js';
import { createSolanaEncryptClient } from '../clients/createFhevmEncryptClient.js';
import { cleartextSubmitInputProof } from './inputAttestation.js';
import { getCleartextSolanaRuntime } from './runtime.js';

/**
 * Creates an encrypt client for a cleartext host: inputs carry their plaintexts, attested with the
 * cleartext coprocessor key. Same parameters and actions as `createFhevmEncryptClient`.
 */
export function createFhevmCleartextEncryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C> & { readonly options?: SolanaEncryptOptions | undefined },
): FhevmSolanaEncryptClient<C> {
  return createSolanaEncryptClient(parameters, getCleartextSolanaRuntime(), {
    encryptModule,
    submitInputProof: cleartextSubmitInputProof(parameters.rpc),
  });
}
