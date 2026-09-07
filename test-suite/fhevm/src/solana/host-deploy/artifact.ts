import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";

/** Filename the contracts chart scrapes (`addresses/.env.*`) after `deployCommands`. */
export const SOLANA_ADDRESS_ENV_FILE = ".env.solana";

export type SolanaAddressArtifact = {
  readonly zamaHostId: string;
  readonly confidentialTokenId: string;
  /** Confirmed slot after bootstrap; stored as `bootstrap_slot.address` by the chart. */
  readonly bootstrapSlot: string;
};

/**
 * Writes the address artifact `deploy-contracts.sh` patches into ConfigMap
 * `solana-program-ids`. Keys keep the `_ADDRESS` suffix so the chart's
 * `s/_ADDRESS//` + lowercase transform yields `zama_host.address`.
 */
export const writeSolanaAddressArtifact = async (
  addressesDir: string,
  ids: SolanaAddressArtifact,
): Promise<string> => {
  await mkdir(addressesDir, { recursive: true });
  const file = path.join(addressesDir, SOLANA_ADDRESS_ENV_FILE);
  const body = [
    `ZAMA_HOST_ADDRESS=${ids.zamaHostId}`,
    `CONFIDENTIAL_TOKEN_ADDRESS=${ids.confidentialTokenId}`,
    `BOOTSTRAP_SLOT_ADDRESS=${ids.bootstrapSlot}`,
    "",
  ].join("\n");
  await writeFile(file, body);
  return file;
};
