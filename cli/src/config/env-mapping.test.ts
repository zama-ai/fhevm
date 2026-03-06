import { describe, expect, it } from "bun:test";
import { deriveAllKeys } from "./keys";
import {
  ENV_GENERATORS,
  generateCoprocessorEnv,
  generateCoprocessorEnvWithOverrides,
  generateDatabaseEnv,
  generateGatewayScEnv,
  generateHostNodeEnv,
  generateKmsConnectorEnv,
} from "./env-mapping";
import { DEFAULT_MNEMONIC, createDefaultConfig, type FhevmConfig } from "./model";

function buildConfig(overrides: Partial<FhevmConfig> = {}): FhevmConfig {
  const keys = deriveAllKeys(DEFAULT_MNEMONIC, 3, 1);
  return createDefaultConfig(keys, overrides);
}

describe("env mapping", () => {
  it("generates non-empty envs for all service env files", () => {
    const config = buildConfig();

    for (const generator of Object.values(ENV_GENERATORS)) {
      const values = generator(config);
      expect(Object.keys(values).length).toBeGreaterThan(0);
      for (const value of Object.values(values)) {
        expect(typeof value).toBe("string");
      }
    }
  });

  it("maps database and chain ids consistently across envs", () => {
    const config = buildConfig();
    const coprocessorEnv = generateCoprocessorEnv(config);
    const kmsConnectorEnv = generateKmsConnectorEnv(config);

    expect(coprocessorEnv.DATABASE_URL).toContain("/coprocessor");
    expect(kmsConnectorEnv.KMS_CONNECTOR_GATEWAY_CHAIN_ID).toBe(String(config.chainIds.gateway));
  });

  it("generates gateway arrays for multi-coprocessor topology", () => {
    const config = buildConfig({
      topology: {
        numKmsNodes: 1,
        numCoprocessors: 3,
        numCustodians: 3,
        numPausers: 2,
        numHostChains: 1,
      },
    });
    const gatewayEnv = generateGatewayScEnv(config);

    expect(gatewayEnv.NUM_COPROCESSORS).toBe("3");
    expect(gatewayEnv.COPROCESSOR_TX_SENDER_ADDRESS_0).toBeTruthy();
    expect(gatewayEnv.COPROCESSOR_TX_SENDER_ADDRESS_1).toBeTruthy();
    expect(gatewayEnv.COPROCESSOR_TX_SENDER_ADDRESS_2).toBeTruthy();
  });

  it("database and host node env generators have expected shapes", () => {
    const config = buildConfig();
    const databaseEnv = generateDatabaseEnv(config);
    const hostNodeEnv = generateHostNodeEnv(config);

    expect(Object.keys(databaseEnv)).toEqual(["POSTGRES_USER", "POSTGRES_PASSWORD"]);
    expect(Object.keys(hostNodeEnv)).toEqual(["MNEMONIC"]);
  });

  it("uses discovered minio ip in coprocessor endpoint when present", () => {
    const config = buildConfig({ runtime: { minioIp: "10.0.0.99" } });
    const coprocessorEnv = generateCoprocessorEnv(config);

    expect(coprocessorEnv.AWS_ENDPOINT_URL).toBe("http://10.0.0.99:9000");
  });

  it("allows overriding instance-specific coprocessor values", () => {
    const config = buildConfig();
    const env = generateCoprocessorEnvWithOverrides(config, {
      databaseUrl: "postgresql://postgres:postgres@localhost:5432/coprocessor_2",
      txSenderPrivateKey: "0xdeadbeef",
    });

    expect(env.DATABASE_URL).toContain("/coprocessor_2");
    expect(env.TX_SENDER_PRIVATE_KEY).toBe("0xdeadbeef");
  });
});
