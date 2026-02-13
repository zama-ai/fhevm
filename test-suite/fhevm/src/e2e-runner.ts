#!/usr/bin/env bun
/**
 * Migration test — remove when the original bash scripts are retired.
 *
 * Live e2e comparison runner.
 *
 * Extracts the original bash fhevm-cli from git (pinned SHA), runs both CLIs
 * with the same command against a live Docker stack, and compares exit codes
 * + output.
 *
 * Requires: Docker running, FHEVM stack deployed.
 *
 * Usage:
 *   bun run src/e2e-runner.ts test input-proof
 *   bun run src/e2e-runner.ts test --all
 *   bun run src/e2e-runner.ts upgrade coprocessor
 *   bun run src/e2e-runner.ts logs kms-core
 *   bun run src/e2e-runner.ts pause gateway
 *   bun run src/e2e-runner.ts unpause gateway
 *   bun run src/e2e-runner.ts clean
 */

import { execSync } from "child_process";
import { mkdtempSync, writeFileSync, readFileSync, chmodSync, rmSync } from "fs";
import { join } from "path";
import { tmpdir } from "os";
import chalk from "chalk";
import { FHEVM_ROOT } from "./paths.js";
import { SUITE_NAMES } from "./test-suites.js";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface RunResult {
  stdout: string;
  stderr: string;
  exitCode: number;
}

