import path from "node:path";
import { mkdir, rm, writeFile } from "node:fs/promises";
import { describe, expect, test } from "bun:test";
import { TEST_SUITE_CONTAINER } from "./layout";
import { buildTestContainerArgs } from "./commands/test";
import { resumeOptionConflicts, shouldShowResumeHint } from "./flow/up-flow";
import type { State } from "./types";

const CLI_DIR = path.resolve(import.meta.dir, "..");
const STATE_ROOT = path.resolve(CLI_DIR, "..", "..", ".fhevm");
const STATE_FILE = path.join(STATE_ROOT, "state", "state.json");

const execCli = async (args: string[]) => {
  const proc = Bun.spawn([process.execPath, "run", "src/cli.ts", ...args], {
    cwd: CLI_DIR,
    stdout: "pipe",
    stderr: "pipe",
    env: process.env,
  });
  const [stdout, stderr, code] = await Promise.all([
    new Response(proc.stdout).text(),
    new Response(proc.stderr).text(),
    proc.exited,
  ]);
  return { stdout, stderr, code };
};

const normalizeCliOutput = (value: string) =>
  value
    .replace(/\u001b\[[0-9;]*m/g, "")
    .replace(/\s+/g, " ")
    .trim();

const withState = async (state: State, run: () => Promise<void>) => {
  await mkdir(path.dirname(STATE_FILE), { recursive: true });
  await writeFile(STATE_FILE, JSON.stringify(state, null, 2));
  try {
    await run();
  } finally {
    await rm(STATE_ROOT, { recursive: true, force: true });
  }
};

const persistedState = (target: State["target"] = "latest-main"): State => ({
  target,
  lockPath: "/tmp/latest-main.json",
  requiresGitHub: true,
  versions: {
    target,
    lockName: `${target}.json`,
    env: {} as State["versions"]["env"],
    sources: [],
  },
  overrides: [],
  scenario: {
    version: 1,
    kind: "coprocessor-consensus",
    origin: "default",
    hostChains: [{ key: "host", chainId: "12345", rpcPort: 8545 }],
    topology: { count: 1, threshold: 1 },
    instances: [{ index: 0, source: { mode: "inherit" }, env: {}, args: {} }],
  },
  completedSteps: ["preflight"],
  updatedAt: "2026-03-19T00:00:00.000Z",
});

const bootstrappedState = (target: State["target"] = "latest-main"): State => ({
  ...persistedState(target),
  discovery: {
    gateway: {} as NonNullable<State["discovery"]>["gateway"],
    hosts: { host: {} as NonNullable<State["discovery"]>["hosts"][string] },
    endpoints: {
      gateway: { http: "http://127.0.0.1:8545", ws: "ws://127.0.0.1:8546" },
      hosts: { host: { http: "http://127.0.0.1:9545", ws: "ws://127.0.0.1:9546" } },
      minioExternal: "http://127.0.0.1:9000",
      minioInternal: "http://minio:9000",
    },
    kmsSigner: "0x0000000000000000000000000000000000000014",
    fheKeyId: "a".repeat(64),
    crsKeyId: "b".repeat(64),
    actualFheKeyId: "a".repeat(64),
    actualCrsKeyId: "b".repeat(64),
  },
  completedSteps: ["bootstrap"],
});

describe("cli", () => {
  test("prints root help", async () => {
    const result = await execCli(["--help"]);
    expect(result.code).toBe(0);
    expect(result.stdout).toContain("COMMANDS");
    expect(result.stdout).toContain("up");
    expect(result.stderr).toBe("");
  });

  test("prints subcommand help without executing up", async () => {
    const result = await execCli(["up", "--help"]);
    const output = normalizeCliOutput(result.stdout);
    expect(result.code).toBe(0);
    expect(output).toContain("Boot the fhevm stack");
    expect(output).toContain("fhevm-cli up");
    expect(output).toContain("--target");
    expect(result.stdout).not.toContain("[up] target=");
  });

  test("prints test help", async () => {
    const result = await execCli(["test", "--help"]);
    const output = normalizeCliOutput(result.stdout);
    expect(result.code).toBe(0);
    expect(output).toContain("fhevm-cli test");
    expect(output).toContain("[TESTNAME]");
  });

  test("lists bundled test profiles", async () => {
    const result = await execCli(["test", "list"]);
    expect(result.code).toBe(0);
    expect(result.stdout).toContain("standard");
    expect(result.stdout).toContain("multi-chain-isolation");
  });

  test("lists bundled scenarios", async () => {
    const result = await execCli(["scenario", "list"]);
    expect(result.code).toBe(0);
    expect(result.stdout).toContain("two-of-two");
  });

  test("rejects unsupported targets", async () => {
    const result = await execCli(["up", "--target", "bogus"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("Unsupported target bogus");
  });

  test("requires --sha for sha target", async () => {
    const result = await execCli(["up", "--target", "sha"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("--target sha requires --sha");
  });

  test("rejects combining resume with an explicit target", async () => {
    const result = await execCli(["up", "--target", "latest-main", "--resume"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("--resume cannot be used with --target");
  });

  test("invalid sha does not print the resume hint", async () => {
    await withState(persistedState(), async () => {
      const result = await execCli(["up", "--target", "sha", "--sha", "invalidhex"]);
      expect(result.code).toBe(1);
      expect(result.stderr).toContain("Invalid sha invalidhex; expected 7 or 40 hex characters");
      expect(result.stderr).not.toContain("Hint: run with --resume");
    });
  });

  test("validates pause scope", async () => {
    const result = await execCli(["pause", "nope"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("pause expects `host` or `gateway`");
  });

  test("lists valid overrides when override parsing fails", async () => {
    const result = await execCli(["up", "--override", "bogus"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("Valid: all, coprocessor, kms-connector, gateway-contracts, host-contracts, test-suite");
  });

  test("prints logs help with an optional service argument", async () => {
    const result = await execCli(["logs", "--help"]);
    const output = normalizeCliOutput(result.stdout);
    expect(result.code).toBe(0);
    expect(output).toContain("fhevm-cli logs");
    expect(output).toContain("[SERVICE]");
    expect(output).toContain("first running fhevm container");
  });

  test("places extra docker exec flags before the test container", () => {
    expect(buildTestContainerArgs(["./run-tests.sh"], ["-e", "GATEWAY_RPC_URL="])).toEqual([
      "docker",
      "exec",
      "-e",
      "npm_config_update_notifier=false",
      "-e",
      "NPM_CONFIG_UPDATE_NOTIFIER=false",
      "-e",
      "GATEWAY_RPC_URL=",
      TEST_SUITE_CONTAINER,
      "./run-tests.sh",
    ]);
  });

  test("resume rejects any explicit target override", () => {
    expect(
      resumeOptionConflicts(persistedState("latest-supported"), {
        requestedTarget: "latest-supported",
        sha: undefined,
        lockFile: undefined,
        scenarioPath: undefined,
        overrides: [],
        allowSchemaMismatch: false,
        reset: false,
      }),
    ).toEqual(["target=latest-supported"]);
  });

  test("resume hint is suppressed for explicit fresh-stack flags", () => {
    expect(shouldShowResumeHint(["up"])).toBe(true);
    expect(shouldShowResumeHint(["up", "--target", "sha", "--sha", "badbad"])).toBe(false);
  });

  test("gates multi-chain isolation before launching tests on a single-chain stack", async () => {
    await withState(bootstrappedState(), async () => {
      const result = await execCli(["test", "multi-chain-isolation"]);
      expect(result.code).toBe(1);
      expect(result.stderr).toContain("multi-chain-isolation requires a multi-chain topology");
    });
  });
});
