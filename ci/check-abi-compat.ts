#!/usr/bin/env bun
import { PACKAGE_CONFIG, type PackageName } from "./abi-compat-config";
import { collectAbiCompatResults } from "./abi-compat-lib";

const [baselineDir, targetDir, pkg] = process.argv.slice(2) as [
  string | undefined,
  string | undefined,
  PackageName | undefined,
];

if (!baselineDir || !targetDir || !pkg || !(pkg in PACKAGE_CONFIG)) {
  console.error(
    "Usage: bun ci/check-abi-compat.ts <baseline-pkg-dir> <target-pkg-dir> <host-contracts|gateway-contracts>",
  );
  process.exit(1);
}

const results = collectAbiCompatResults(baselineDir, targetDir, pkg);
let errors = 0;

for (const result of results) {
  console.log(`::group::Checking ${result.name}`);
  try {
    if (!result.baselineExists) {
      console.log(`Skipping ${result.name} (new contract, not in baseline)`);
      continue;
    }

    console.log(
      `${result.name}: ${result.baselineStableCount} stable baseline ABI entries, ${result.targetStableCount} stable target ABI entries`,
    );

    for (const error of result.errors) {
      console.error(`::error::${error}`);
      errors++;
    }

    for (const signature of result.missing) {
      console.error(`::error::${result.name} missing stable ABI signature: ${signature}`);
      errors++;
    }

    if (result.added.length > 0) {
      console.log(`${result.name}: added stable ABI entries`);
      for (const signature of result.added) {
        console.log(`  + ${signature}`);
      }
    }

    if (result.errors.length === 0 && result.missing.length === 0 && result.added.length === 0) {
      console.log(`${result.name}: no stable ABI changes`);
    }
  } finally {
    console.log("::endgroup::");
  }
}

if (errors > 0) {
  console.error(`::error::ABI compatibility check failed with ${errors} error(s)`);
  process.exit(1);
}

console.log("All contracts passed ABI compatibility checks");
