import type { ChecksummedAddress } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { CoprocessorSignersContext } from '../types/coprocessorSignersContext.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { createCoprocessorSignersContext } from './CoprocessorSignersContext-p.js';
import { coprocessorSignersCache } from './getCoprocessorContextSignersAndThreshold-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly chain?: { readonly id: number } | undefined;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly address: ChecksummedAddress;
  readonly fhevmContext: FhevmClientFrozenContext;
};

type ReturnType = CoprocessorSignersContext;

////////////////////////////////////////////////////////////////////////////////

export async function readCoprocessorSignersContext(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { value } = await readVersionedCoprocessorSignersContext(context, parameters);
  return value;
}

/**
 * {@link readCoprocessorSignersContext}, with the cache version of the signer
 * set, for a request that records it (see `retryOnStaleCache`).
 */
export async function readVersionedCoprocessorSignersContext(
  context: Context,
  parameters: Parameters,
): Promise<{ readonly value: ReturnType; readonly version: string }> {
  // TTL-Cached
  const { value: c, version } = await coprocessorSignersCache.get(context, { address: parameters.address });

  const value = createCoprocessorSignersContext(new WeakRef(context.runtime), {
    ...parameters,
    coprocessorSigners: c.signers,
    coprocessorSignerThreshold: c.threshold,
  });

  return { value, version };
}
