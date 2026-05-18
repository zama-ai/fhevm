import type { ChecksummedAddress } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { HandleAccountPair } from '../types/other-p.js';
import { persistAllowedAbi } from './abi-fragments/fragments.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';
import { executeWithBatching } from '../base/promise.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly address: ChecksummedAddress;
  readonly pairs: readonly HandleAccountPair[];
};

type ReturnType = boolean[];

////////////////////////////////////////////////////////////////////////////////

export async function persistAllowed(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { address, pairs } = parameters;

  const rpcCalls = pairs.map(
    (pair) => () =>
      _persistAllowed(context, {
        address,
        handle: pair.handle,
        account: pair.account,
      }),
  );

  const results = await executeWithBatching(rpcCalls, context.options.batchRpcCalls);

  return results;
}

////////////////////////////////////////////////////////////////////////////////

async function _persistAllowed(
  context: Context,
  parameters: {
    readonly address: ChecksummedAddress;
    readonly handle: Handle;
    readonly account: ChecksummedAddress;
  },
): Promise<boolean> {
  const { address, handle, account } = parameters;
  const trustedClient = getTrustedClient(context);

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: persistAllowedAbi,
    args: [handle.bytes32Hex, account],
    functionName: persistAllowedAbi[0].name,
  });

  if (typeof res !== 'boolean') {
    throw new Error(`Invalid persistAllowed result.`);
  }

  return res;
}
