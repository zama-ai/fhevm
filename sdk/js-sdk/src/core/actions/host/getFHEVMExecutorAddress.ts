import { assertIsChecksummedAddress } from "../../base/address.js";
import { getFHEVMExecutorAddressAbi } from "../../host-contracts/abi/fragments.js";
import { getTrustedClient } from "../../runtime/CoreFhevm-p.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type { ChecksummedAddress } from "../../types/primitives.js";

export type GetFHEVMExecutorAddressParameters = {
  readonly address: ChecksummedAddress;
};

export type GetFHEVMExecutorAddressReturnType = ChecksummedAddress;

export async function getFHEVMExecutorAddress(
  fhevm: Fhevm,
  parameters: GetFHEVMExecutorAddressParameters,
): Promise<GetFHEVMExecutorAddressReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;
  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getFHEVMExecutorAddressAbi,
    args: [],
    functionName: getFHEVMExecutorAddressAbi[0].name,
  });

  try {
    assertIsChecksummedAddress(res, {});
  } catch (e) {
    throw new Error(`Invalid FHEVMExecutor address.`, {
      cause: e,
    });
  }

  return res;
}
