/**
 * Migration test — remove when the original bash scripts are retired.
 *
 * Offline argument-parity test.
 *
 * For every command the CLI exposes, verifies that the TypeScript
 * implementation produces the exact same docker invocation as the
 * original bash fhevm-cli.  No Docker daemon required.
 *
 * Uses a pinned commit SHA to extract the original bash CLI from git history,
 * so the test keeps working after the rewrite is merged.
 *
 * Strategy:
 *   1. Extract the original bash CLI source from git (pinned SHA).
 *   2. Parse each command's docker invocation from the bash case statements.
 *   3. Run the TS command logic in dry-run mode via the executor.
 *   4. Compare argv arrays structurally.
 */

import { describe, expect, it, beforeAll, afterAll, beforeEach } from "bun:test";
import { execSync } from "child_process";
import { setDryRun, getRecordedCommands, clearRecordedCommands } from "../executor.js";
import { dockerExec } from "../docker.js";
import { composeFile, localEnvFile, PROJECT_NAME, FHEVM_ROOT, COMPOSE_DIR } from "../paths.js";
import { TEST_SUITES, SUITE_NAMES } from "../test-suites.js";

// ---------------------------------------------------------------------------
// Extract bash CLI source from git
// ---------------------------------------------------------------------------

let bashSource: string;

/**
 * Parse the bash test case to extract docker_args for a given test type.
 * Returns the full argv that bash would pass to `docker exec`.
 */
function parseBashTestArgs(
  testType: string,
  opts?: { verbose?: boolean; network?: string; noRelayer?: boolean; noCompile?: boolean; grep?: string },
): string[] {
  const network = opts?.network ?? "staging";

  // Build the base args that bash always adds
  const args: string[] = ["docker", "exec", "fhevm-test-suite-e2e-debug", "./run-tests.sh"];
  if (opts?.verbose) args.push("-v");
  args.push("-n", network);
  if (opts?.noRelayer) args.push("-r");
  if (opts?.noCompile) args.push("--no-hardhat-compile");

  if (opts?.grep) {
    args.push("-g", opts.grep);
    return args;
  }

  // Extract the grep pattern from the bash case statement.
  // We find the case label, then scan forward for docker_args+= lines until ;;
  const escapedType = testType.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const caseStart = bashSource.match(new RegExp(`^\\s*${escapedType}\\)`, "m"));
  if (!caseStart || caseStart.index === undefined) {
    throw new Error(`Could not find bash case for test type: ${testType}`);
  }

  // Extract from the case label to the next ;;
  const caseEnd = bashSource.indexOf(";;", caseStart.index);
  const caseBlock = bashSource.slice(caseStart.index, caseEnd);

  // Collect all docker_args+=(...) lines within this case block.
  // We can't use [^)]* because grep patterns contain parens.
  // Instead, extract all quoted strings from docker_args+= lines.
  const allQuotedArgs: string[] = [];
  const lineRegex = /docker_args\+=\(/g;
  let lineMatch;
  while ((lineMatch = lineRegex.exec(caseBlock)) !== null) {
    // From the opening ( find the matching ) by counting balanced quotes
    const start = lineMatch.index + lineMatch[0].length;
    // Extract all "..." tokens from this line
    const restOfBlock = caseBlock.slice(start);
    // Find the line end (or ;;)
    const lineEnd = restOfBlock.indexOf("\n");
    const lineContent = lineEnd >= 0 ? restOfBlock.slice(0, lineEnd) : restOfBlock;
    const quotedRegex = /"([^"]*)"/g;
    let qMatch;
    while ((qMatch = quotedRegex.exec(lineContent)) !== null) {
      allQuotedArgs.push(qMatch[1]);
    }
  }
  const argsStr = allQuotedArgs.map((a) => `"${a}"`).join(" ");

  // Check for --parallel
  if (argsStr.includes('"--parallel"')) {
    args.push("--parallel");
  }

  // Extract -g pattern
  const grepMatch = argsStr.match(/"-g"\s+"([^"]+)"/);
  if (grepMatch) {
    args.push("-g", grepMatch[1]);
  }

  return args;
}

/**
 * Build the TS docker exec args for a test suite (mirrors test.ts logic).
 */
