// sdkEncrypt — the scenarios' shared seam to the public `@fhevm/sdk/solana` encrypt client.
//
// Every input-proof phase does the same dance: dynamically import the SDK (kept out of the static
// module graph so `bun test src` stays runnable before the SDK workspace is materialized),
// configure the relayer auth, define the chain, and submit one uint64 input proof — with the
// relayer's docker-internal MinIO URLs rewritten to the host-published endpoint while the prover
// fetches key material.

import { hostReachableMaterialUrl } from "../../../src/utils/fs";

/** The SDK encrypt surface the scenarios drive (untyped: runtime dynamic-import seam). */
export type SolanaSdkEncryptSurface = {
  setFhevmRuntimeConfig(config: { auth: { type: "ApiKeyHeader"; value: string } }): void;
  defineFhevmSolanaChain(definition: { id: bigint; fhevm: { relayerUrl: string } }): unknown;
  createFhevmEncryptClient(parameters: { chain: unknown; aclProgramAddress: `0x${string}` }): {
    buildInputProof(parameters: {
      contractAddress: `0x${string}`;
      userAddress: `0x${string}`;
      values: readonly { type: "uint64"; value: bigint }[];
    }): Promise<unknown>;
    submitInputProof(parameters: { inputProof: unknown }): Promise<SolanaInputProofSubmission>;
  };
};

export type SolanaInputProofSubmission = {
  handles: readonly { bytes32Hex: `0x${string}` }[];
  signatures: readonly `0x${string}`[];
  extraData: `0x${string}`;
};

const loadSolanaSdkEncrypt = async (): Promise<SolanaSdkEncryptSurface> => {
  const solanaModule = "@fhevm/sdk/solana";
  return (await import(solanaModule)) as unknown as SolanaSdkEncryptSurface;
};

/**
 * Runs `body` with `globalThis.fetch` rewriting docker-internal MinIO URLs to the host-published
 * endpoint. The relayer hands out key-material URLs naming the compose-internal `minio:9000`;
 * a host-side prover has to fetch them through the published port instead.
 */
export const withHostReachableFetch = async <T>(body: () => Promise<T>): Promise<T> => {
  const originalFetch = globalThis.fetch;
  globalThis.fetch = ((url: string | URL | Request, options?: RequestInit) =>
    originalFetch(typeof url === "string" ? hostReachableMaterialUrl(url) : url, options)) as typeof fetch;
  try {
    return await body();
  } finally {
    globalThis.fetch = originalFetch;
  }
};

/**
 * Builds and submits one uint64 input proof through the public SDK encrypt client: a REAL ZK
 * proof to the relayer's /v2/input-proof, returning the attested handles + coprocessor
 * signatures the on-chain VerifiedInput consumption re-verifies.
 */
export const submitUint64InputProof = async (parameters: {
  readonly chainId: bigint;
  readonly relayerUrl: string;
  readonly aclProgramAddress: `0x${string}`;
  readonly contractAddress: `0x${string}`;
  readonly userAddress: `0x${string}`;
  readonly value: bigint;
}): Promise<SolanaInputProofSubmission> => {
  const solanaSdk = await loadSolanaSdkEncrypt();
  solanaSdk.setFhevmRuntimeConfig({
    auth: { type: "ApiKeyHeader", value: process.env.ZAMA_FHEVM_API_KEY ?? "local" },
  });
  const chain = solanaSdk.defineFhevmSolanaChain({ id: parameters.chainId, fhevm: { relayerUrl: parameters.relayerUrl } });
  const encryptClient = solanaSdk.createFhevmEncryptClient({
    chain,
    aclProgramAddress: parameters.aclProgramAddress,
  });
  return withHostReachableFetch(async () => {
    const inputProof = await encryptClient.buildInputProof({
      contractAddress: parameters.contractAddress,
      userAddress: parameters.userAddress,
      values: [{ type: "uint64", value: parameters.value }],
    });
    return encryptClient.submitInputProof({ inputProof });
  });
};
