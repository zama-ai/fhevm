import type { Bytes32Hex, ChecksummedAddress } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { assertIsBytes32Hex } from '../base/bytes.js';
import { decryptionResultTypehashAbi } from './abi-fragments/fragments.js';
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
 * Reads the decryption result typehash from the given contract.
 * The result is not cached; each call performs a fresh on-chain read.
 */
export async function decryptionResultTypehash(context: Context, parameters: Parameters): Promise<ReturnType> {
  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: decryptionResultTypehashAbi,
    args: [],
    functionName: decryptionResultTypehashAbi[0].name,
  });

  try {
    assertIsBytes32Hex(res, {});
  } catch (e) {
    throw new Error(`Invalid decryption result typehash.`, {
      cause: e,
    });
  }

  return res;
}
