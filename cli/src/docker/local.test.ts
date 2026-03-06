import { describe, expect, test } from "bun:test";

import { ExitCode } from "../errors";

import {
  LOCAL_PORT_MAP,
  filterDockerServices,
  getActiveComposeFiles,
  getLocalRunHints,
  getRunHintForService,
  resolveExcludedServices,
  validateLocalComponents,
} from "./local";

describe("docker local mode", () => {
  test("validates known components", () => {
    expect(() => validateLocalComponents(["coprocessor", "tfhe-worker"])).not.toThrow();
  });

  test("rejects unknown components", () => {
    expect(() => validateLocalComponents(["unknown"])).toThrowError(
      expect.objectContaining({ exitCode: ExitCode.CONFIG }),
    );
  });

  test("resolves exclusions with deduplication", () => {
    const excluded = resolveExcludedServices(["coprocessor", "tfhe-worker"]);

    expect(excluded).toContain("coprocessor-tfhe-worker");
    expect(excluded.filter((name) => name === "coprocessor-tfhe-worker")).toHaveLength(1);
    expect(excluded).toHaveLength(8);
  });

  test("filters services for docker mode", () => {
    const filtered = filterDockerServices(["coprocessor"]);
    expect(filtered.find((service) => service.group === "coprocessor")).toBeUndefined();
    expect(filtered.length).toBeGreaterThan(0);
  });

  test("computes active compose files without fully excluded groups", () => {
    const composeFiles = getActiveComposeFiles(["coprocessor"]);
    expect(composeFiles).not.toContain("test-suite/fhevm/docker-compose/coprocessor-docker-compose.yml");
  });

  test("emits local run hints for excluded services", () => {
    const hints = getLocalRunHints(["tfhe-worker", "relayer"]);

    expect(hints.get("coprocessor-tfhe-worker")).toContain("cargo run --bin tfhe_worker");
    expect(hints.get("relayer")).toContain("source");
    expect(hints.get("relayer")).toContain("relayer");
  });

  test("returns run hint for a single service", () => {
    expect(getRunHintForService("coprocessor-tfhe-worker")).toContain("source .fhevm/env/.env.coprocessor.local");
    expect(getRunHintForService("minio")).toBeUndefined();
  });

  test("includes required local port map", () => {
    expect(LOCAL_PORT_MAP.postgres.port).toBe(5432);
    expect(LOCAL_PORT_MAP.relayerPostgres.port).toBe(5433);
    expect(LOCAL_PORT_MAP.hostRpc.port).toBe(8545);
    expect(LOCAL_PORT_MAP.gatewayRpc.port).toBe(8546);
    expect(LOCAL_PORT_MAP.minioApi.port).toBe(9000);
    expect(LOCAL_PORT_MAP.kmsCore.port).toBe(50051);
    expect(LOCAL_PORT_MAP.relayerHttp.port).toBe(3000);
  });
});
