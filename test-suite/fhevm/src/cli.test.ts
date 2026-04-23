import path from "node:path";
import { mkdir, writeFile } from "node:fs/promises";
import { describe, expect, test } from "bun:test";
import { DEFAULT_GATEWAY_RPC_PORT, DEFAULT_HOST_RPC_PORT, MINIO_PORT, STANDARD_TEST_PROFILES, TEST_SUITE_CONTAINER } from "./layout";
import {
  buildTestContainerArgs,
  dbRevertDeleteExpectations,
  dbRevertTargetBlock,
  keyBootstrapLogArgs,
  validateNamedProfileGrep,
  waitForKeyBootstrap,
} from "./commands/test";
import { resumeOptionConflicts, shouldShowResumeHint } from "./flow/up-flow";
import { resolveLogsFollow } from "./cli";
import { withTempStateDir } from "./test-state";
import { testDefaultScenario } from "./test-fixtures";
import type { State } from "./types";

const CLI_DIR = path.resolve(import.meta.dir, "..");

const execCli = async (args: string[], env: Record<string, string> = {}) => {
  const proc = Bun.spawn([process.execPath, "run", "src/cli.ts", ...args], {
    cwd: CLI_DIR,
    stdout: "pipe",
    stderr: "pipe",
    env: { ...process.env, ...env },
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

const withState = (state: State, run: (env: Record<string, string>) => Promise<void>) =>
  withTempStateDir(async (stateDir) => {
    const stateFile = path.join(stateDir, "state", "state.json");
    await mkdir(path.dirname(stateFile), { recursive: true });
    await writeFile(stateFile, JSON.stringify(state, null, 2));
    await run({ FHEVM_STATE_DIR: stateDir });
  });

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
  scenario: testDefaultScenario(),
  completedSteps: ["preflight"],
  updatedAt: "2026-03-19T00:00:00.000Z",
});

const bootstrappedState = (target: State["target"] = "latest-main"): State => ({
  ...persistedState(target),
  discovery: {
    gateway: {} as NonNullable<State["discovery"]>["gateway"],
    hosts: { host: {} as NonNullable<State["discovery"]>["hosts"][string] },
    endpoints: {
      gateway: { http: `http://127.0.0.1:${DEFAULT_GATEWAY_RPC_PORT}`, ws: `ws://127.0.0.1:${DEFAULT_GATEWAY_RPC_PORT}` },
      hosts: { host: { http: `http://127.0.0.1:${DEFAULT_HOST_RPC_PORT}`, ws: `ws://127.0.0.1:${DEFAULT_HOST_RPC_PORT}` } },
      minioExternal: `http://127.0.0.1:${MINIO_PORT}`,
      minioInternal: `http://minio:${MINIO_PORT}`,
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
    expect(output).toContain("preflight, resolve, generate");
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
    expect(result.stdout).toContain("ciphertext-drift - standard, 2+ coprocessors");
  });

  test("standard suite includes multi-chain isolation coverage", () => {
    expect(STANDARD_TEST_PROFILES).toContain("multi-chain-isolation");
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

  test("rejects --ref without a sha target", async () => {
    const result = await execCli(["up", "--target", "latest-main", "--ref", "release/0.12.x"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("--ref requires --target sha");
  });

  test("rejects combining resume with an explicit target", async () => {
    const result = await execCli(["up", "--target", "latest-main", "--resume"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("--resume cannot be used with --target");
  });

  test("invalid sha does not print the resume hint", async () => {
    await withState(persistedState(), async (env) => {
      const result = await execCli(["up", "--target", "sha", "--sha", "invalidhex"], env);
      expect(result.code).toBe(1);
      expect(result.stderr).toContain("Invalid sha invalidhex; expected 7 or 40 hex characters");
      expect(result.stderr).not.toContain("Hint: run with --resume");
    });
  });

  test("invalid sha with equals-form flags does not print the resume hint", async () => {
    await withState(persistedState(), async (env) => {
      const result = await execCli(["up", "--target=sha", "--sha=invalidhex"], env);
      expect(result.code).toBe(1);
      expect(result.stderr).toContain("Invalid sha invalidhex; expected 7 or 40 hex characters");
      expect(result.stderr).not.toContain("Hint: run with --resume");
    });
  });

  test("sha target accepts ref before validating the sha value", async () => {
    await withState(persistedState(), async (env) => {
      const result = await execCli(["up", "--target", "sha", "--sha", "invalidhex", "--ref", "release/0.12.x"], env);
      expect(result.code).toBe(1);
      expect(result.stderr).toContain("Invalid sha invalidhex; expected 7 or 40 hex characters");
      expect(result.stderr).not.toContain("--ref requires --target sha");
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
    expect(result.stderr).toContain("Valid: all, coprocessor, kms-connector, relayer, gateway-contracts, host-contracts, test-suite");
  });

  test("prints logs help with an optional service argument", async () => {
    const result = await execCli(["logs", "--help"]);
    const output = normalizeCliOutput(result.stdout);
    expect(result.code).toBe(0);
    expect(output).toContain("fhevm-cli logs");
    expect(output).toContain("[SERVICE]");
    expect(output).toContain("--no-follow");
    expect(output).toContain("first running fhevm container");
  });

  test("prints clean help with keep-images semantics", async () => {
    const result = await execCli(["clean", "--help"]);
    const output = normalizeCliOutput(result.stdout);
    expect(result.code).toBe(0);
    expect(output).toContain("fhevm-cli clean");
    expect(output).toContain("--keep-images");
    expect(output).not.toContain("--images");
  });

  test("resolves logs follow mode from citty's negated boolean", () => {
    expect(resolveLogsFollow({})).toBe(true);
    expect(resolveLogsFollow({ follow: true })).toBe(true);
    expect(resolveLogsFollow({ follow: false })).toBe(false);
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

  test("db-state-revert targets the block before the seed range", () => {
    expect(dbRevertTargetBlock(370)).toBe(369);
    expect(() => dbRevertTargetBlock(1)).toThrow("db-state-revert requires a positive seed boundary");
  });

  test("db-state-revert only expects seeded tables to shrink", () => {
    expect(
      dbRevertDeleteExpectations(
        {
          computationsDone: 11,
          computationsTotal: 11,
          allowedHandles: 19,
          pbsComputations: 11,
          ciphertextDigest: 11,
          ciphertexts: 11,
          ciphertexts128: 2,
        },
        {
          computationsDone: 13,
          computationsTotal: 13,
          allowedHandles: 22,
          pbsComputations: 12,
          ciphertextDigest: 12,
          ciphertexts: 12,
          ciphertexts128: 2,
        },
      ),
    ).toEqual(["computationsDone", "computationsTotal", "allowedHandles", "pbsComputations", "ciphertextDigest", "ciphertexts"]);
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
    await withState(bootstrappedState(), async (env) => {
      const result = await execCli(["test", "multi-chain-isolation"], env);
      expect(result.code).toBe(1);
      expect(result.stderr).toContain("multi-chain-isolation requires a multi-chain topology");
    });
  });

  test("drift profile rejects shared networks explicitly", async () => {
    await withState(bootstrappedState(), async (env) => {
      const result = await execCli(["test", "ciphertext-drift", "--network", "sepolia"], env);
      expect(result.code).toBe(1);
      expect(result.stderr).toContain("ciphertext-drift is not allowed on sepolia");
    });
  });

  test("drift profile honors the requested non-live network", async () => {
    await withState(bootstrappedState(), async (env) => {
      const result = await execCli(["test", "ciphertext-drift", "--network", "devnet"], env);
      expect(result.code).toBe(1);
      expect(result.stderr).not.toContain("not allowed on devnet");
    });
  });

  test("grep-backed named profiles accept targeted grep narrowing", () => {
    expect(() => validateNamedProfileGrep("operators", "manual")).not.toThrow();
    expect(() => validateNamedProfileGrep("erc20", "manual")).not.toThrow();
    expect(() => validateNamedProfileGrep("ciphertext-drift", "manual")).toThrow(
      "`fhevm-cli test ciphertext-drift` does not accept `--grep`; use either a named profile or a custom grep",
    );
  });

  test("named profile grep narrowing preserves both the profile and custom filters", async () => {
    const { narrowedProfileGrep } = await import("./commands/test");
    expect(narrowedProfileGrep("erc20", "manual")).toBe("(?=.*(?:erc20))(?=.*(?:manual))");
    expect(narrowedProfileGrep("erc20")).toBe("erc20");
  });

  test("rejects unknown flags on destructive commands", async () => {
    const result = await execCli(["clean", "--keep-imagse"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("Unknown option --keep-imagse");
  });

  test("rejects unknown flags on test commands", async () => {
    const result = await execCli(["test", "list", "--not-a-real-flag"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("Unknown option --not-a-real-flag");
  });

  test("rejects unexpected extra positionals", async () => {
    const cleanResult = await execCli(["clean", "foo"]);
    expect(cleanResult.code).toBe(1);
    expect(cleanResult.stderr).toContain("Unexpected positional argument foo");

    const upResult = await execCli(["up", "latest-supported", "--dry-run"]);
    expect(upResult.code).toBe(1);
    expect(upResult.stderr).toContain("Unexpected positional argument latest-supported");
  });

  test("waitForKeyBootstrap succeeds once threshold sns-workers report keysets", async () => {
    const state = bootstrappedState();
    state.scenario.topology = { count: 2, threshold: 2 };
    state.scenario.instances = [
      { index: 0, source: { mode: "inherit" }, env: {}, args: {} },
      { index: 1, source: { mode: "inherit" }, env: {}, args: {} },
    ];
    await expect(
      waitForKeyBootstrap(state, {
        readLogs: async (container) => ({
          stdout: container.includes("sns-worker") ? "Fetched keyset" : "",
          stderr: "",
        }),
        sleep: async () => undefined,
      }),
    ).resolves.toBeUndefined();
  });

  test("waitForKeyBootstrap fails when threshold sns-workers never report keysets", async () => {
    const state = bootstrappedState();
    state.scenario.topology = { count: 2, threshold: 2 };
    state.scenario.instances = [
      { index: 0, source: { mode: "inherit" }, env: {}, args: {} },
      { index: 1, source: { mode: "inherit" }, env: {}, args: {} },
    ];
    await expect(
      waitForKeyBootstrap(state, {
        readLogs: async (container) => ({
          stdout: container === "coprocessor-sns-worker" ? "Fetched keyset" : "",
          stderr: "",
        }),
        sleep: async () => undefined,
      }),
    ).rejects.toThrow("key bootstrap did not reach threshold");
  });

  test("key bootstrap log lookup does not expire on long-lived stacks", () => {
    expect(keyBootstrapLogArgs("coprocessor-sns-worker")).toEqual(["docker", "logs", "coprocessor-sns-worker"]);
  });
});
