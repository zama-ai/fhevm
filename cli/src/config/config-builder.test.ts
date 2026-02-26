import { afterEach, describe, expect, it } from "bun:test";
import { ExitCode, FhevmCliError } from "../errors";
import { applyLocalMode, buildConfig, buildConfigVariants, computeThresholds } from "./config-builder";

function clearEnv(): void {
  delete process.env.FHEVM_COPROCESSORS;
  delete process.env.FHEVM_KMS_NODES;
  delete process.env.FHEVM_MNEMONIC;
  delete process.env.FHEVM_HOST_CHAIN_ID;
  delete process.env.FHEVM_GATEWAY_CHAIN_ID;
  delete process.env.FHEVM_DB_USER;
  delete process.env.FHEVM_DB_PASSWORD;
}

afterEach(() => {
  clearEnv();
});

describe("config builder", () => {
  it("builds default config", () => {
    const config = buildConfig();
    expect(config.topology.numCoprocessors).toBe(1);
    expect(config.topology.numKmsNodes).toBe(1);
    expect(config.thresholds.coprocessor).toBe(1);
    expect(config.db.user).toBe("postgres");
  });

  it("builds multi-coprocessor config", () => {
    const config = buildConfig({ numCoprocessors: 3 });
    expect(config.topology.numCoprocessors).toBe(3);
    expect(config.thresholds.coprocessor).toBe(2);
    expect(config.keys.coprocessors).toHaveLength(3);
  });

  it("computes thresholds", () => {
    expect(computeThresholds(1, 1)).toEqual({
      publicDecryption: 1,
      userDecryption: 1,
      kmsGeneration: 1,
      coprocessor: 1,
      mpc: 0,
    });
    expect(computeThresholds(3, 2)).toEqual({
      publicDecryption: 2,
      userDecryption: 2,
      kmsGeneration: 2,
      coprocessor: 2,
      mpc: 1,
    });
  });

  it("applies local mode endpoint swaps", () => {
    const config = buildConfig();
    const local = applyLocalMode(config, ["coprocessor"]);
    expect(local.db.host).toBe("localhost");
    expect(local.rpc.gatewayHttp).toBe("http://localhost:8546");
    expect(local.minio.endpoint).toBe("http://localhost:9000");
  });

  it("keeps docker config endpoints when --local is requested", () => {
    const variants = buildConfigVariants({ local: ["coprocessor"] });
    expect(variants.docker.db.host).toBe("coprocessor-and-kms-db");
    expect(variants.docker.rpc.gatewayHttp).toBe("http://gateway-node:8546");
    expect(variants.local?.db.host).toBe("localhost");
    expect(variants.local?.rpc.gatewayHttp).toBe("http://localhost:8546");
  });

  it("respects env overrides", () => {
    process.env.FHEVM_HOST_CHAIN_ID = "99999";
    process.env.FHEVM_GATEWAY_CHAIN_ID = "88888";
    process.env.FHEVM_DB_USER = "alice";
    process.env.FHEVM_DB_PASSWORD = "secret";

    const config = buildConfig();
    expect(config.chainIds.host).toBe(99999);
    expect(config.chainIds.gateway).toBe(88888);
    expect(config.db.user).toBe("alice");
    expect(config.db.password).toBe("secret");
  });

  it("throws config error for invalid coprocessor count", () => {
    expect(() => buildConfig({ numCoprocessors: 9 })).toThrow(FhevmCliError);
    try {
      buildConfig({ numCoprocessors: 9 });
    } catch (error) {
      expect((error as FhevmCliError).exitCode).toBe(ExitCode.CONFIG);
    }
  });
});
