import type { TypedValue } from '../../core/types/primitives.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaClientParameters } from './createFhevmBaseClient.js';
import type {
  SolanaDecryptTrust,
  SolanaPermitDecryptActions,
  SolanaUserDecryptParameters,
  SolanaUserDecryptEntry,
} from './decorators/permitDecrypt.js';
import type { SolanaUserDecryptExecution } from './decorators/permitDecrypt.js';
import type { FhevmSolanaPublicDecryptClient } from './createFhevmPublicDecryptClient.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
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
  return withPermitDecrypt(createFhevmPublicDecryptClient(parameters), parameters.trust, getSolanaRuntime());
}

/** Adds the permit-path actions to a public decrypt client; the cleartext client supplies `execution`. */
export function withPermitDecrypt<C extends FhevmSolanaChain>(
  base: FhevmSolanaPublicDecryptClient<C>,
  trust: SolanaDecryptTrust,
  runtime: FhevmRuntime,
  execution?: SolanaUserDecryptExecution,
): FhevmSolanaDecryptClient<C> {
  const actions = solanaPermitDecryptActions(base.chain, trust, runtime, base.fetchPermitInvalidation, execution);
  return Object.assign(base, actions, {
    decryptValue: async ({ entry, ...request }: SolanaDecryptValueParameters) => {
      const values = await actions.decryptValues({ ...request, entries: [entry] });
      const value = values[0];
      if (value === undefined) throw new Error('Decryption returned no value');
      return value;
    },
  });
}
