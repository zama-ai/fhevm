import path from "node:path";
import { describe, expect, test } from "bun:test";

import { contractTaskEnvArgs, previousContractsMountArgs, withPreviousContractsSnapshot } from "./flow/contracts";
import { withTempStateDir } from "./test-state";

describe("contract tasks", () => {
  test("passes explicit task env through docker compose run", () => {
    expect(contractTaskEnvArgs({ MIGRATION_CONTEXT_ID: "0x1", MIGRATION_TX_SENDERS: "0xabc,0xdef" })).toEqual([
      "--env",
      "MIGRATION_CONTEXT_ID=0x1",
      "--env",
      "MIGRATION_TX_SENDERS=0xabc,0xdef",
    ]);
  });

  test("mounts previous contract source snapshots read-only", async () => {
    await withTempStateDir(async (stateDir) => {
      expect(previousContractsMountArgs("host", true)).toEqual([
        "--volume",
        `${path.join(stateDir, "runtime", "previous-contracts", "host")}:/app/previous-contracts-snapshot:ro`,
      ]);
      expect(previousContractsMountArgs("host", false)).toEqual([]);
    });
  });

  test("copies previous contract snapshots to the Hardhat task path", () => {
    expect(withPreviousContractsSnapshot("npx hardhat compile")).toContain(
      "cp -R /app/previous-contracts-snapshot/. /app/previous-contracts",
    );
  });
});
