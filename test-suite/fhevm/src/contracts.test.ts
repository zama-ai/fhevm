import path from "node:path";
import { describe, expect, test } from "bun:test";

import { contractTaskEnvArgs, previousContractsMountArgs, withPreviousContractsSnapshot } from "./flow/contracts";
import { renderHostChainAddressesSolidity } from "./generate/addresses";
import { withTempStateDir } from "./test-state";

describe("host address artifacts", () => {
  const solidityFor = (host?: Record<string, string>) =>
    renderHostChainAddressesSolidity({ discovery: host ? { gateway: {}, hosts: { host } } : undefined } as never, "host");

  // v0.14 ACL.sol imports confidentialBridgeAdd unconditionally. A stack booted on pre-v0.14 host
  // contracts discovers no bridge address, so without a default an N-1 -> N rollout fails to
  // compile the v0.14 sources with `Declaration "confidentialBridgeAdd" not found`.
  test("always declares a bridge address for a discovered chain", () => {
    expect(solidityFor({ ACL_CONTRACT_ADDRESS: "0xacl" })).toContain(
      "address constant confidentialBridgeAdd = 0x0000000000000000000000000000000000000000;",
    );
  });

  test("keeps a discovered bridge address", () => {
    expect(solidityFor({ CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS: "0xbridge" })).toContain(
      "address constant confidentialBridgeAdd = 0xbridge;",
    );
  });

  test("declares nothing for an undiscovered chain", () => {
    expect(solidityFor(undefined)).not.toContain("confidentialBridgeAdd");
  });
});

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
