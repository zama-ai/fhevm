import type { RelayerPublicDecryptOptions } from '../types/relayer.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { PublicDecryptionProof } from '../types/publicDecryptionProof-p.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly originToken: symbol;
  readonly handles: readonly Handle[];
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

type ReturnType = PublicDecryptionProof;

////////////////////////////////////////////////////////////////////////////////

export async function publicDecryptNoGateway(_context: Context, _parameters: Parameters): Promise<ReturnType> {
  return Promise.reject(new Error('Not yet implemented'));
}
