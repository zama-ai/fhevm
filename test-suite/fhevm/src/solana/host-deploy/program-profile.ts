import type { Address } from "@solana/kit";

import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from "../internal/generated/confidentialToken/programAddress.js";
import { ZAMA_HOST_PROGRAM_ADDRESS } from "../internal/generated/zamaHost/programAddress.js";

export const SOLANA_PROGRAM_PROFILES = ["localnet", "preview-env"] as const;
export type SolanaProgramProfile = (typeof SOLANA_PROGRAM_PROFILES)[number];

/**
 * Stable program ids for the shared preview-env host on public Solana **devnet**.
 * Local e2e keeps the generated/localnet ids. Official Zama protocol 0.15 gets a third set later.
 * Keep in sync with solana/deploy/profiles/preview-env/program-ids.json and `declare_id!`
 * behind `--features preview-env`.
 */
export const PREVIEW_ENV_ZAMA_HOST_PROGRAM_ADDRESS =
  "DPq5y89RDZPq9NcMh9X1NgjBWgYmSXg3QoipSBV3ZMzQ" as Address;
export const PREVIEW_ENV_CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS =
  "FAWs7E52LZmXR5YzFy4aXanfBjNtXV2qooQVtkmBa3cL" as Address;

export type SolanaProgramIds = {
  readonly zamaHost: Address;
  readonly confidentialToken: Address;
};

export const programIdsFor = (profile: SolanaProgramProfile): SolanaProgramIds => {
  if (profile === "preview-env") {
    return {
      zamaHost: PREVIEW_ENV_ZAMA_HOST_PROGRAM_ADDRESS,
      confidentialToken: PREVIEW_ENV_CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    };
  }
  return { zamaHost: ZAMA_HOST_PROGRAM_ADDRESS, confidentialToken: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS };
};

export const readSolanaProgramProfile = (): SolanaProgramProfile => {
  const raw = process.env.SOLANA_PROGRAM_PROFILE ?? "localnet";
  if (raw !== "localnet" && raw !== "preview-env") {
    throw new Error(`SOLANA_PROGRAM_PROFILE must be localnet or preview-env, got "${raw}"`);
  }
  return raw;
};
