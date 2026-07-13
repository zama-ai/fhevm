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
});
