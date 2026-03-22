import { assertIsChecksummedAddress } from "../../base/address.js";
import { getACLAddressAbi } from "../../host-contracts/abi/fragments.js";
import { getTrustedClient } from "../../runtime/CoreFhevm-p.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type { ChecksummedAddress } from "../../types/primitives.js";

export type GetACLAddressParameters = {
  readonly address: ChecksummedAddress;
};

export type GetACLAddressReturnType = ChecksummedAddress;

export async function getACLAddress(
  fhevm: Fhevm,
  parameters: GetACLAddressParameters,
): Promise<GetACLAddressReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
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
