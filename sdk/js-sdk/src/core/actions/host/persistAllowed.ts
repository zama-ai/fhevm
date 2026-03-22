import { persistAllowedAbi } from "../../host-contracts/abi/fragments.js";
import { getTrustedClient } from "../../runtime/CoreFhevm-p.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type { Bytes32Hex, ChecksummedAddress } from "../../types/primitives.js";

export type PersistAllowedParameters = {
  readonly address: ChecksummedAddress;
  readonly args: {
    readonly handle: Bytes32Hex;
    readonly account: ChecksummedAddress;
  };
};

export type PersistAllowedReturnType = boolean;

export async function persistAllowed(
  fhevm: Fhevm,
  parameters: PersistAllowedParameters,
): Promise<PersistAllowedReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: persistAllowedAbi,
    args: [parameters.args.handle, parameters.args.account],
    functionName: persistAllowedAbi[0].name,
  });

  if (typeof res !== "boolean") {
    throw new Error(`Invalid persistAllowed result.`);
  }

  return res;
}
