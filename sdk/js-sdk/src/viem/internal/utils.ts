import type { PublicClient, Chain, Transport } from "viem";
import { getContract } from "viem";
import type { ChecksummedAddress } from "../../core/types/primitives.js";
import { trustedClientToViemPublicClient } from "./viem-p.js";
import type { TrustedClient } from "../../core/modules/ethereum/types.js";

/**
 * Get chain ID from a viem PublicClient.
 */
export async function getChainId(
  hostPublicClient: TrustedClient<PublicClient<Transport, Chain>>,
): Promise<bigint> {
  const client = trustedClientToViemPublicClient(hostPublicClient);
  return BigInt(await client.getChainId());
}

export function getViemContract(
  hostPublicClient: TrustedClient<PublicClient<Transport, Chain>>,
  contractAddress: ChecksummedAddress,
  abi: ReadonlyArray<Record<string, unknown>>,
): ReturnType<typeof getContract> {
  const client = trustedClientToViemPublicClient(hostPublicClient);
  return getContract({
    address: contractAddress,
    abi: abi as readonly unknown[],
    client,
  });
}
