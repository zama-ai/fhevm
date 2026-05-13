import path from "node:path";
import { mkdir, writeFile } from "node:fs/promises";
import { describe, expect, test } from "bun:test";
import { DEFAULT_GATEWAY_RPC_PORT, DEFAULT_HOST_RPC_PORT, envPath, MINIO_PORT, STANDARD_TEST_PROFILES, TEST_SUITE_CONTAINER } from "./layout";
import {
  buildTestContainerArgs,
  dbRevertDeleteExpectations,
  dbRevertTargetBlock,
  keyBootstrapLogArgs,
  nativeTestEnv,
  parseTestRunner,
  prepareNativeTestEnv,
  validateNamedProfileGrep,
  waitForKeyBootstrap,
} from "./commands/test";
import { readEnvFile } from "./utils/fs";
import { REPLACE_TARGET_NAMES, parseBuildProfile, replaceBuildBinary, replaceBuildTargetDir, replaceTargetsForState } from "./commands/replace";
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
    expect(output).toContain("--runner");
  });

  test("lists bundled test profiles", async () => {
    const result = await execCli(["test", "list"]);
    expect(result.code).toBe(0);
    expect(result.stdout).toContain("standard");
    expect(result.stdout).toContain("multi-chain-isolation");
    expect(result.stdout).toContain("ciphertext-drift - 2+ coprocessors");
    expect(result.stdout).toContain("ciphertext-drift-auto-recovery - standard");
  });

  test("standard suite includes multi-chain isolation coverage", () => {
    expect(STANDARD_TEST_PROFILES).toContain("multi-chain-isolation");
  });

  test("parses test runner backends", () => {
    expect(parseTestRunner(undefined)).toBe("docker");
    expect(parseTestRunner("docker")).toBe("docker");
    expect(parseTestRunner("native")).toBe("native");
    expect(() => parseTestRunner("container")).toThrow("Unsupported test runner container");
  });

  test("rejects invalid test runner before listing profiles", async () => {
    const result = await execCli(["test", "list", "--runner", "container"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("Unsupported test runner container");
  });

  test("rewrites generated test env URLs for native runs", () => {
    expect(
      nativeTestEnv({
        RPC_URL: "http://host-node:8545",
        GATEWAY_RPC_URL: "http://gateway-node:8546",
        RELAYER_URL: "http://fhevm-relayer:3000/v2",
        HOST_CHAIN_1_RPC_URL: "http://host-node-1:8547",
        MAINNET_ETH_RPC_URL: "http://mainnet.example",
      }),
    ).toEqual({
      RPC_URL: "http://localhost:8545",
      GATEWAY_RPC_URL: "http://localhost:8546",
      RELAYER_URL: "http://localhost:3000/v2",
      HOST_CHAIN_1_RPC_URL: "http://localhost:8547",
      MAINNET_ETH_RPC_URL: "http://mainnet.example",
    });
  });

  test("native runner writes a host-reachable env and returns it as spawn vars", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("test-suite")), { recursive: true });
      await writeFile(envPath("test-suite"), "RPC_URL=http://host-node:8545\nMAINNET_ETH_RPC_URL=http://mainnet.example\n");
      await expect(prepareNativeTestEnv()).resolves.toEqual({
        RPC_URL: "http://localhost:8545",
        MAINNET_ETH_RPC_URL: "http://mainnet.example",
        DOTENV_CONFIG_PATH: envPath("test-suite.native"),
        FHEVM_NATIVE_RUNNER: "true",
      });
      await expect(readEnvFile(envPath("test-suite.native"))).resolves.toEqual({
        RPC_URL: "http://localhost:8545",
        MAINNET_ETH_RPC_URL: "http://mainnet.example",
      });
    });
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

  test("prints replace help with valid targets", async () => {
    const result = await execCli(["replace", "--help"]);
    const output = normalizeCliOutput(result.stdout);
    expect(result.code).toBe(0);
    expect(output).toContain("fhevm-cli replace");
    expect(output).toContain("coprocessor:tfhe-worker");
    expect(output).toContain("kms-connector:gw-listener");
    expect(output).toContain("--local-build");
    expect(output).toContain("--build-profile");
  });

  test("lists replace targets without a running stack", async () => {
    const result = await execCli(["replace", "list"]);
    expect(result.code).toBe(0);
    expect(result.stdout).toContain("coprocessor:tfhe-worker");
    expect(result.stdout).toContain("default binary: coprocessor/fhevm-engine/target/release/tfhe_worker");
    expect(result.stdout).toContain("local build: cached Linux cargo build for package tfhe-worker");
    expect(result.stdout).toContain("kms-connector:tx-sender");
  });

  test("parses replace build profiles", () => {
    expect(parseBuildProfile(undefined)).toBe("dev");
    expect(parseBuildProfile("release")).toBe("release");
    expect(parseBuildProfile("profiling.local")).toBe("profiling.local");
    expect(() => parseBuildProfile("../release")).toThrow("Invalid build profile");
    expect(() => parseBuildProfile("--release")).toThrow("Invalid build profile");
  });

  test("rejects combining replace --binary with --local-build", async () => {
    const result = await execCli(["replace", "coprocessor:tfhe-worker", "--binary", "/tmp/tfhe_worker", "--local-build"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("--binary cannot be combined with --local-build");
  });

  test("rejects replace --build-profile without --local-build", async () => {
    const result = await execCli(["replace", "coprocessor:tfhe-worker", "--build-profile", "release"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("--build-profile can only be used with --local-build");
  });

  test("accepts several replace targets at the CLI boundary", async () => {
    const result = await execCli(["replace", "coprocessor:tfhe-worker", "coprocessor:zkproof-worker"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("replace requires persisted stack state");
    expect(result.stderr).not.toContain("Unexpected positional argument");
  });

  test("maps replace targets onto active topology containers", () => {
    const state = bootstrappedState();
    state.scenario = testDefaultScenario({
      topology: { count: 2, threshold: 1 },
      hostChains: [
        { key: "host", chainId: "12345", rpcPort: 8545 },
        { key: "side", chainId: "12346", rpcPort: 8547 },
      ],
    });
    expect(replaceTargetsForState(state, "coprocessor:tfhe-worker").containers).toEqual([
      "coprocessor-tfhe-worker",
      "coprocessor1-tfhe-worker",
    ]);
    expect(replaceTargetsForState(state, "coprocessor:tfhe-worker").localBuild.package).toBe("tfhe-worker");
    expect(replaceTargetsForState(state, "coprocessor:host-listener").containers).toEqual([
      "coprocessor-host-listener",
      "coprocessor1-host-listener",
      "coprocessor-host-listener-side",
      "coprocessor1-host-listener-side",
    ]);
    expect(REPLACE_TARGET_NAMES).toContain("kms-connector:kms-worker");
  });

  test("local replace builds share one cargo target cache per workspace", () => {
    const state = bootstrappedState();
    const tfhe = replaceTargetsForState(state, "coprocessor:tfhe-worker").localBuild;
    const zkproof = replaceTargetsForState(state, "coprocessor:zkproof-worker").localBuild;
    const kmsWorker = replaceTargetsForState(state, "kms-connector:kms-worker").localBuild;

    expect(replaceBuildTargetDir(tfhe)).toBe(replaceBuildTargetDir(zkproof));
    expect(replaceBuildTargetDir(tfhe)).not.toBe(replaceBuildTargetDir(kmsWorker));
    expect(replaceBuildBinary(tfhe, "dev")).toContain("replace-build/coprocessor/target/debug/tfhe_worker");
    expect(replaceBuildBinary(tfhe, "release")).toContain("replace-build/coprocessor/target/release/tfhe_worker");
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
