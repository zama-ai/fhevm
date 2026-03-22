import { assertIsChecksummedAddress } from "../../base/address.js";
import { getInputVerifierAddressAbi } from "../../host-contracts/abi/fragments.js";
import { getTrustedClient } from "../../runtime/CoreFhevm-p.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type { ChecksummedAddress } from "../../types/primitives.js";

export type GetInputVerifierAddressParameters = {
  readonly address: ChecksummedAddress;
};

export type GetInputVerifierAddressReturnType = ChecksummedAddress;

export async function getInputVerifierAddress(
  fhevm: Fhevm,
  parameters: GetInputVerifierAddressParameters,
): Promise<GetInputVerifierAddressReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
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
