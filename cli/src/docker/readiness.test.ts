import { afterEach, beforeEach, describe, expect, test } from "bun:test";

import { ExitCode } from "../errors";
import type { ServiceDefinition } from "../config/service-map";

import { __internal, getDefaultTimeout, waitForAllReady, waitForReady } from "./readiness";

function makeService(partial: Partial<ServiceDefinition>): ServiceDefinition {
  return {
    name: "test-service",
    group: "infra",
    composeFile: "compose.yml",
    envFile: "database",
    containerName: "test-service",
    isOneShot: false,
    isBuildable: false,
    healthCheck: "docker-state",
    ...partial,
  };
}

let server: ReturnType<typeof Bun.serve> | undefined;

beforeEach(() => {
  delete Bun.env.FHEVM_WAIT_TIMEOUT;
});

afterEach(() => {
  server?.stop(true);
  server = undefined;
  delete Bun.env.FHEVM_WAIT_TIMEOUT;
});

describe("docker readiness", () => {
  test("resolves default timeouts by service type", () => {
    expect(getDefaultTimeout(makeService({}))).toBe(150_000);
    expect(getDefaultTimeout(makeService({ name: "relayer" }))).toBe(120_000);
    expect(getDefaultTimeout(makeService({ isOneShot: true }))).toBe(300_000);
    expect(getDefaultTimeout(makeService({ name: "gateway-sc-trigger-keygen", isOneShot: true }))).toBe(
      300_000,
    );
  });

  test("uses FHEVM_WAIT_TIMEOUT override when valid", () => {
    Bun.env.FHEVM_WAIT_TIMEOUT = "7";
    expect(__internal.getWaitTimeoutOverrideMs()).toBe(7_000);
    expect(getDefaultTimeout(makeService({ name: "fhevm-relayer" }))).toBe(7_000);
  });

  test("waits for http strategy to become ready", async () => {
    server = Bun.serve({
      port: 0,
      fetch: () => new Response("ok", { status: 200 }),
    });

    const service = makeService({
      name: "coprocessor-gw-listener",
      containerName: "fhevm-cli-non-existent-container",
      healthCheck: "http",
      healthEndpoint: server.url.href,
    });

    const result = await waitForReady(service, { timeoutMs: 1_000, pollIntervalMs: 25 });
    expect(result.ready).toBe(true);
    expect(result.service).toBe("coprocessor-gw-listener");
  });

  test("times out and returns docker error context", async () => {
    const service = makeService({
      name: "coprocessor-host-listener",
      containerName: "fhevm-cli-non-existent-container",
      healthCheck: "docker-state",
    });

    await expect(waitForReady(service, { timeoutMs: 60, pollIntervalMs: 20 })).rejects.toMatchObject({
      exitCode: ExitCode.DOCKER,
      step: "readiness",
      service: "coprocessor-host-listener",
    });
  });

  test("waits for all services in parallel", async () => {
    server = Bun.serve({
      port: 0,
      fetch: (req) => {
        if (req.method === "POST") {
          return Response.json({ jsonrpc: "2.0", id: 1, result: "0xd431" });
        }
        return new Response("ok", { status: 200 });
      },
    });

    const serviceA = makeService({
      name: "coprocessor-gw-listener",
      containerName: "fhevm-cli-non-existent-container",
      healthCheck: "http",
      healthEndpoint: server.url.href,
    });

    const serviceB = makeService({
      name: "gateway-node",
      containerName: "fhevm-cli-non-existent-container",
      healthCheck: "rpc",
      healthEndpoint: server.url.href,
    });

    const results = await waitForAllReady([serviceA, serviceB], {
      timeoutMs: 1_000,
      pollIntervalMs: 20,
    });

    expect(results).toHaveLength(2);
    expect(results.every((result) => result.ready)).toBe(true);
  });
});
