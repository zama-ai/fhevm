#!/usr/bin/env bun
// Checks that upgradeable contracts have proper version bumps when bytecode changes.
// Usage: bun ci/check-upgrade-hygiene.ts <main-pkg-dir> <pr-pkg-dir>

import { readFileSync, existsSync } from "fs";
import { execSync } from "child_process";
import { join } from "path";

const [mainDir, prDir] = process.argv.slice(2);
if (!mainDir || !prDir) {
  console.error("Usage: bun ci/check-upgrade-hygiene.ts <main-pkg-dir> <pr-pkg-dir>");
  process.exit(1);
}

const manifestPath = join(prDir, "upgrade-manifest.json");
if (!existsSync(manifestPath)) {
  console.error(`::error::upgrade-manifest.json not found in ${prDir}`);
  process.exit(1);
}

const VERSION_RE = /(?<name>REINITIALIZER_VERSION|MAJOR_VERSION|MINOR_VERSION|PATCH_VERSION)\s*=\s*(?<value>\d+)/g;

function extractVersions(filePath: string) {
  const src = readFileSync(filePath, "utf-8");
  const versions: Record<string, number> = {};
  for (const { groups } of src.matchAll(VERSION_RE)) {
    versions[groups!.name] = Number(groups!.value);
  }
  return versions;
}

function forgeInspect(contract: string, root: string): string | null {
  try {
    return execSync(`forge inspect "contracts/${contract}.sol:${contract}" --root "${root}" deployedBytecode`, {
      encoding: "utf-8",
      stdio: ["pipe", "pipe", "ignore"],
    }).trim();
  } catch {
    return null;
  }
}

const contracts: string[] = JSON.parse(readFileSync(manifestPath, "utf-8"));
let errors = 0;

for (const name of contracts) {
  console.log(`::group::Checking ${name}`);

  const mainSol = join(mainDir, "contracts", `${name}.sol`);
  const prSol = join(prDir, "contracts", `${name}.sol`);

  if (!existsSync(mainSol)) {
    console.log(`Skipping ${name} (new contract, not on main)`);
    console.log("::endgroup::");
    continue;
  }

  if (!existsSync(prSol)) {
    console.error(`::error::${name} listed in upgrade-manifest.json but missing in PR`);
    errors++;
    console.log("::endgroup::");
    continue;
  }

  const mainV = extractVersions(mainSol);
  const prV = extractVersions(prSol);

  for (const key of ["REINITIALIZER_VERSION", "MAJOR_VERSION", "MINOR_VERSION", "PATCH_VERSION"]) {
    if (mainV[key] == null || prV[key] == null) {
      console.error(`::error::Failed to parse ${key} for ${name}`);
      errors++;
    }
  }
  if (errors > 0) {
    console.log("::endgroup::");
    continue;
  }

  const mainBytecode = forgeInspect(name, mainDir);
  if (mainBytecode == null) {
    console.error(`::error::Failed to compile ${name} on main`);
    errors++;
    console.log("::endgroup::");
    continue;
  }

  const prBytecode = forgeInspect(name, prDir);
  if (prBytecode == null) {
    console.error(`::error::Failed to compile ${name} on PR`);
    errors++;
    console.log("::endgroup::");
    continue;
  }

  const bytecodeChanged = mainBytecode !== prBytecode;
  const reinitChanged = mainV.REINITIALIZER_VERSION !== prV.REINITIALIZER_VERSION;
  const versionChanged =
    mainV.MAJOR_VERSION !== prV.MAJOR_VERSION ||
    mainV.MINOR_VERSION !== prV.MINOR_VERSION ||
    mainV.PATCH_VERSION !== prV.PATCH_VERSION;

  if (!bytecodeChanged) {
    console.log(`${name}: bytecode unchanged`);
    if (reinitChanged) {
      console.error(
        `::error::${name} REINITIALIZER_VERSION bumped (${mainV.REINITIALIZER_VERSION} -> ${prV.REINITIALIZER_VERSION}) but bytecode is unchanged`,
      );
      errors++;
    }
    console.log("::endgroup::");
    continue;
  }

  console.log(`${name}: bytecode CHANGED`);

  if (!reinitChanged) {
    console.error(
      `::error::${name} bytecode changed but REINITIALIZER_VERSION was not bumped (still ${prV.REINITIALIZER_VERSION})`,
    );
    errors++;
  } else {
    // Convention: reinitializeV{N-1} for REINITIALIZER_VERSION=N
    const expectedFn = `reinitializeV${prV.REINITIALIZER_VERSION - 1}`;
    const prSrc = readFileSync(prSol, "utf-8");
    if (!new RegExp(`function\\s+${expectedFn}\\s*\\(`).test(prSrc)) {
      console.error(
        `::error::${name} has REINITIALIZER_VERSION=${prV.REINITIALIZER_VERSION} but no ${expectedFn}() function found`,
      );
      errors++;
    }
  }

  if (!versionChanged) {
    console.error(
      `::error::${name} bytecode changed but semantic version was not bumped (still v${prV.MAJOR_VERSION}.${prV.MINOR_VERSION}.${prV.PATCH_VERSION})`,
    );
    errors++;
  }

  console.log("::endgroup::");
}

if (errors > 0) {
  console.error(`::error::Upgrade hygiene check failed with ${errors} error(s)`);
  process.exit(1);
}

console.log("All contracts passed upgrade hygiene checks");
