import { describe, expect, test } from "bun:test";
import { mkdtemp, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";

import {
  SQUADS_PROGRAM_CONFIG_ADDRESS,
  SQUADS_PROGRAM_ID,
  SQUADS_TREASURY_ADDRESS,
  squadsGenesisExtras,
} from "./squads";

const FIXTURES = ["squads_multisig_program.so", "program_config.json", "treasury.json"] as const;

describe("squadsGenesisExtras", () => {
  test("returns the genesis flags when all three fixtures are present", async () => {
    const dir = await mkdtemp(path.join(tmpdir(), "squads-fixtures-"));
    for (const file of FIXTURES) {
      await writeFile(path.join(dir, file), "fixture");
    }

    const extras = await squadsGenesisExtras(dir);

    expect(extras?.genesisPrograms).toEqual([
      { address: SQUADS_PROGRAM_ID, soPath: path.join(dir, "squads_multisig_program.so") },
    ]);
    expect(extras?.genesisAccounts).toEqual([
      { address: SQUADS_PROGRAM_CONFIG_ADDRESS, jsonPath: path.join(dir, "program_config.json") },
      { address: SQUADS_TREASURY_ADDRESS, jsonPath: path.join(dir, "treasury.json") },
    ]);
  });

  test("is all-or-nothing: any missing fixture disables the whole group", async () => {
    const dir = await mkdtemp(path.join(tmpdir(), "squads-fixtures-"));
    await writeFile(path.join(dir, "squads_multisig_program.so"), "fixture");
    await writeFile(path.join(dir, "program_config.json"), "fixture");

    expect(await squadsGenesisExtras(dir)).toBeUndefined();
  });
});
