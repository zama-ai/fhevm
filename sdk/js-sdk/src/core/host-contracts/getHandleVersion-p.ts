import type { ChecksummedAddress, Uint8Number } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { isUint8 } from '../base/uint.js';
import { getHandleVersionAbi } from './abi-fragments/fragments.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
};

type Parameters = {
  readonly address: ChecksummedAddress;
};

type ReturnType = Uint8Number;

////////////////////////////////////////////////////////////////////////////////

/**
 * @internal
 * Reads the handle version from the given contract.
 * The result is not cached; each call performs a fresh on-chain read.
 */
export async function getHandleVersion(context: Context, parameters: Parameters): Promise<ReturnType> {
  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getHandleVersionAbi,
    args: [],
    functionName: getHandleVersionAbi[0].name,
  });

  if (!isUint8(res)) {
    throw new Error(`Invalid handle version.`);
  }

  return Number(res) as Uint8Number;
}
