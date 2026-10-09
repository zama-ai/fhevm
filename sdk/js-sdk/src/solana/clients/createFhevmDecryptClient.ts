import type { TypedValue } from '../../core/types/primitives.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaClientParameters } from './createFhevmBaseClient.js';
import type {
  SolanaPermitDecryptActions,
  SolanaUserDecryptParameters,
  SolanaUserDecryptEntry,
} from './decorators/permitDecrypt.js';
import type { SolanaUserDecryptExecution } from './decorators/permitDecrypt.js';
import type { FhevmSolanaPublicDecryptClient } from './createFhevmPublicDecryptClient.js';
import { createSolanaPublicDecryptClient } from './createFhevmPublicDecryptClient.js';
import { getSolanaRuntime } from '../internal/runtime.js';
import { relayerUserDecryptExecution, solanaPermitDecryptActions } from './decorators/permitDecrypt.js';
import { createSolanaHostKmsReads, type SolanaHostKmsReads } from '../actions/hostKms.js';
import { publicDecryptCertificate } from '../actions/publicDecryptCertificate.js';

export type FhevmSolanaDecryptClient<C extends FhevmSolanaChain = FhevmSolanaChain> =
  FhevmSolanaPublicDecryptClient<C> &
    SolanaPermitDecryptActions & {
      readonly decryptValue: (parameters: SolanaDecryptValueParameters) => Promise<TypedValue>;
    };
export type SolanaDecryptValueParameters = Omit<SolanaUserDecryptParameters, 'entries'> & {
  readonly entry: SolanaUserDecryptEntry;
};

/**
 * Creates the private and public decrypt client. Public-only callers use the public factory.
 *
 * The KMS trust is read from the host program; the caller supplies only the FHE parameter its
 * deployment runs, e.g. `default` or `test`.
 */
export function createFhevmDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C> & { readonly fheParameter: string },
): FhevmSolanaDecryptClient<C> {
  const runtime = getSolanaRuntime();
  const host = createSolanaHostKmsReads(parameters);
  const base = createSolanaPublicDecryptClient(parameters, runtime, host, (batch) =>
    publicDecryptCertificate({ chain: parameters.chain, runtime }, batch),
  );
  return withPermitDecrypt(
    base,
    host,
    relayerUserDecryptExecution(parameters.chain, host, runtime, parameters.fheParameter),
  );
}

/** Adds the permit-path actions to a public decrypt client sharing its host reads. */
export function withPermitDecrypt<C extends FhevmSolanaChain>(
  base: FhevmSolanaPublicDecryptClient<C>,
  host: SolanaHostKmsReads,
  execution: SolanaUserDecryptExecution,
): FhevmSolanaDecryptClient<C> {
  const actions = solanaPermitDecryptActions(base.chain, host, base.fetchPermitInvalidation, execution);
  return Object.assign(base, actions, {
    decryptValue: async ({ entry, ...request }: SolanaDecryptValueParameters) => {
      const values = await actions.decryptValues({ ...request, entries: [entry] });
      const value = values[0];
      if (value === undefined) throw new Error('Decryption returned no value');
      return value;
    },
  });
}
