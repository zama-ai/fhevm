#!/usr/bin/env bun
import { execSync } from "child_process";
import { existsSync, mkdtempSync, rmSync } from "fs";
import { tmpdir } from "os";
import { join, resolve } from "path";

import { PACKAGE_CONFIG, type PackageName } from "./abi-compat-config";
import { collectAbiCompatResults } from "./abi-compat-lib";

function usage(): never {
  console.error(
    "Usage: bun ci/list-abi-compat.ts --from <tag/ref> [--to <tag/ref>] [--package host-contracts|gateway-contracts]",
  );
  process.exit(1);
}

function logStep(message: string) {
  console.log(`\n==> ${message}`);
}

function formatExecError(error: unknown) {
  if (!(error instanceof Error)) {
    return String(error);
  }

  const execError = error as Error & { stdout?: string | Buffer; stderr?: string | Buffer };
  const stdout = execError.stdout ? String(execError.stdout).trim() : "";
  const stderr = execError.stderr ? String(execError.stderr).trim() : "";
  const details = [stderr, stdout].filter(Boolean).join("\n");
  if (!details) {
    return execError.message;
  }

  const maxLen = 2000;
  return details.length > maxLen ? `${details.slice(-maxLen)}` : details;
}

function parseArgs() {
  const args = process.argv.slice(2);
  let fromRef: string | undefined;
  let toRef: string | undefined;
  const packages: PackageName[] = [];

  for (let idx = 0; idx < args.length; idx++) {
    const arg = args[idx];
    if (arg === "--from") {
      fromRef = args[++idx];
    } else if (arg === "--to") {
      toRef = args[++idx];
    } else if (arg === "--package") {
      const value = args[++idx] as PackageName;
      if (!(value in PACKAGE_CONFIG)) {
        usage();
      }
      packages.push(value);
    } else {
      usage();
    }
  }

  if (!fromRef) {
    usage();
  }

  return {
    fromRef,
    toRef,
    packages: packages.length > 0 ? packages : (Object.keys(PACKAGE_CONFIG) as PackageName[]),
  };
}

function run(cmd: string, cwd: string) {
  try {
    execSync(cmd, {
      cwd,
      stdio: ["ignore", "pipe", "pipe"],
      encoding: "utf-8",
      env: { ...process.env, NO_COLOR: "1" },
    });
  } catch (error) {
    throw new Error(`Command failed in ${cwd}: ${cmd}\n${formatExecError(error)}`);
  }
}

function addWorktree(repoRoot: string, path: string, ref: string) {
  run(`git worktree add --detach "${path}" "${ref}"`, repoRoot);
}

function preparePackage(currentRepoRoot: string, targetRoot: string, baselineRoot: string, pkg: PackageName) {
  const baselineDir = join(baselineRoot, pkg);
  const targetDir = join(targetRoot, pkg);
  const extraDeps = PACKAGE_CONFIG[pkg].extraDeps;

  logStep(`Installing dependencies for ${pkg}`);
  run("npm ci", baselineDir);
  run("npm ci", targetDir);
  if (extraDeps) {
    logStep(`Installing extra build dependencies for ${pkg}`);
    run(extraDeps, baselineDir);
    run(extraDeps, targetDir);
  }

  logStep(`Generating local compile-time addresses for ${pkg} (no real network deployment)`);
  run("make ensure-addresses", baselineDir);
  run("make ensure-addresses", targetDir);
  logStep(`Normalizing address constants for ${pkg}`);
  run(
    `bun ci/merge-address-constants.ts "${join(baselineDir, "addresses")}" "${join(targetDir, "addresses")}"`,
    currentRepoRoot,
  );
  run(`cp "${join(targetDir, "foundry.toml")}" "${join(baselineDir, "foundry.toml")}"`, currentRepoRoot);
}

function printPackageReport(baselineRoot: string, targetRoot: string, pkg: PackageName) {
  const results = collectAbiCompatResults(join(baselineRoot, pkg), join(targetRoot, pkg), pkg);

  logStep(`Comparing stable ABI surface for ${pkg}`);
  let packageFailures = 0;

  for (const result of results) {
    if (!result.baselineExists) {
      continue;
    }

    console.log(`\n### ${result.name}`);
    console.log(`baseline stable entries: ${result.baselineStableCount}`);
    console.log(`target stable entries: ${result.targetStableCount}`);

    if (result.errors.length > 0) {
      for (const error of result.errors) {
        console.log(`error: ${error}`);
        packageFailures++;
      }
      continue;
    }

    if (result.missing.length === 0) {
      console.log("missing stable signatures: none");
    } else {
      console.log("missing stable signatures:");
      for (const signature of result.missing) {
        console.log(`- ${signature}`);
      }
      packageFailures += result.missing.length;
    }

    if (result.added.length === 0) {
      console.log("added stable signatures: none");
    } else {
      console.log("added stable signatures:");
      for (const signature of result.added) {
        console.log(`+ ${signature}`);
      }
    }
  }

  if (packageFailures > 0) {
    return packageFailures;
  }

  return 0;
}

const { fromRef, toRef, packages } = parseArgs();
const repoRoot = resolve(import.meta.dir, "..");
const tempRoot = mkdtempSync(join(tmpdir(), "fhevm-abi-compat-"));
const baselineRoot = join(tempRoot, "baseline");
const targetRoot = toRef ? join(tempRoot, "target") : repoRoot;

let totalFailures = 0;

try {
  logStep(`Using temporary workspace at ${tempRoot}`);
  logStep(`Preparing temporary baseline checkout for ${fromRef}`);
  addWorktree(repoRoot, baselineRoot, fromRef);
  if (toRef) {
    logStep(`Preparing temporary target checkout for ${toRef}`);
    addWorktree(repoRoot, targetRoot, toRef);
  } else {
    logStep("Using current checkout as target");
  }

  for (const pkg of packages) {
    preparePackage(repoRoot, targetRoot, baselineRoot, pkg);
    totalFailures += printPackageReport(baselineRoot, targetRoot, pkg);
  }
} catch (error) {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`\nABI compatibility run failed: ${message}`);
} finally {
  if (toRef && existsSync(targetRoot)) {
    logStep(`Cleaning temporary target checkout ${targetRoot}`);
    try {
      run(`git worktree remove --force "${targetRoot}"`, repoRoot);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`Failed to clean target worktree: ${message}`);
    }
  }
  if (existsSync(baselineRoot)) {
    logStep(`Cleaning temporary baseline checkout ${baselineRoot}`);
    try {
      run(`git worktree remove --force "${baselineRoot}"`, repoRoot);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`Failed to clean baseline worktree: ${message}`);
    }
  }
  rmSync(tempRoot, { recursive: true, force: true });
  logStep(`Removed temporary workspace ${tempRoot}`);
}

if (totalFailures > 0) {
  console.error(`\nABI compatibility check found ${totalFailures} issue(s)`);
  process.exit(1);
}

console.log("\nABI compatibility check passed");
