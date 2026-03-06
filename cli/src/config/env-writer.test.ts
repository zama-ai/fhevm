import { describe, expect, it } from "bun:test";
import { readdir, readFile, rm } from "fs/promises";
import { deriveAllKeys } from "./keys";
import { DEFAULT_MNEMONIC, createDefaultConfig } from "./model";
import { formatEnvContent, generateAllEnvFiles, writeEnvFile } from "./env-writer";

function makeTempDir(): string {
  return `.fhevm/test-env/${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

describe("env writer", () => {
  it("formats env vars deterministically and quotes values when required", () => {
    const content = formatEnvContent({
      B: "value with spaces",
      A: "simple",
      C: "needs#quote",
      D: 'quote"value',
      E: "",
    });

    expect(content).toBe(
      [
        "A=simple",
        'B="value with spaces"',
        'C="needs#quote"',
        'D="quote\\"value"',
        "E=",
        "",
      ].join("\n"),
    );
  });

  it("writes env files atomically", async () => {
    const dir = makeTempDir();
    const path = `${dir}/single.env`;
    await writeEnvFile(path, { A: "1", B: "2" });

    const content = await readFile(path, "utf8");
    expect(content).toContain("A=1");
    expect(content).toContain("B=2");

    const files = await readdir(dir);
    expect(files.some((file) => file.includes(".tmp."))).toBe(false);

    await rm(dir, { recursive: true, force: true });
  });

  it("generates all service env files", async () => {
    const dir = makeTempDir();
    const keys = deriveAllKeys(DEFAULT_MNEMONIC, 1, 1);
    const config = createDefaultConfig(keys);

    const written = await generateAllEnvFiles(config, dir);
    expect(written.size).toBe(12);

    const files = await readdir(dir);
    expect(files).toContain(".env.coprocessor.local");
    expect(files).toContain(".env.kms-connector.local");
    expect(files).toContain(".env.gateway-sc.local");
    expect(files).toContain(".env.host-sc.local");
    expect(files).toContain(".env.gateway-mocked-payment.local");

    for (const file of files) {
      const content = await readFile(`${dir}/${file}`, "utf8");
      expect(content.trim().length).toBeGreaterThan(0);
      expect(content.split("\n").every((line) => line.includes("=") || line.length === 0)).toBe(true);
    }

    await rm(dir, { recursive: true, force: true });
  });

  it("supports per-env config overrides", async () => {
    const dir = makeTempDir();
    const keys = deriveAllKeys(DEFAULT_MNEMONIC, 1, 1);
    const dockerConfig = createDefaultConfig(keys);
    const localConfig = structuredClone(dockerConfig);
    localConfig.db.host = "localhost";
    localConfig.rpc.gatewayHttp = "http://localhost:8546";

    await generateAllEnvFiles(
      {
        defaultConfig: dockerConfig,
        overrides: { coprocessor: localConfig },
      },
      dir,
    );

    const coprocessorEnv = await readFile(`${dir}/.env.coprocessor.local`, "utf8");
    const gatewayScEnv = await readFile(`${dir}/.env.gateway-sc.local`, "utf8");

    expect(coprocessorEnv).toContain("DATABASE_URL=postgresql://postgres:postgres@localhost:5432/coprocessor");
    expect(gatewayScEnv).toContain("RPC_URL=http://gateway-node:8546");

    await rm(dir, { recursive: true, force: true });
  });
});
