#!/usr/bin/env bun
// Checks that upgradeable contracts have proper version bumps when bytecode changes.
// Usage: bun ci/upgrade-check/check.ts <baseline-pkg-dir> <pr-pkg-dir>

import { collectUpgradeVersionResults } from "./lib";

const [baselineDir, prDir] = process.argv.slice(2);
if (!baselineDir || !prDir) {
  console.error("Usage: bun ci/upgrade-check/check.ts <baseline-pkg-dir> <pr-pkg-dir>");
  process.exit(1);
}

const results = collectUpgradeVersionResults(baselineDir, prDir);
let errors = 0;

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
  } finally {
    console.log("::endgroup::");
  }
}

if (errors > 0) {
  console.error(`::error::Upgrade version check failed with ${errors} error(s)`);
  process.exit(1);
}

console.log("All contracts passed upgrade version checks");
