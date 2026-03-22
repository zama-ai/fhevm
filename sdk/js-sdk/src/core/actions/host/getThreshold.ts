import { isUint8 } from "../../base/uint.js";
import { getThresholdAbi } from "../../host-contracts/abi/fragments.js";
import { getTrustedClient } from "../../runtime/CoreFhevm-p.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type {
  ChecksummedAddress,
  Uint8Number,
} from "../../types/primitives.js";

export type GetThresholdParameters = {
  readonly address: ChecksummedAddress;
};

export type GetThresholdReturnType = Uint8Number;

export async function getThreshold(
  fhevm: Fhevm,
  parameters: GetThresholdParameters,
): Promise<GetThresholdReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getThresholdAbi,
    args: [],
    functionName: getThresholdAbi[0].name,
  });

  if (!isUint8(res)) {
    throw new Error(`Invalid threshold.`);
  }

  return Number(res) as Uint8Number;
}
