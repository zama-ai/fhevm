import { assertIsBytes32Hex } from '../../base/bytes.js';
import { decryptionResultTypehashAbi } from '../../host-contracts/abi-fragments/fragments.js';
import { getTrustedClient } from '../../runtime/CoreFhevm-p.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { Bytes32Hex, ChecksummedAddress } from '../../types/primitives.js';

export type DecryptionResultTypehashParameters = {
  readonly address: ChecksummedAddress;
};

export type DecryptionResultTypehashReturnType = Bytes32Hex;

export async function decryptionResultTypehash(
  fhevm: Fhevm,
  parameters: DecryptionResultTypehashParameters,
): Promise<DecryptionResultTypehashReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
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
