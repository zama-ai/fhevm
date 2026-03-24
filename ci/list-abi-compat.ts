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
  execSync(cmd, {
    cwd,
    stdio: "inherit",
    env: { ...process.env, NO_COLOR: "1" },
  });
}

function addWorktree(repoRoot: string, path: string, ref: string) {
  run(`git worktree add --detach "${path}" "${ref}"`, repoRoot);
}

function preparePackage(currentRepoRoot: string, targetRoot: string, baselineRoot: string, pkg: PackageName) {
  const baselineDir = join(baselineRoot, pkg);
  const targetDir = join(targetRoot, pkg);
  const extraDeps = PACKAGE_CONFIG[pkg].extraDeps;

  run("npm ci", baselineDir);
  run("npm ci", targetDir);
  if (extraDeps) {
    run(extraDeps, baselineDir);
    run(extraDeps, targetDir);
  }

  run("make ensure-addresses", baselineDir);
  run("make ensure-addresses", targetDir);
  run(
    `bun ci/merge-address-constants.ts "${join(baselineDir, "addresses")}" "${join(targetDir, "addresses")}"`,
    currentRepoRoot,
  );
  run(`cp "${join(targetDir, "foundry.toml")}" "${join(baselineDir, "foundry.toml")}"`, currentRepoRoot);
}

function printPackageReport(baselineRoot: string, targetRoot: string, pkg: PackageName) {
  const results = collectAbiCompatResults(join(baselineRoot, pkg), join(targetRoot, pkg), pkg);

  console.log(`\n## ${pkg}`);
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

try {
  let totalFailures = 0;
  addWorktree(repoRoot, baselineRoot, fromRef);
  if (toRef) {
    addWorktree(repoRoot, targetRoot, toRef);
  }

  for (const pkg of packages) {
    preparePackage(repoRoot, targetRoot, baselineRoot, pkg);
    totalFailures += printPackageReport(baselineRoot, targetRoot, pkg);
  }

  if (totalFailures > 0) {
    throw new Error(`ABI compatibility check found ${totalFailures} issue(s)`);
  }
} finally {
  if (toRef && existsSync(targetRoot)) {
    run(`git worktree remove --force "${targetRoot}"`, repoRoot);
  }
  if (existsSync(baselineRoot)) {
    run(`git worktree remove --force "${baselineRoot}"`, repoRoot);
  }
  rmSync(tempRoot, { recursive: true, force: true });
}
