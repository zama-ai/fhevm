import type { ChecksummedAddress } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import { isAllowedForDecryptionAbi } from './abi-fragments/fragments.js';
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
  readonly handles: readonly Handle[];
};

type ReturnType = boolean[];

////////////////////////////////////////////////////////////////////////////////

export async function isAllowedForDecryption(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { address, handles } = parameters;

  const rpcCalls = handles.map(
    (handle) => () =>
      _isAllowedForDecryption(context, {
        address,
        handle,
      }),
  );

  const results = await executeWithBatching(rpcCalls, context.options.batchRpcCalls);

  return results;
}

////////////////////////////////////////////////////////////////////////////////

async function _isAllowedForDecryption(
  context: Context,
  parameters: {
    readonly address: ChecksummedAddress;
    readonly handle: Handle;
  },
): Promise<boolean> {
  const { address, handle } = parameters;
  const trustedClient = getTrustedClient(context);

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: isAllowedForDecryptionAbi,
    args: [handle.bytes32Hex],
    functionName: isAllowedForDecryptionAbi[0].name,
  });

  if (typeof res !== 'boolean') {
    throw new Error(`Invalid isAllowedForDecryption result.`);
  }

  return res;
}
