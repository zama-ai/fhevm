import type { ChecksummedAddress } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { assertIsChecksummedAddress } from '../base/address.js';
import { getACLAddressAbi } from './abi-fragments/fragments.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
};

type Parameters = {
  readonly address: ChecksummedAddress;
};

type ReturnType = ChecksummedAddress;

////////////////////////////////////////////////////////////////////////////////

/**
 * @internal
 * Reads the ACL contract address from the given contract.
 * The result is not cached; each call performs a fresh on-chain read.
 */
export async function getAclAddress(context: Context, parameters: Parameters): Promise<ReturnType> {
  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getACLAddressAbi,
    args: [],
    functionName: getACLAddressAbi[0].name,
  });

  try {
    assertIsChecksummedAddress(res, {});
  } catch (e) {
    throw new Error(`Invalid ACL address.`, {
      cause: e,
    });
  }

  return res;
}
