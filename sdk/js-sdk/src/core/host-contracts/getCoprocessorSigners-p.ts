import type { ChecksummedAddress } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { assertIsChecksummedAddressArray } from '../base/address.js';
import { getCoprocessorSignersAbi } from './abi-fragments/fragments.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
};

type Parameters = {
  readonly address: ChecksummedAddress;
};

type ReturnType = ChecksummedAddress[];

////////////////////////////////////////////////////////////////////////////////

/**
 * @internal
 * Reads the coprocessor signers addresses from the given contract.
 * The result is not cached; each call performs a fresh on-chain read.
 */
export async function getCoprocessorSigners(context: Context, parameters: Parameters): Promise<ReturnType> {
  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getCoprocessorSignersAbi,
    args: [],
    functionName: getCoprocessorSignersAbi[0].name,
  });

  try {
    assertIsChecksummedAddressArray(res, {});
  } catch (e) {
    throw new Error(`Invalid coprocessor signers addresses.`, {
      cause: e,
    });
  }

  return res;
}
