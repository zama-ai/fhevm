import type { SolanaInputProof } from './decorators/encrypt.js';
import type { EncryptedValue } from '../../core/types/encryptedTypes.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaEncryptActions } from './decorators/encrypt.js';
import type { FhevmSolanaBaseClient, SolanaClientParameters, SolanaEncryptOptions } from './createFhevmBaseClient.js';
import { createSolanaCore, solanaClientSurface, solanaHostProgram } from './createFhevmBaseClient.js';
import { solanaEncryptActions } from './decorators/encrypt.js';
import { getAddressEncoder } from '@solana/kit';
import { asBytes32Hex, bytesToHex } from '../../core/base/bytes.js';

export type FhevmSolanaEncryptClient<C extends FhevmSolanaChain = FhevmSolanaChain> = FhevmSolanaBaseClient<C> &
  SolanaEncryptActions & {
    readonly encryptValue: (
      parameters: SolanaEncryptValueParameters,
    ) => Promise<{ readonly encryptedValue: EncryptedValue; readonly inputProof: SolanaInputProof }>;
  };

export type SolanaEncryptValueParameters = Omit<Parameters<SolanaEncryptActions['encryptValues']>[0], 'values'> & {
  readonly value: Parameters<SolanaEncryptActions['encryptValues']>[0]['values'][number];
};

/** Creates an encrypt client using the host identity from its chain configuration. */
export function createFhevmEncryptClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C> & { readonly options?: SolanaEncryptOptions | undefined },
): FhevmSolanaEncryptClient<C> {
  const core = createSolanaCore(parameters).extend(
    solanaEncryptActions(
      asBytes32Hex(bytesToHex(new Uint8Array(getAddressEncoder().encode(solanaHostProgram(parameters.chain))))),
    ),
  );
  return Object.assign(solanaClientSurface(core, parameters), {
    generateZkProof: core.generateZkProof,
    submitInputProof: core.submitInputProof,
    encryptValues: core.encryptValues,
    encryptValue: async ({ value, ...request }: SolanaEncryptValueParameters) => {
      const result = await core.encryptValues({ ...request, values: [value] });
      const encryptedValue = result.encryptedValues[0];
      if (encryptedValue === undefined) throw new Error('Encryption returned no value');
      return { encryptedValue, inputProof: result.inputProof };
    },
  });
}
