import type { TypedValue } from '../../core/types/primitives.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaClientParameters } from './createFhevmBaseClient.js';
import type {
  SolanaDecryptTrust,
  SolanaPermitDecryptActions,
  SolanaUserDecryptParameters,
  SolanaUserDecryptEntry,
} from './decorators/permitDecrypt.js';
import type { FhevmSolanaPublicDecryptClient } from './createFhevmPublicDecryptClient.js';
import { createFhevmPublicDecryptClient } from './createFhevmPublicDecryptClient.js';
import { getSolanaRuntime } from '../internal/runtime.js';
import { solanaPermitDecryptActions } from './decorators/permitDecrypt.js';

export type FhevmSolanaDecryptClient<C extends FhevmSolanaChain = FhevmSolanaChain> =
  FhevmSolanaPublicDecryptClient<C> &
    SolanaPermitDecryptActions & {
      readonly decryptValue: (parameters: SolanaDecryptValueParameters) => Promise<TypedValue>;
    };
export type SolanaDecryptValueParameters = Omit<SolanaUserDecryptParameters, 'entries'> & {
  readonly entry: SolanaUserDecryptEntry;
};

/** Creates the private and public decrypt client. Public-only callers use the public factory. */
export function createFhevmDecryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C> & { readonly trust: SolanaDecryptTrust },
): FhevmSolanaDecryptClient<C> {
  const base = createFhevmPublicDecryptClient(parameters);
  const actions = solanaPermitDecryptActions(
    parameters.chain,
    parameters.trust,
    getSolanaRuntime(),
    base.fetchPermitInvalidation,
  );
  return Object.assign(base, actions, {
    decryptValue: async ({ entry, ...request }: SolanaDecryptValueParameters) => {
      const values = await actions.decryptValues({ ...request, entries: [entry] });
      const value = values[0];
      if (value === undefined) throw new Error('Decryption returned no value');
      return value;
    },
  });
}
