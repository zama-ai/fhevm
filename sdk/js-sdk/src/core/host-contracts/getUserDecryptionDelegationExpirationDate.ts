import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import { isUint64 } from '../base/uint.js';
import { getUserDecryptionDelegationExpirationDateAbi } from '../host-contracts/abi-fragments/fragments.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly address: ChecksummedAddress;
  readonly delegator: ChecksummedAddress;
  readonly delegate: ChecksummedAddress;
  readonly contractAddress: ChecksummedAddress;
};

type ReturnType = Uint64BigInt;

////////////////////////////////////////////////////////////////////////////////

export async function getUserDecryptionDelegationExpirationDate(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getUserDecryptionDelegationExpirationDateAbi,
    args: [parameters.delegator, parameters.delegate, parameters.contractAddress],
    functionName: getUserDecryptionDelegationExpirationDateAbi[0].name,
  });

  if (!isUint64(res)) {
    throw new Error(`Invalid expiration date.`);
  }

  return BigInt(res) as Uint64BigInt;
}
