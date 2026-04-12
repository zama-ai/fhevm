import { assertIsBytes32Hex } from '../../base/bytes.js';
import { eip712InputVerificationTypehashAbi } from '../../host-contracts/abi-fragments/fragments.js';
import { getTrustedClient } from '../../runtime/CoreFhevm-p.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { Bytes32Hex, ChecksummedAddress } from '../../types/primitives.js';

export type Eip712InputVerificationTypehashParameters = {
  readonly address: ChecksummedAddress;
};

export type Eip712InputVerificationTypehashReturnType = Bytes32Hex;

export async function eip712InputVerificationTypehash(
  fhevm: Fhevm,
  parameters: Eip712InputVerificationTypehashParameters,
): Promise<Eip712InputVerificationTypehashReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
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
