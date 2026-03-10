import fs from "node:fs/promises";
import path from "node:path";

import { describe, expect, test } from "bun:test";

import { composeUp, resolvedComposeEnv, serviceNameList } from "./artifacts";
import { composePath, TEMPLATE_COMPOSE_DIR } from "./layout";
import { stubState } from "./test-helpers";

describe("resolvedComposeEnv", () => {
  test("includes version env and COMPOSE_IGNORE_ORPHANS", () => {
    const env = resolvedComposeEnv(stubState());
    expect(env.GATEWAY_VERSION).toBe("v0.11.0");
    expect(env.CORE_VERSION).toBe("v0.13.0");
    expect(env.COMPOSE_IGNORE_ORPHANS).toBe("true");
  });
});

describe("serviceNameList", () => {
  const state = stubState();

  test("returns empty for non-coprocessor components", () => {
    expect(serviceNameList(state, "relayer")).toEqual([]);
    expect(serviceNameList(state, "database")).toEqual([]);
    expect(serviceNameList(state, "minio")).toEqual([]);
  });

  test("returns single-instance service names for count=1", () => {
    const names = serviceNameList(state, "coprocessor");
    expect(names).toEqual([
      "coprocessor-db-migration",
      "coprocessor-host-listener",
      "coprocessor-host-listener-poller",
      "coprocessor-gw-listener",
      "coprocessor-tfhe-worker",
      "coprocessor-zkproof-worker",
      "coprocessor-sns-worker",
      "coprocessor-transaction-sender",
    ]);
  });

  test("returns multi-instance service names for count=2", () => {
    const names = serviceNameList(stubState({ count: 2 }), "coprocessor");
    expect(names).toContain("coprocessor-db-migration");
    expect(names).toContain("coprocessor-host-listener");
    expect(names).toContain("coprocessor1-db-migration");
    expect(names).toContain("coprocessor1-host-listener");
    expect(names).toContain("coprocessor1-tfhe-worker");
    expect(names.length).toBe(16);
  });

  test("returns multi-instance service names for count=3", () => {
    const names = serviceNameList(stubState({ count: 3 }), "coprocessor");
    expect(names).toContain("coprocessor-db-migration");
    expect(names).toContain("coprocessor1-db-migration");
    expect(names).toContain("coprocessor2-db-migration");
    expect(names.length).toBe(24);
  });
});

describe("compose templates", () => {
  test("transaction sender does not hardcode host chain url", async () => {
    const template = await fs.readFile(
      path.join(TEMPLATE_COMPOSE_DIR, "coprocessor-docker-compose.yml"),
      "utf8",
    );
    expect(template.includes("--host-chain-url")).toBe(false);
  });

  test("coprocessor local builds are serialized", async () => {
    const file = composePath("coprocessor");
    await fs.mkdir(path.dirname(file), { recursive: true });
    await fs.writeFile(
      file,
      [
        "services:",
        "  coprocessor-host-listener:",
        "    image: ghcr.io/zama-ai/fhevm/coprocessor/host-listener:fhevm-local",
        "  coprocessor-gw-listener:",
        "    image: ghcr.io/zama-ai/fhevm/coprocessor/gw-listener:fhevm-local",
      ].join("\n"),
    );
    const calls: string[][] = [];
    try {
      await composeUp(
        "coprocessor",
        stubState({ overrides: [{ group: "coprocessor" }] }),
        {
          runner: async () => ({ stdout: "", stderr: "", code: 1 }),
          liveRunner: async (argv) => {
            calls.push(argv);
            return 0;
          },
        },
        async () => {},
        () => {},
      );
    } finally {
      await fs.rm(file, { force: true });
    }
    expect(calls.filter((argv) => argv.includes("build"))).toEqual([
      [
        "docker",
        "compose",
        "-p",
        "fhevm",
        "--env-file",
        path.join(path.dirname(path.dirname(file)), "env", "versions.env"),
        "-f",
        file,
        "build",
        "coprocessor-host-listener",
      ],
      [
        "docker",
        "compose",
        "-p",
        "fhevm",
        "--env-file",
        path.join(path.dirname(path.dirname(file)), "env", "versions.env"),
        "-f",
        file,
        "build",
        "coprocessor-gw-listener",
      ],
    ]);
  });
});
