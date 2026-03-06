import { describe, expect, it } from "bun:test";
import { readFile, rm } from "fs/promises";
import { deriveAllKeys } from "./keys";
import { DEFAULT_MNEMONIC, createDefaultConfig, type FhevmConfig } from "./model";
import { generateRelayerConfigFile, renderRelayerConfig } from "./relayer-config";

function buildConfig(overrides: Partial<FhevmConfig> = {}): FhevmConfig {
  const keys = deriveAllKeys(DEFAULT_MNEMONIC, 1, 1);
  return createDefaultConfig(keys, {
    contracts: {
      decryption: "0xdecryption",
      inputVerification: "0xinput",
    },
    ...overrides,
  });
}

describe("relayer config", () => {
  it("renders relayer yaml from config model values", () => {
    const config = buildConfig({
      chainIds: { host: 12345, gateway: 77777 },
      rpc: {
        hostHttp: "http://host-node:8545",
        hostWs: "ws://host-node:8545",
        gatewayHttp: "http://gateway-node:9999",
        gatewayWs: "ws://gateway-node:9999",
        kmsCore: "http://kms-core:50051",
        relayerHttp: "http://fhevm-relayer:3000",
      },
      db: {
        user: "postgres",
        password: "postgres",
        host: "coprocessor-and-kms-db",
        port: 5432,
        relayerHost: "fhevm-relayer-db",
        relayerPort: 5432,
        coprocessorDb: "coprocessor",
        kmsConnectorDb: "kms-connector",
        relayerDb: "relayer_db",
      },
    });
    const template = [
      '    http_url: "http://gateway-node:8546"',
      '    read_http_url: "http://gateway-node:8546"',
      "    chain_id: 54321",
      '        url: "ws://gateway-node:8546"',
      "    private_key: 0xaaaa000000000000000000000000000000000000000000000000000000000001",
      '    decryption_address: "0xold1"',
      '    input_verification_address: "0xold2"',
      '  sql_database_url: "postgresql://postgres:postgres@relayer-db:5433/relayer_db"',
      "",
    ].join("\n");

    const rendered = renderRelayerConfig(template, config);
    expect(rendered).toContain('http_url: "http://gateway-node:9999"');
    expect(rendered).toContain('read_http_url: "http://gateway-node:9999"');
    expect(rendered).toContain("chain_id: 77777");
    expect(rendered).toContain('url: "ws://gateway-node:9999"');
    expect(rendered).toContain('decryption_address: "0xdecryption"');
    expect(rendered).toContain('input_verification_address: "0xinput"');
    expect(rendered).toContain('sql_database_url: "postgresql://postgres:postgres@fhevm-relayer-db:5432/relayer_db"');
    expect(rendered).toContain(`private_key: ${config.keys.txSender.privateKey}`);
    expect(rendered).not.toContain("0xoldprivatekey");
  });

  it("patches keyurl URLs to MinIO when runtime key IDs are set", () => {
    const config = buildConfig();
    config.runtime.fheKeyId = "04aabbccdd";
    config.runtime.crsKeyId = "05eeff0011";
    const template = [
      '    http_url: "http://gateway-node:8546"',
      '    read_http_url: "http://gateway-node:8546"',
      "    chain_id: 54321",
      '        url: "ws://gateway-node:8546"',
      "    private_key: 0xaaaa000000000000000000000000000000000000000000000000000000000001",
      '    decryption_address: "0xold1"',
      '    input_verification_address: "0xold2"',
      '  sql_database_url: "postgresql://postgres:postgres@relayer-db:5433/relayer_db"',
      "keyurl:",
      "  fhe_public_key:",
      '    url: "http://0.0.0.0:3001/publicKey.bin"',
      "  crs:",
      '    url: "http://0.0.0.0:3001/crs2048.bin"',
      "",
    ].join("\n");

    const rendered = renderRelayerConfig(template, config);
    expect(rendered).toContain('url: "http://fhevm-minio:9000/kms-public/PUB/PublicKey/04aabbccdd"');
    expect(rendered).toContain('url: "http://fhevm-minio:9000/kms-public/PUB/CRS/05eeff0011"');
    expect(rendered).not.toContain("0.0.0.0:3001");
  });

  it("leaves keyurl URLs unchanged when runtime key IDs are not set", () => {
    const config = buildConfig();
    const template = [
      '    http_url: "http://gateway-node:8546"',
      '    read_http_url: "http://gateway-node:8546"',
      "    chain_id: 54321",
      '        url: "ws://gateway-node:8546"',
      "    private_key: 0xaaaa000000000000000000000000000000000000000000000000000000000001",
      '    decryption_address: "0xold1"',
      '    input_verification_address: "0xold2"',
      '  sql_database_url: "postgresql://postgres:postgres@relayer-db:5433/relayer_db"',
      "keyurl:",
      "  fhe_public_key:",
      '    url: "http://0.0.0.0:3001/publicKey.bin"',
      "  crs:",
      '    url: "http://0.0.0.0:3001/crs2048.bin"',
      "",
    ].join("\n");

    const rendered = renderRelayerConfig(template, config);
    expect(rendered).toContain("0.0.0.0:3001/publicKey.bin");
    expect(rendered).toContain("0.0.0.0:3001/crs2048.bin");
  });

  it("writes generated relayer config atomically", async () => {
    const root = `.fhevm/test-relayer/${Date.now()}-${Math.random().toString(16).slice(2)}`;
    const templatePath = `${root}/template.yaml`;
    const outputPath = `${root}/local.yaml`;
    const config = buildConfig();
    const template = [
      '    http_url: "http://gateway-node:8546"',
      '    read_http_url: "http://gateway-node:8546"',
      "    chain_id: 54321",
      '        url: "ws://gateway-node:8546"',
      "    private_key: 0xaaaa000000000000000000000000000000000000000000000000000000000001",
      '    decryption_address: "0xold1"',
      '    input_verification_address: "0xold2"',
      '  sql_database_url: "postgresql://postgres:postgres@relayer-db:5433/relayer_db"',
      "",
    ].join("\n");
    await Bun.write(templatePath, template);

    const path = await generateRelayerConfigFile(config, { templatePath, outputPath });
    const content = await readFile(path, "utf8");

    expect(path).toBe(outputPath);
    expect(content).toContain('decryption_address: "0xdecryption"');
    expect(content).toContain('input_verification_address: "0xinput"');

    await rm(root, { recursive: true, force: true });
  });
});