interface CompareResult {
  label: string;
  passed: boolean;
  exitCodeMatch: boolean;
  bashExit: number;
  tsExit: number;
  stdoutDiffLines: number;
  patternMismatches: string[];
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** Strip ANSI escape codes */
function stripAnsi(s: string): string {
  return s.replace(/\x1b\[[0-9;]*m/g, "");
}

/** Normalize output: strip ANSI, remove empty lines */
function normalize(s: string): string {
  return stripAnsi(s)
    .split("\n")
    .filter((l) => l.trim().length > 0)
    .join("\n");
}

/** Run a command and capture stdout, stderr, exit code */
function runCommand(args: string[], cwd?: string): RunResult {
  try {
    const result = Bun.spawnSync(args, {
      cwd,
      stdout: "pipe",
      stderr: "pipe",
      env: process.env,
    });
    return {
      stdout: result.stdout.toString(),
      stderr: result.stderr.toString(),
      exitCode: result.exitCode,
    };
  } catch (e: any) {
    return {
      stdout: e.stdout?.toString() ?? "",
      stderr: e.stderr?.toString() ?? "",
      exitCode: e.status ?? 1,
    };
  }
}

/** Compare two runs and return a structured result */
function compareRuns(
  label: string,
  bash: RunResult,
  ts: RunResult,
): CompareResult {
  const exitCodeMatch = bash.exitCode === ts.exitCode;

  // Normalize and diff stdout
  const bashNorm = normalize(bash.stdout);
  const tsNorm = normalize(ts.stdout);
  const bashLines = bashNorm.split("\n");
  const tsLines = tsNorm.split("\n");

  // Simple line-level diff count
  let diffLines = 0;
  const maxLen = Math.max(bashLines.length, tsLines.length);
  for (let i = 0; i < maxLen; i++) {
    if (bashLines[i] !== tsLines[i]) diffLines++;
  }

  // Sample key patterns from combined output (stdout + stderr)
  const patterns = ["passing", "failing", "Error", "AssertionError", "test "];
  const patternMismatches: string[] = [];
  for (const p of patterns) {
    const bashCombined = bash.stdout + bash.stderr;
    const tsCombined = ts.stdout + ts.stderr;
    const bashCount = (bashCombined.match(new RegExp(p, "g")) ?? []).length;
    const tsCount = (tsCombined.match(new RegExp(p, "g")) ?? []).length;
    if (bashCount !== tsCount) {
      patternMismatches.push(`'${p}': bash=${bashCount} ts=${tsCount}`);
    }
  }

  const passed = exitCodeMatch && diffLines <= 10; // Allow minor formatting diffs

  return { label, passed, exitCodeMatch, bashExit: bash.exitCode, tsExit: ts.exitCode, stdoutDiffLines: diffLines, patternMismatches };
}

/** Print a comparison result */
function printResult(r: CompareResult, tmpDir: string, bashResult: RunResult, tsResult: RunResult): void {
  console.log(`\n${chalk.bold(`=== ${r.label} ===`)}`);

  if (r.exitCodeMatch) {
    console.log(`${chalk.green("[PASS]")} Exit codes match: ${r.bashExit}`);
  } else {
    console.log(`${chalk.red("[FAIL]")} Exit code mismatch: bash=${r.bashExit} ts=${r.tsExit}`);
  }

  if (r.stdoutDiffLines === 0) {
    console.log(`${chalk.green("[PASS]")} Stdout matches exactly`);
  } else if (r.stdoutDiffLines <= 10) {
    console.log(`${chalk.yellow("[WARN]")} Minor stdout differences (${r.stdoutDiffLines} lines) — likely formatting`);
  } else {
    console.log(`${chalk.red("[FAIL]")} Stdout differs significantly (${r.stdoutDiffLines} lines)`);
    // Write diffs to temp dir for inspection
    const bashPath = join(tmpDir, `${r.label}-bash.txt`);
    const tsPath = join(tmpDir, `${r.label}-ts.txt`);
    writeFileSync(bashPath, normalize(bashResult.stdout));
    writeFileSync(tsPath, normalize(tsResult.stdout));
    console.log(`  Bash: ${bashPath}`);
    console.log(`  TS:   ${tsPath}`);
  }

  for (const m of r.patternMismatches) {
    console.log(`${chalk.yellow("[WARN]")} Pattern count mismatch: ${m}`);
  }

  if (r.passed) {
    console.log(`${chalk.green("[RESULT]")} ${chalk.bold(`${r.label}: PASSED`)}`);
  } else {
    console.log(`${chalk.red("[RESULT]")} ${chalk.bold(`${r.label}: FAILED`)}`);
  }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main() {
  const args = process.argv.slice(2);
  const command = args[0];

  if (!command) {
    console.log(`Usage: bun run src/e2e-runner.ts <command> [args...]

Commands:
  test <suite|--all>   Compare test execution
  upgrade <service>    Compare upgrade command
  logs <service>       Compare logs command
  pause <contracts>    Compare pause command
  unpause <contracts>  Compare unpause command
  clean                Compare clean command`);
    process.exit(1);
  }

  // Create temp dir
  const tmpDir = mkdtempSync(join(tmpdir(), "fhevm-e2e-"));

  // Extract original bash CLI from git
  console.log("Extracting original bash fhevm-cli from git (pinned SHA)...");
  const bashCliPath = join(tmpDir, "fhevm-cli-bash");
  const bashSource = execSync("git show a4a9aa47d204cd2c6b6320883b7f3c4121957cb6:test-suite/fhevm/fhevm-cli", {
    cwd: FHEVM_ROOT,
    encoding: "utf-8",
  });
  writeFileSync(bashCliPath, bashSource);
  chmodSync(bashCliPath, 0o755);

  const tsCliPath = join(FHEVM_ROOT, "src", "index.ts");
  const results: CompareResult[] = [];
  const bashResults: RunResult[] = [];
  const tsResults: RunResult[] = [];

  /** Run one comparison */
  function runComparison(label: string, cliArgs: string[]): void {
    console.log(`\n${chalk.yellow(`[BASH]`)} Running: bash ${bashCliPath} ${cliArgs.join(" ")}`);
    const bashRun = runCommand(["bash", bashCliPath, ...cliArgs]);

    console.log(`${chalk.yellow(`[TS]`)} Running: bun run ${tsCliPath} ${cliArgs.join(" ")}`);
    const tsRun = runCommand(["bun", "run", tsCliPath, ...cliArgs]);

    const result = compareRuns(label, bashRun, tsRun);
    printResult(result, tmpDir, bashRun, tsRun);

    results.push(result);
    bashResults.push(bashRun);
    tsResults.push(tsRun);
  }

  // Dispatch command
  const subArgs = args.slice(1);

  switch (command) {
    case "test": {
      const suite = subArgs[0];
      const extraArgs = subArgs.slice(1);

      if (suite === "--all") {
        console.log(chalk.bold("Running e2e comparison for all test suites..."));
        for (const s of SUITE_NAMES) {
          runComparison(`test-${s}`, ["test", s, ...extraArgs]);
        }
      } else if (suite) {
        runComparison(`test-${suite}`, ["test", suite, ...extraArgs]);
      } else {
        console.error("Usage: bun run src/e2e-runner.ts test <suite|--all> [extra-args...]");
        process.exit(1);
      }
      break;
    }
    case "upgrade": {
      const service = subArgs[0];
      if (!service) { console.error("Service required"); process.exit(1); }
      runComparison(`upgrade-${service}`, ["upgrade", service]);
      break;
    }
    case "logs": {
      const service = subArgs[0];
      if (!service) { console.error("Service required"); process.exit(1); }
      runComparison(`logs-${service}`, ["logs", service]);
      break;
    }
    case "pause": {
      const contracts = subArgs[0];
      if (!contracts) { console.error("Contracts required (gateway|host)"); process.exit(1); }
      runComparison(`pause-${contracts}`, ["pause", contracts]);
      break;
    }
    case "unpause": {
      const contracts = subArgs[0];
      if (!contracts) { console.error("Contracts required (gateway|host)"); process.exit(1); }
      runComparison(`unpause-${contracts}`, ["unpause", contracts]);
      break;
    }
    case "clean":
      runComparison("clean", ["clean"]);
      break;
    default:
      console.error(chalk.red(`Unknown command: ${command}`));
      process.exit(1);
  }

  // Summary
  const passed = results.filter((r) => r.passed).length;
  const failed = results.filter((r) => !r.passed).length;

  console.log(`\n${chalk.bold("=== E2E Test Summary ===")}`);
  console.log(`  Total:  ${results.length}`);
  console.log(`  Passed: ${chalk.green(String(passed))}`);
  console.log(`  Failed: ${chalk.red(String(failed))}`);

  // Cleanup on success, preserve on failure
  if (failed === 0) {
    rmSync(tmpDir, { recursive: true, force: true });
  } else {
    console.log(`\n  Temp dir preserved: ${tmpDir}`);
    process.exit(1);
  }
}

main();
