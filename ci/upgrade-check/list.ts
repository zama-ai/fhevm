#!/usr/bin/env bun

import { execSync } from "child_process";
import { existsSync, mkdtempSync, rmSync } from "fs";
import { tmpdir } from "os";
import { join, resolve } from "path";

import { collectUpgradeVersionResults } from "./lib";
import { CONTRACT_HINTS, PACKAGE_CONSTRAINTS } from "./hints";

type PackageName = "host-contracts" | "gateway-contracts";

const PACKAGE_CONFIG: Record<PackageName, { extraDeps?: string }> = {
  "host-contracts": { extraDeps: "forge soldeer install" },
  "gateway-contracts": {},
};

function usage(): never {
  console.error("Usage: bun ci/upgrade-check/list.ts --from <tag/ref> [--to <tag/ref>] [--package host-contracts|gateway-contracts]");
  process.exit(1);
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
      if (value !== "host-contracts" && value !== "gateway-contracts") usage();
      packages.push(value);
    } else {
      usage();
    }
  }

  if (!fromRef) usage();

  return {
    fromRef,
    toRef,
    packages: packages.length > 0 ? packages : (["host-contracts", "gateway-contracts"] as PackageName[]),
  };
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
  const targetDir = join(targetRoot, pkg);
  const baselineDir = join(baselineRoot, pkg);
  const extraDeps = PACKAGE_CONFIG[pkg].extraDeps;

  run("npm ci", targetDir);
  run("npm ci", baselineDir);
  if (extraDeps) {
    run(extraDeps, targetDir);
    run(extraDeps, baselineDir);
  }
  run("make ensure-addresses", targetDir);
  run("make ensure-addresses", baselineDir);
  run(`bun ci/shared/merge-address-constants.ts "${join(baselineDir, "addresses")}" "${join(targetDir, "addresses")}"`, currentRepoRoot);
  run(`cp "${join(targetDir, "foundry.toml")}" "${join(baselineDir, "foundry.toml")}"`, currentRepoRoot);
}

function printPackageReport(pkg: PackageName, repoRoot: string, baselineRoot: string) {
  const results = collectUpgradeVersionResults(join(baselineRoot, pkg), join(repoRoot, pkg));
  const changed = results.filter((result) => result.baselineExists && result.bytecodeChanged);
  const unchanged = results.filter((result) => result.baselineExists && !result.bytecodeChanged);
  const errors = results.flatMap((result) => result.errors.map((error) => `${result.name}: ${error}`));

  console.log(`\n## ${pkg}`);

  if (errors.length > 0) {
    console.log("\nErrors:");
    for (const error of errors) {
      console.log(`- ${error}`);
    }
    process.exitCode = 1;
    return;
  }

  console.log("\nNeed upgrade:");
  for (const result of changed) {
    console.log(`- ${result.name}`);
    if (result.reinitializer) {
      console.log(`  reinitializer: ${result.reinitializer.signature}`);
      console.log(`  upgrade args: ${result.reinitializer.inputs.length > 0 ? "yes" : "no"}`);
      const defaults = CONTRACT_HINTS[pkg][result.name]?.defaults;
      if (defaults) {
        console.log("  task defaults:");
        for (const [name, value] of Object.entries(defaults)) {
          console.log(`  - ${name} = ${value}`);
        }
      } else if (result.reinitializer.inputs.length > 0) {
        console.log("  note: check arg values with a repo owner");
      }
    }
  }

  console.log("\nNo upgrade needed:");
  for (const result of unchanged) {
    console.log(`- ${result.name}`);
  }

  const changedNames = new Set(changed.map((result) => result.name));
  const activeConstraints = PACKAGE_CONSTRAINTS[pkg].filter((constraint) =>
    constraint.contracts.every((contract) => changedNames.has(contract)),
  );
  if (activeConstraints.length > 0) {
    console.log("\nAttention points:");
    for (const constraint of activeConstraints) {
      console.log(`- ${constraint.message}`);
    }
  }
}

const { fromRef, toRef, packages } = parseArgs();
const repoRoot = resolve(import.meta.dir, "../..");
const tempRoot = mkdtempSync(join(tmpdir(), "fhevm-upgrade-report-"));
const baselineRoot = join(tempRoot, "baseline");
const targetRoot = toRef ? join(tempRoot, "target") : repoRoot;

try {
  addWorktree(repoRoot, baselineRoot, fromRef);
  if (toRef) {
    addWorktree(repoRoot, targetRoot, toRef);
  }

  for (const pkg of packages) {
    preparePackage(repoRoot, targetRoot, baselineRoot, pkg);
    printPackageReport(pkg, targetRoot, baselineRoot);
  }
} catch (error) {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`\nUpgrade report failed: ${message}`);
  process.exitCode = 1;
} finally {
  if (toRef && existsSync(targetRoot)) {
    try {
      run(`git worktree remove --force "${targetRoot}"`, repoRoot);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`Failed to clean target worktree: ${message}`);
    }
  }
  if (existsSync(baselineRoot)) {
    try {
      run(`git worktree remove --force "${baselineRoot}"`, repoRoot);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`Failed to clean baseline worktree: ${message}`);
    }
  }
  rmSync(tempRoot, { recursive: true, force: true });
}
