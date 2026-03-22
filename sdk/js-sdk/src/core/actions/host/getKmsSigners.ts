import { assertIsChecksummedAddressArray } from "../../base/address.js";
import { getKmsSignersAbi } from "../../host-contracts/abi/fragments.js";
import { getTrustedClient } from "../../runtime/CoreFhevm-p.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type { ChecksummedAddress } from "../../types/primitives.js";

export type GetKmsSignersParameters = {
  readonly address: ChecksummedAddress;
};

export type GetKmsSignersReturnType = ChecksummedAddress[];

export async function getKmsSigners(
  fhevm: Fhevm,
  parameters: GetKmsSignersParameters,
): Promise<GetKmsSignersReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getKmsSignersAbi,
    args: [],
    functionName: getKmsSignersAbi[0].name,
  });

  try {
    assertIsChecksummedAddressArray(res, {});
  } catch (e) {
    throw new Error(`Invalid kms signers addresses.`, {
      cause: e,
    });
  }

  return res;
}
