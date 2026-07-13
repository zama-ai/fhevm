import { readFile } from "node:fs/promises";
import { describe, expect, it } from "vitest";

describe("README capability contract", () => {
  it("documents redaction and does not advertise a PostgreSQL collector", async () => {
    const readme = await readFile(new URL("../README.md", import.meta.url), "utf8");
    expect(readme).toContain("recursively redacting secret-bearing fields");
    expect(readme).toContain("intentionally does not connect to PostgreSQL");
    expect(readme).toContain("future adapter boundaries");
    expect(readme).toContain("scenarios/my-scenario.json");
    expect(readme).toContain("not relative to a referring suite file");
    expect(readme).toContain("`SEPOLIA_RPC_URL` for `testnet` and `devnet`");
    expect(readme).toContain("`POLYGON_AMOY_RPC_URL` for `devnet-amoy`");
    expect(readme).toContain("`MAINNET_RPC_URL` for");
    expect(readme).not.toContain("when the DB collector is on");
  });

  it("documents the explicit planning authority and evidence contract", async () => {
    const [readme, operations] = await Promise.all([
      readFile(new URL("../README.md", import.meta.url), "utf8"),
      readFile(new URL("../docs/OPERATIONS.md", import.meta.url), "utf8"),
    ]);
    for (const document of [readme, operations]) {
      expect(document).toContain("suite prepare");
      expect(document).toContain("--prepare");
      expect(document).toContain("pool-plan.json");
      expect(document).toContain("preparation.json");
      expect(document).toContain("JSON");
      expect(document).toContain("authoritative");
      expect(document).toMatch(/scenario\s+digest/);
      expect(document).toMatch(/environment\s+identity/);
      expect(document).toContain("inspect artifacts before sharing");
      expect(document).toContain("duration-bound");
      expect(document).toContain("[LOCAL CPU]");
      expect(document).toContain("[ON-CHAIN]");
      expect(document).toContain("`130`");
      expect(document).not.toContain("--prepare-only");
      expect(document).not.toContain("--skip-prepare");
    }
    expect(readme).toContain("`scenario run` is the canonical entry point");
    expect(readme).toContain("**Executive Summary**");
    expect(operations).toContain("implementation-agnostic");
  });
});
