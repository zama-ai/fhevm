import type { ChecksummedAddress } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { CoprocessorSignersContext } from '../types/coprocessorSignersContext.js';
import { createCoprocessorSignersContext } from './CoprocessorSignersContext-p.js';
import { getCoprocessorContextSignersAndThreshold } from './getCoprocessorContextSignersAndThreshold-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly address: ChecksummedAddress;
  /**
   * If `true`, bypasses the TTL cache and forces a fresh on-chain read of the
   * coprocessor signers and threshold. The fresh result is stored back in the
   * cache. Used to recover from a stale signer set after a verification failure.
   */
  readonly forceRefresh?: boolean | undefined;
};

type ReturnType = CoprocessorSignersContext;

////////////////////////////////////////////////////////////////////////////////

export async function readCoprocessorSignersContext(context: Context, parameters: Parameters): Promise<ReturnType> {
  // TTL-Cached
  const c = await getCoprocessorContextSignersAndThreshold(context, parameters);

  const data = createCoprocessorSignersContext(new WeakRef(context.runtime), {
    ...parameters,
    coprocessorSigners: c.signers,
    coprocessorSignerThreshold: c.threshold,
  });

  return data;
}
