import { isUint8 } from "../../base/uint.js";
import { getHandleVersionAbi } from "../../host-contracts/abi/fragments.js";
import { getTrustedClient } from "../../runtime/CoreFhevm-p.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type {
  ChecksummedAddress,
  Uint8Number,
} from "../../types/primitives.js";

export type GetHandleVersionParameters = {
  readonly address: ChecksummedAddress;
};

export type GetHandleVersionReturnType = Uint8Number;

export async function getHandleVersion(
  fhevm: Fhevm,
  parameters: GetHandleVersionParameters,
): Promise<GetHandleVersionReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getHandleVersionAbi,
    args: [],
    functionName: getHandleVersionAbi[0].name,
  });

  if (!isUint8(res)) {
    throw new Error(`Invalid handle version.`);
  }

  return Number(res) as Uint8Number;
}
