// squads — the real Squads v4 multisig program as a genesis fixture of the e2e validator.
//
// The repo tracks no executable binaries (the only vendored binaries are the SDK's WASM blobs),
// so the program is not committed: `solana/scripts/e2e/fetch-squads-fixtures.sh` dumps it from
// mainnet into `solana/target/squads/` and verifies it against sha256 pins. This module only
// *discovers* the fixtures — when they are present the validator loads them at genesis, when
// they are not the stack still boots and only the Squads scenario refuses to run.
//
// Genesis, not a deploy transaction, because the program must live at its canonical mainnet id
// (`SQDS4…`): `solana program deploy --program-id` demands the program's keypair, which a
// mainnet program never gives us, and the `@sqds/multisig` client hardcodes the id.

import { access } from "node:fs/promises";
import path from "node:path";

import { REPO_ROOT } from "../layout";

/** The canonical Squads v4 program id, on mainnet-beta and here. */
export const SQUADS_PROGRAM_ID = "SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf";
/** The program config PDA `multisigCreateV2` reads; cloned from mainnet by the fetch script. */
export const SQUADS_PROGRAM_CONFIG_ADDRESS = "BSTq9w3kZwNwpBXJEvTZz2G9ZTNyKBvoSeXMvwb4cNZr";
/**
 * The treasury the program config names. The mainnet creation fee is 0, so it never receives
 * anything here — the fetch script synthesizes a plain funded system account for it.
 */
export const SQUADS_TREASURY_ADDRESS = "5DH2e3cJmFpyi6mk65EGFediunm4ui6BiKNUNrhWtD1b";

/** Where the fetch script puts the fixtures. Untracked by design. */
export const SQUADS_FIXTURES_DIR = path.join(REPO_ROOT, "solana", "target", "squads");

export type SquadsGenesisExtras = {
  readonly genesisPrograms: readonly { readonly address: string; readonly soPath: string }[];
  readonly genesisAccounts: readonly { readonly address: string; readonly jsonPath: string }[];
};

const exists = async (file: string): Promise<boolean> => {
  try {
    await access(file);
    return true;
  } catch {
    return false;
  }
};

/**
 * The genesis flags for the Squads fixtures, or `undefined` when any of the three files is
 * missing — all or nothing, because a program without its config account cannot create a
 * multisig and would fail later and less legibly.
 */
export const squadsGenesisExtras = async (
  fixturesDir: string = SQUADS_FIXTURES_DIR,
): Promise<SquadsGenesisExtras | undefined> => {
  const soPath = path.join(fixturesDir, "squads_multisig_program.so");
  const programConfigPath = path.join(fixturesDir, "program_config.json");
  const treasuryPath = path.join(fixturesDir, "treasury.json");
  const present = await Promise.all([exists(soPath), exists(programConfigPath), exists(treasuryPath)]);
  if (!present.every(Boolean)) {
    return undefined;
  }
  return {
    genesisPrograms: [{ address: SQUADS_PROGRAM_ID, soPath }],
    genesisAccounts: [
      { address: SQUADS_PROGRAM_CONFIG_ADDRESS, jsonPath: programConfigPath },
      { address: SQUADS_TREASURY_ADDRESS, jsonPath: treasuryPath },
    ],
  };
};
