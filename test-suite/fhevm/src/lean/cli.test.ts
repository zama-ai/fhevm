import path from "node:path";
import { mkdir, rm, writeFile } from "node:fs/promises";
import { describe, expect, test } from "bun:test";
import { TEST_SUITE_CONTAINER } from "../layout";
import { buildTestContainerArgs } from "./test";
import { resumeOptionConflicts, shouldShowResumeHint } from "./stack";
import type { State } from "../types";

const CLI_DIR = path.resolve(import.meta.dir, "..", "..");
const STATE_ROOT = path.resolve(CLI_DIR, "..", "..", ".fhevm");
const STATE_FILE = path.join(STATE_ROOT, "state", "state.json");

const shellEscape = (value: string) => `'${value.replaceAll("'", `'\\''`)}'`;

const execCli = async (args: string[]) => {
  const proc = Bun.spawn(["zsh", "-lc", `bun run src/cli.ts ${args.map(shellEscape).join(" ")}`], {
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
    topology: { count: 1, threshold: 1 },
    instances: [{ index: 0, source: { mode: "inherit" }, env: {}, args: {} }],
  },
  completedSteps: ["preflight"],
  updatedAt: "2026-03-19T00:00:00.000Z",
});

describe("cli", () => {
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

  test("invalid sha does not print the resume hint", async () => {
    await withState(persistedState(), async () => {
      const result = await execCli(["up", "--target", "sha", "--sha", "invalidhex"]);
      expect(result.code).toBe(1);
      expect(result.stderr).toContain("Invalid sha invalidhex; expected 7 or 40 hex characters");
      expect(result.stderr).not.toContain("Hint: run with --resume");
    });
  });

  test("rejects scenario with coprocessor override", async () => {
    const result = await execCli([
      "up",
      "--scenario",
      "two-of-two",
      "--override",
      "coprocessor",
      "--dry-run",
    ]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("--scenario cannot be combined with --override coprocessor");
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
});
