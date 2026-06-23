#!/usr/bin/env bun
// Checks that upgradeable contracts have proper version bumps when bytecode changes.
// Usage: bun ci/upgrade-check/check.ts <baseline-pkg-dir> <pr-pkg-dir> [--changed-contracts <comma-separated names>]
//
// With --changed-contracts, only the listed contracts are enforced; violations
// on other contracts are warnings.  An empty list downgrades everything.
import { execFileSync } from "child_process";
import { basename } from "path";

import { collectUpgradeVersionResults } from "./lib";

const positional: string[] = [];
let changedContracts: Set<string> | undefined;
const argv = process.argv.slice(2);
for (let idx = 0; idx < argv.length; idx++) {
  if (argv[idx] === "--changed-contracts") {
    const value = argv[++idx];
    if (value === undefined) {
      console.error("--changed-contracts requires a value (may be an empty string)");
      process.exit(1);
    }
    changedContracts = new Set(value.split(",").filter(Boolean));
  } else {
    positional.push(argv[idx]);
  }
}

const [baselineDir, prDir] = positional;
if (!baselineDir || !prDir) {
  console.error(
    "Usage: bun ci/upgrade-check/check.ts <baseline-pkg-dir> <pr-pkg-dir> [--changed-contracts <comma-separated names>]",
  );
  process.exit(1);
}

const results = collectUpgradeVersionResults(baselineDir, prDir);
let errors = 0;
let warnings = 0;

function printCommits(title: string, baseRef: string, targetRef: string, paths: string[]) {
  const repo = process.env.GITHUB_REPOSITORY;
  const format = repo ? `- %h %s (https://github.com/${repo}/commit/%H)` : "- %h %s";
  let output = "";

  try {
    output = execFileSync("git", ["log", `--format=${format}`, `${baseRef}..${targetRef}`, "--", ...paths], {
      encoding: "utf-8",
      env: { ...process.env, NO_COLOR: "1" },
    }).trim();
  } catch (error) {
    const stderr = error instanceof Error && "stderr" in error ? String(error.stderr).trim() : "";
    output = stderr || "git log failed";
  }

  console.log(title);
  console.log(output || "- none");
}

function printDiagnostics(contract: string) {
  const baseRef = process.env.UPGRADE_CHECK_BASE_REF;
  if (!baseRef) return;

  const targetRef = process.env.UPGRADE_CHECK_TARGET_REF ?? process.env.GITHUB_SHA ?? "HEAD";
  const pkg = basename(prDir);
  console.log(`Diagnostics for ${contract}:`);
  console.log(`Baseline ref: ${baseRef}`);
  console.log(`Target ref: ${targetRef}`);
  printCommits("Non-exhaustive candidate commits touching the direct contract source:", baseRef, targetRef, [
    `${pkg}/contracts/${contract}.sol`,
  ]);
  printCommits("Non-exhaustive candidate commits touching package build/config files:", baseRef, targetRef, [
    `${pkg}/foundry.toml`,
    `${pkg}/hardhat.config.ts`,
    `${pkg}/package.json`,
    `${pkg}/package-lock.json`,
    `${pkg}/remappings.txt`,
  ]);
}

for (const result of results) {
  console.log(`::group::Checking ${result.name}`);
  try {
    if (!result.baselineExists) {
      console.log(`Skipping ${result.name} (new contract, not in baseline)`);
      continue;
    }

    if (result.bytecodeChanged) {
      console.log(`${result.name}: bytecode CHANGED`);
    } else {
      console.log(`${result.name}: bytecode unchanged`);
    }

    const enforced = changedContracts === undefined || changedContracts.has(result.name);
    for (const error of result.errors) {
      if (enforced) {
        console.error(`::error::${error}`);
        errors++;
      } else {
        console.log(
          `::warning::${error} — ${result.name} was not changed by this PR, so this is not blocking. Fix it with a version-bump PR before the next release; this check runs strictly when the release is published.`,
        );
        warnings++;
      }
    }
    if (result.errors.length > 0 && enforced) {
      printDiagnostics(result.name);
    }
  } finally {
    console.log("::endgroup::");
  }
}

if (errors > 0) {
  console.error(`::error::Upgrade version check failed with ${errors} error(s)`);
  process.exit(1);
}

if (warnings > 0) {
  console.log(`${warnings} violation(s) on contracts not changed by this PR were reported as warnings.`);
}
console.log("All contracts passed upgrade version checks");
