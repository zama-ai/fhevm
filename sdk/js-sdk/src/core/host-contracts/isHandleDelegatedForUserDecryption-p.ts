import type { ChecksummedAddress } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { HandleContractPair } from '../types/other-p.js';
import { isHandleDelegatedForUserDecryptionAbi } from './abi-fragments/fragments.js';
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
  readonly delegate: ChecksummedAddress;
  readonly delegator: ChecksummedAddress;
  readonly pairs: readonly HandleContractPair[];
};

type ReturnType = boolean[];

////////////////////////////////////////////////////////////////////////////////

export async function isHandleDelegatedForUserDecryption(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  const { address, delegate, delegator, pairs } = parameters;

  const rpcCalls = pairs.map(
    (pair) => () =>
      _isHandleDelegatedForUserDecryption(context, {
        address,
        delegate,
        delegator,
        handle: pair.handle,
        contractAddress: pair.contractAddress,
      }),
  );

  const results = await executeWithBatching(rpcCalls, context.options.batchRpcCalls);

  return results;
}

////////////////////////////////////////////////////////////////////////////////

async function _isHandleDelegatedForUserDecryption(
  context: Context,
  parameters: {
    readonly address: ChecksummedAddress;
    readonly delegator: ChecksummedAddress;
    readonly delegate: ChecksummedAddress;
    readonly contractAddress: ChecksummedAddress;
    readonly handle: Handle;
  },
): Promise<boolean> {
  const { address, delegator, delegate, contractAddress, handle } = parameters;
  const trustedClient = getTrustedClient(context);

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: isHandleDelegatedForUserDecryptionAbi,
    args: [delegator, delegate, contractAddress, handle.bytes32Hex],
    functionName: isHandleDelegatedForUserDecryptionAbi[0].name,
  });

  if (typeof res !== 'boolean') {
    throw new Error(`Invalid isHandleDelegatedForUserDecryptionAbi result.`);
  }

  return res;
}