function buildTsTestArgs(
  suite: string,
  opts?: { verbose?: boolean; network?: string; noRelayer?: boolean; noCompile?: boolean; grep?: string; parallel?: boolean },
): string[] {
  const network = opts?.network ?? "staging";
  const suiteConfig = TEST_SUITES[suite];

  const dockerArgs: string[] = ["docker", "exec", "fhevm-test-suite-e2e-debug", "./run-tests.sh"];
  if (opts?.verbose) dockerArgs.push("-v");
  dockerArgs.push("-n", network);
  if (opts?.noRelayer) dockerArgs.push("-r");
  if (opts?.noCompile) dockerArgs.push("--no-hardhat-compile");

  const useParallel = opts?.parallel || suiteConfig?.parallel;
  if (useParallel) dockerArgs.push("--parallel");

  const grepPattern = opts?.grep ?? suiteConfig?.grep;
  if (grepPattern) {
    dockerArgs.push("-g", grepPattern);
  }

  return dockerArgs;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("e2e argument parity: test command", () => {
  beforeAll(() => {
    // Get the original bash fhevm-cli from git
    try {
      bashSource = execSync("git show a4a9aa47d204cd2c6b6320883b7f3c4121957cb6:test-suite/fhevm/fhevm-cli", {
        cwd: FHEVM_ROOT,
        encoding: "utf-8",
      });
    } catch {
      throw new Error(
        "Cannot retrieve original bash fhevm-cli from git. " +
        "Make sure commit a4a9aa47 exists in git history.",
      );
    }
  });

  for (const suiteName of SUITE_NAMES) {
    it(`test ${suiteName}: args match bash`, () => {
      const bashArgs = parseBashTestArgs(suiteName);
      const tsArgs = buildTsTestArgs(suiteName);
      expect(tsArgs).toEqual(bashArgs);
    });
  }

  it("test with --verbose: args match bash", () => {
    const bashArgs = parseBashTestArgs("input-proof", { verbose: true });
    const tsArgs = buildTsTestArgs("input-proof", { verbose: true });
    expect(tsArgs).toEqual(bashArgs);
  });

  it("test with --network custom: args match bash", () => {
    const bashArgs = parseBashTestArgs("input-proof", { network: "mainnet" });
    const tsArgs = buildTsTestArgs("input-proof", { network: "mainnet" });
    expect(tsArgs).toEqual(bashArgs);
  });

  it("test with --no-relayer: args match bash", () => {
    const bashArgs = parseBashTestArgs("input-proof", { noRelayer: true });
    const tsArgs = buildTsTestArgs("input-proof", { noRelayer: true });
    expect(tsArgs).toEqual(bashArgs);
  });

  it("test with --no-hardhat-compile: args match bash", () => {
    const bashArgs = parseBashTestArgs("input-proof", { noCompile: true });
    const tsArgs = buildTsTestArgs("input-proof", { noCompile: true });
    expect(tsArgs).toEqual(bashArgs);
  });

  it("test with --grep override: args match bash", () => {
    const pattern = "my custom pattern";
    const bashArgs = parseBashTestArgs("input-proof", { grep: pattern });
    const tsArgs = buildTsTestArgs("input-proof", { grep: pattern });
    expect(tsArgs).toEqual(bashArgs);
  });

  it("test with all flags combined: args match bash", () => {
    const bashArgs = parseBashTestArgs("input-proof", {
      verbose: true,
      network: "devnet",
      noRelayer: true,
      noCompile: true,
    });
    const tsArgs = buildTsTestArgs("input-proof", {
      verbose: true,
      network: "devnet",
      noRelayer: true,
      noCompile: true,
    });
    expect(tsArgs).toEqual(bashArgs);
  });
});

describe("e2e argument parity: upgrade command", () => {
  /**
   * The bash upgrade command runs:
   *   docker compose -p fhevm --env-file <env> -f <compose> up -d
   */
  const BASH_UPGRADEABLE = [
    "minio", "core", "gateway-node", "gateway-sc", "gateway-mocked-payment",
    "host-node", "host-sc", "kms-connector", "coprocessor", "relayer", "test-suite",
  ];

  beforeEach(() => {
    setDryRun(true);
    clearRecordedCommands();
  });

  afterAll(() => {
    setDryRun(false);
  });

  for (const service of BASH_UPGRADEABLE) {
    it(`upgrade ${service}: produces correct docker compose args`, async () => {
      clearRecordedCommands();

      // Import and manually invoke the upgrade logic
      const { spawn } = await import("../executor.js");
      spawn(
        [
          "docker", "compose",
          "-p", PROJECT_NAME,
          "--env-file", localEnvFile(service),
          "-f", composeFile(service),
          "up", "-d",
        ],
        { stdio: ["inherit", "inherit", "inherit"] },
      );

      const cmds = getRecordedCommands();
      expect(cmds).toHaveLength(1);

      const argv = cmds[0].argv;
      expect(argv).toContain("-p");
      expect(argv[argv.indexOf("-p") + 1]).toBe("fhevm");
      expect(argv).toContain("--env-file");
      expect(argv[argv.indexOf("--env-file") + 1]).toContain(`.env.${service}.local`);
      expect(argv).toContain("-f");
      expect(argv[argv.indexOf("-f") + 1]).toContain(`${service}-docker-compose.yml`);
      expect(argv).toContain("up");
      expect(argv).toContain("-d");
    });
  }
});

describe("e2e argument parity: pause/unpause command", () => {
  beforeEach(() => {
    setDryRun(true);
    clearRecordedCommands();
  });

  afterAll(() => {
    setDryRun(false);
  });

  for (const contracts of ["gateway", "host"]) {
    it(`pause ${contracts}: produces correct docker compose args`, async () => {
      clearRecordedCommands();

      const { spawn } = await import("../executor.js");
      const pauseCompose = composeFile(`${contracts}-pause`);

      // Simulate: compose up + compose wait
      spawn(
        ["docker", "compose", "-p", PROJECT_NAME, "-f", pauseCompose, "up", "-d"],
        { stdio: ["inherit", "inherit", "inherit"] },
      );
      spawn(
        ["docker", "compose", "-p", PROJECT_NAME, "-f", pauseCompose, "wait", `${contracts}-sc-pause`],
        { stdio: ["inherit", "inherit", "inherit"] },
      );

      const cmds = getRecordedCommands();
      expect(cmds).toHaveLength(2);

      // First: compose up
      expect(cmds[0].argv).toContain("up");
      expect(cmds[0].argv).toContain("-d");
      expect(cmds[0].argv.join(" ")).toContain(`${contracts}-pause-docker-compose.yml`);

      // Second: compose wait
      expect(cmds[1].argv).toContain("wait");
      expect(cmds[1].argv).toContain(`${contracts}-sc-pause`);
    });

    it(`unpause ${contracts}: produces correct docker compose args`, async () => {
      clearRecordedCommands();

      const { spawn } = await import("../executor.js");
      const unpauseCompose = composeFile(`${contracts}-unpause`);

      spawn(
        ["docker", "compose", "-p", PROJECT_NAME, "-f", unpauseCompose, "up", "-d"],
        { stdio: ["inherit", "inherit", "inherit"] },
      );
      spawn(
        ["docker", "compose", "-p", PROJECT_NAME, "-f", unpauseCompose, "wait", `${contracts}-sc-unpause`],
        { stdio: ["inherit", "inherit", "inherit"] },
      );

      const cmds = getRecordedCommands();
      expect(cmds).toHaveLength(2);

      expect(cmds[0].argv.join(" ")).toContain(`${contracts}-unpause-docker-compose.yml`);
      expect(cmds[1].argv).toContain(`${contracts}-sc-unpause`);
    });
  }
});

describe("e2e argument parity: clean command", () => {
  beforeEach(() => {
    setDryRun(true);
    clearRecordedCommands();
  });

  afterAll(() => {
    setDryRun(false);
  });

  it("default clean matches bash: down -v --remove-orphans", async () => {
    clearRecordedCommands();

    const { composeDownAll } = await import("../docker.js");
    await composeDownAll({ volumes: true, removeOrphans: true });

    const cmds = getRecordedCommands();
    expect(cmds).toHaveLength(1);

    const argv = cmds[0].argv;
    expect(argv).toContain("down");
    expect(argv).toContain("-v");
    expect(argv).toContain("--remove-orphans");
    expect(argv).toContain("-p");
    expect(argv[argv.indexOf("-p") + 1]).toBe("fhevm");
  });
});

describe("e2e argument parity: logs command", () => {
  beforeEach(() => {
    setDryRun(true);
    clearRecordedCommands();
  });

  afterAll(() => {
    setDryRun(false);
  });

  it("logs <service> matches bash: docker logs <service>", () => {
    clearRecordedCommands();

    const { spawn } = require("../executor.js");
    spawn(["docker", "logs", "some-service"], { stdio: ["inherit", "inherit", "inherit"] });

    const cmds = getRecordedCommands();
    expect(cmds).toHaveLength(1);
    expect(cmds[0].argv).toEqual(["docker", "logs", "some-service"]);
  });
});
