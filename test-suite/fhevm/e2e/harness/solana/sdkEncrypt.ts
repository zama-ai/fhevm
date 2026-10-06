import { createSolanaRpc } from "@solana/kit";
// sdkEncrypt — the scenarios' shared seam to the public `@fhevm/sdk/solana` encrypt client.
//
// Every input-proof phase does the same dance: dynamically import the SDK (kept out of the static
// module graph so `bun test src` stays runnable before the SDK workspace is materialized),
// configure the relayer auth, define the chain, and submit one uint64 input proof — with the
// relayer's docker-internal object-store URLs rewritten to the host-published endpoint while the
// prover fetches key material.
import { asBytes32Hex } from "@fhevm/sdk/base";
import type { SolanaSubmitInputProofResult } from "@fhevm/sdk/solana";

import { loadSolanaSdk } from "../../../src/solana/target";
import { withHostReachableFetch } from "../../../src/utils/fs";

/**
 * Builds and submits one uint64 input proof through the public SDK encrypt client: a REAL ZK
 * proof to the relayer's /v2/input-proof, returning the attested handles + coprocessor
 * signatures the on-chain VerifiedInput consumption re-verifies.
 */
export const submitUint64InputProof = async (parameters: {
  readonly chainId: bigint;
  readonly relayerUrl: string;
  readonly rpcUrl: string;
  readonly aclProgramAddress: `0x${string}`;
  readonly contractAddress: `0x${string}`;
  readonly userAddress: `0x${string}`;
  readonly value: bigint;
}): Promise<SolanaSubmitInputProofResult> => {
  const solanaSdk = await loadSolanaSdk();
  solanaSdk.setFhevmRuntimeConfig({
    auth: { type: "ApiKeyHeader", value: process.env.ZAMA_FHEVM_API_KEY ?? "local" },
  });
  const chain = solanaSdk.defineFhevmSolanaChain({ id: parameters.chainId, fhevm: { relayerUrl: parameters.relayerUrl, programs: { host: { address: asBytes32Hex(parameters.aclProgramAddress) } } } });
  const encryptClient = solanaSdk.createFhevmEncryptClient({
    chain,
    rpc: createSolanaRpc(parameters.rpcUrl),
  });
  return withHostReachableFetch(async () => {
    const inputProof = await encryptClient.generateZkProof({
      contractAddress: asBytes32Hex(parameters.contractAddress),
      userAddress: asBytes32Hex(parameters.userAddress),
      values: [{ type: "uint64", value: parameters.value }],
    });
    return encryptClient.submitInputProof({ inputProof });
  });
};
