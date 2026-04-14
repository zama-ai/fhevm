import type { ChecksummedAddress } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { assertIsChecksummedAddress } from '../base/address.js';
import { getInputVerifierAddressAbi } from './abi-fragments/fragments.js';
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
 * Reads the InputVerifier contract address from the given contract.
 * The result is not cached; each call performs a fresh on-chain read.
 */
export async function getInputVerifierAddress(context: Context, parameters: Parameters): Promise<ReturnType> {
  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getInputVerifierAddressAbi,
    args: [],
    functionName: getInputVerifierAddressAbi[0].name,
  });

  try {
    assertIsChecksummedAddress(res, {});
  } catch (e) {
    throw new Error(`Invalid InputVerifier address.`, {
      cause: e,
    });
  }

  return res;
}
