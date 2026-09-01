// kms-trust-anchor — resolves what the SDK verifies KMS user-decrypt responses against.
//
// The SDK's Solana de-signcryption is fail-closed: it releases a share only under a valid KMS node
// signature, so `defineFhevmSolanaChain` needs the registered KMS signer set, the host program id
// the link is bound to, and the EIP-712 domain the nodes sign responses under (the gateway
// `Decryption` contract's domain). This module reads all of that from the running stack — the
// signer set and gateway chain id live from the gateway RPC, the `Decryption` address from the
// fhevm-cli address artifact — with `UD_KMS_*` env overrides for a fully pinned run.

import { PreflightError } from "../errors";
import { readGatewayBootstrapInputs } from "./addresses";
import { ZAMA_HOST_PROGRAM_ADDRESS } from "./internal/generated/zamaHost/programAddress";

type Environment = Readonly<Record<string, string | undefined>>;

/** Structural twin of the SDK's `FhevmSolanaKms` (the SDK types come from generated `_types`). */
export type SolanaKmsTrustAnchor = {
  readonly signers: readonly string[];
  readonly verifyingProgramId: string;
  readonly responseDomain: {
    readonly name: string;
    readonly version: string;
    readonly chainId: number;
    readonly verifyingContract: string;
  };
  readonly fheParameter: "default" | "test";
};

/** Matches the loadEnv default; UD_GATEWAY_RPC_URL overrides it for a non-default local stack. */
const DEFAULT_GATEWAY_RPC_URL = "http://127.0.0.1:8546";

const hex0x = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString("hex")}`;

/**
 * Resolves the KMS trust anchor for Solana user-decrypt calls. With `UD_KMS_SIGNERS` set, every
 * value comes from the environment (no network); otherwise the signer set, gateway chain id and
 * `Decryption` contract are read from the live gateway at `UD_GATEWAY_RPC_URL`.
 */
export const resolveSolanaKmsTrustAnchor = async (
  environment: Environment = process.env,
): Promise<SolanaKmsTrustAnchor> => {
  const name = environment.UD_KMS_DOMAIN_NAME ?? "Decryption";
  const version = environment.UD_KMS_DOMAIN_VERSION ?? "1";
  const fheParameter = environment.UD_KMS_FHE_PARAMETER === "test" ? "test" : "default";
  const verifyingProgramId = environment.UD_KMS_VERIFYING_PROGRAM_ID ?? ZAMA_HOST_PROGRAM_ADDRESS;

  const pinnedSigners = environment.UD_KMS_SIGNERS;
  if (pinnedSigners !== undefined) {
    const signers = pinnedSigners
      .split(",")
      .map((value) => value.trim())
      .filter(Boolean);
    if (signers.length === 0) {
      throw new PreflightError("UD_KMS_SIGNERS must list at least one KMS signer address");
    }
    const chainId = environment.UD_KMS_GATEWAY_CHAIN_ID;
    const verifyingContract = environment.UD_KMS_DECRYPTION_CONTRACT;
    if (!chainId || !verifyingContract) {
      throw new PreflightError(
        "UD_KMS_GATEWAY_CHAIN_ID and UD_KMS_DECRYPTION_CONTRACT are required alongside UD_KMS_SIGNERS",
      );
    }
    return {
      signers,
      verifyingProgramId,
      responseDomain: { name, version, chainId: Number(chainId), verifyingContract },
      fheParameter,
    };
  }

  const gateway = await readGatewayBootstrapInputs({
    gatewayRpcUrl: environment.UD_GATEWAY_RPC_URL ?? DEFAULT_GATEWAY_RPC_URL,
  });
  if (gateway.kmsSigners.length === 0) {
    throw new PreflightError("the gateway reports an empty KMS signer set; nothing could verify");
  }
  return {
    signers: gateway.kmsSigners.map(hex0x),
    verifyingProgramId,
    responseDomain: {
      name,
      version,
      chainId: Number(gateway.gatewayChainId),
      verifyingContract: hex0x(gateway.decryptionContract),
    },
    fheParameter,
  };
};
