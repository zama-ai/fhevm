import { describe, expect, test } from "bun:test";
import { upgradeMigrationHostContracts } from "../rollouts/v0.14-to-v0.15-gpu-key-migration/host-contract-upgrades";

describe("migration host contracts", () => {
  test("upgrades each host chain in dependency order before returning", async () => {
    const calls: string[] = [];
    await upgradeMigrationHostContracts({
      async runHostContractTaskOnChain(chain, command) {
        const contract = command.match(/npx hardhat task:upgrade(\w+)/)?.[1];
        expect(command).toContain("REINITIALIZER_VERSION");
        expect(command).toContain(`previous-contracts/${contract}.sol:${contract}`);
        calls.push(`${chain}:${contract}`);
      },
    }, [{ key: "host" }, { key: "chain-b" }]);
    expect(calls).toEqual([
      "host:FHEVMExecutor", "chain-b:FHEVMExecutor",
      "host:ACL", "chain-b:ACL", "host:HCULimit", "chain-b:HCULimit",
      "host:InputVerifier", "chain-b:InputVerifier",
      "host:KMSVerifier", "chain-b:KMSVerifier",
      "host:ProtocolConfig", "chain-b:ProtocolConfig",
    ]);
  });

  test("stops before advancing to another contract when a chain upgrade fails", async () => {
    const calls: string[] = [];
    await expect(upgradeMigrationHostContracts({
      async runHostContractTaskOnChain(chain) {
        calls.push(chain);
        if (chain === "chain-b") throw new Error("upgrade rejected");
      },
    }, [{ key: "host" }, { key: "chain-b" }])).rejects.toThrow("upgrade rejected");
    expect(calls).toEqual(["host", "chain-b"]);
  });

  test("rejects an empty host topology", async () => {
    await expect(upgradeMigrationHostContracts({
      async runHostContractTaskOnChain() { throw new Error("must not execute"); },
    }, [])).rejects.toThrow("at least one host chain");
  });
});
