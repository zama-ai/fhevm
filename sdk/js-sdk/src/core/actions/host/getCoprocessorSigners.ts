import { assertIsChecksummedAddressArray } from '../../base/address.js';
import { getCoprocessorSignersAbi } from '../../host-contracts/abi-fragments/fragments.js';
import { getTrustedClient } from '../../runtime/CoreFhevm-p.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { ChecksummedAddress } from '../../types/primitives.js';

export type GetCoprocessorSignersParameters = {
  readonly address: ChecksummedAddress;
};

export type GetCoprocessorSignersReturnType = ChecksummedAddress[];

export async function getCoprocessorSigners(
  fhevm: Fhevm,
  parameters: GetCoprocessorSignersParameters,
): Promise<GetCoprocessorSignersReturnType> {
  const trustedClient = getTrustedClient(fhevm);

  const address = parameters.address;

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getCoprocessorSignersAbi,
    args: [],
    functionName: getCoprocessorSignersAbi[0].name,
  });

  try {
    assertIsChecksummedAddressArray(res, {});
  } catch (e) {
    throw new Error(`Invalid coprocessor signers addresses.`, {
      cause: e,
    });
  }

  return res;
}
