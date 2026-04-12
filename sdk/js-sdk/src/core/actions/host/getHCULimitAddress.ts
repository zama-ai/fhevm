import { assertIsChecksummedAddress } from '../../base/address.js';
import { getHCULimitAddressAbi } from '../../host-contracts/abi-fragments/fragments.js';
import { getTrustedClient } from '../../runtime/CoreFhevm-p.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { ChecksummedAddress } from '../../types/primitives.js';

export type GetHCULimitAddressParameters = {
  readonly address: ChecksummedAddress;
};

export type GetHCULimitAddressReturnType = ChecksummedAddress;

export async function getHCULimitAddress(
  fhevm: Fhevm,
  parameters: GetHCULimitAddressParameters,
): Promise<GetHCULimitAddressReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getHCULimitAddressAbi,
    args: [],
    functionName: getHCULimitAddressAbi[0].name,
  });

  try {
    assertIsChecksummedAddress(res, {});
  } catch (e) {
    throw new Error(`Invalid HCULimit address.`, {
      cause: e,
    });
  }

  return res;
}
