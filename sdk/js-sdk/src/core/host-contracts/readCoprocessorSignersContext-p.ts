import type { ChecksummedAddress } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { createCoprocessorSignersContext } from './CoprocessorSignersContext-p.js';
import type { CoprocessorSignersContext } from '../types/coprocessorSignersContext.js';
import { getCoprocessorContextSignersAndThreshold } from './getCoprocessorContextSignersAndThreshold-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly address: ChecksummedAddress;
};

type ReturnType = CoprocessorSignersContext;

////////////////////////////////////////////////////////////////////////////////

export async function readCoprocessorSignersContext(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  // TTL-Cached
  const c = await getCoprocessorContextSignersAndThreshold(context, parameters);

  const data = createCoprocessorSignersContext(new WeakRef(context.runtime), {
    ...parameters,
    coprocessorSigners: c.signers,
    coprocessorSignerThreshold: c.threshold,
  });

  return data;
}
