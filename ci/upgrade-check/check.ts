#!/usr/bin/env bun
// Checks that upgradeable contracts have proper version bumps when bytecode changes.
// Usage: bun ci/upgrade-check/check.ts <baseline-pkg-dir> <pr-pkg-dir>
import { execFileSync } from "child_process";
import { basename } from "path";

import { collectUpgradeVersionResults } from "./lib";

const [baselineDir, prDir] = process.argv.slice(2);
if (!baselineDir || !prDir) {
  console.error("Usage: bun ci/upgrade-check/check.ts <baseline-pkg-dir> <pr-pkg-dir>");
  process.exit(1);
}

const results = collectUpgradeVersionResults(baselineDir, prDir);
let errors = 0;

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

    for (const error of result.errors) {
      console.error(`::error::${error}`);
      errors++;
    }
    if (result.errors.length > 0) {
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

console.log("All contracts passed upgrade version checks");
