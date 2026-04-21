import type { Bytes32Hex, ChecksummedAddress } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { assertIsBytes32Hex } from '../base/bytes.js';
import { eip712InputVerificationTypehashAbi } from './abi-fragments/fragments.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
};

type Parameters = {
  readonly address: ChecksummedAddress;
};

type ReturnType = Bytes32Hex;

////////////////////////////////////////////////////////////////////////////////

/**
 * @internal
 * Reads the EIP-712 input verification typehash from the given contract.
 * The result is not cached; each call performs a fresh on-chain read.
 */
export async function eip712InputVerificationTypehash(context: Context, parameters: Parameters): Promise<ReturnType> {
  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: eip712InputVerificationTypehashAbi,
    args: [],
    functionName: eip712InputVerificationTypehashAbi[0].name,
  });

  try {
    assertIsBytes32Hex(res, {});
  } catch (e) {
    throw new Error(`Invalid eip712 input verification typehash.`, {
      cause: e,
    });
  }

  return res;
}
