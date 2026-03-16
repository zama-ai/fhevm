#!/usr/bin/env bun
// Checks that upgradeable contracts have proper version bumps when bytecode changes.
// Usage: bun ci/check-upgrade-hygiene.ts <baseline-pkg-dir> <pr-pkg-dir>

import { readFileSync, existsSync } from "fs";
import { execSync } from "child_process";
import { join } from "path";

const [baselineDir, prDir] = process.argv.slice(2);
if (!baselineDir || !prDir) {
  console.error("Usage: bun ci/check-upgrade-hygiene.ts <baseline-pkg-dir> <pr-pkg-dir>");
  process.exit(1);
}

const manifestPath = join(prDir, "upgrade-manifest.json");
if (!existsSync(manifestPath)) {
  console.error(`::error::upgrade-manifest.json not found in ${prDir}`);
  process.exit(1);
}

const VERSION_RE = /(?<name>REINITIALIZER_VERSION|MAJOR_VERSION|MINOR_VERSION|PATCH_VERSION)\s*=\s*(?<value>\d+)/g;

function extractVersions(filePath: string) {
  const source = readFileSync(filePath, "utf-8");
  const versions: Record<string, number> = {};
  for (const { groups } of source.matchAll(VERSION_RE)) {
    versions[groups!.name] = Number(groups!.value);
  }
  return { versions, source };
}

function forgeInspect(contract: string, root: string): string | null {
  try {
    const raw = execSync(`forge inspect "contracts/${contract}.sol:${contract}" --root "${root}" deployedBytecode`, {
      encoding: "utf-8",
      stdio: ["pipe", "pipe", "pipe"],
      env: { ...process.env, NO_COLOR: "1" },
    });
    // Extract hex bytecode — forge may prepend ANSI codes or compilation progress to stdout
    const match = raw.match(/0x[0-9a-fA-F]+/);
    return match ? match[0] : null;
  } catch (e: any) {
    if (e.stderr) console.error(String(e.stderr));
    return null;
  }
}

const contracts: string[] = JSON.parse(readFileSync(manifestPath, "utf-8"));
let errors = 0;

for (const name of contracts) {
  console.log(`::group::Checking ${name}`);
  try {
    const baseSol = join(baselineDir, "contracts", `${name}.sol`);
    const prSol = join(prDir, "contracts", `${name}.sol`);

    if (!existsSync(baseSol)) {
      console.log(`Skipping ${name} (new contract, not in baseline)`);
      continue;
    }

    if (!existsSync(prSol)) {
      console.error(`::error::${name} listed in upgrade-manifest.json but missing in PR`);
      errors++;
      continue;
    }

    const { versions: baseV } = extractVersions(baseSol);
    const { versions: prV, source: prSrc } = extractVersions(prSol);

    let parseFailed = false;
    for (const key of ["REINITIALIZER_VERSION", "MAJOR_VERSION", "MINOR_VERSION", "PATCH_VERSION"]) {
      if (baseV[key] == null || prV[key] == null) {
        console.error(`::error::Failed to parse ${key} for ${name}`);
        errors++;
        parseFailed = true;
      }
    }
    if (parseFailed) continue;

    const prBytecode = forgeInspect(name, prDir);
    if (prBytecode == null) {
      console.error(`::error::Failed to compile ${name} on PR`);
      errors++;
      continue;
    }

    // Baseline may fail to compile if unrelated contracts were restructured (e.g. deleted
    // imports cause forge to fail on the whole project). When that happens, fall back to
    // comparing source files: if this contract's source is identical, bytecode is unchanged.
    const baseBytecode = forgeInspect(name, baselineDir);
    let bytecodeChanged: boolean;
    if (baseBytecode != null) {
      bytecodeChanged = baseBytecode !== prBytecode;
    } else {
      const baseSrc = readFileSync(baseSol, "utf-8");
      const prSrcRaw = readFileSync(prSol, "utf-8");
      if (baseSrc === prSrcRaw) {
        console.log(`${name}: baseline compilation failed but source is identical, treating as unchanged`);
        bytecodeChanged = false;
      } else {
        console.log(`${name}: baseline compilation failed and source differs, treating as changed`);
        bytecodeChanged = true;
      }
    }
    const reinitChanged = baseV.REINITIALIZER_VERSION !== prV.REINITIALIZER_VERSION;
    const versionChanged =
      baseV.MAJOR_VERSION !== prV.MAJOR_VERSION ||
      baseV.MINOR_VERSION !== prV.MINOR_VERSION ||
      baseV.PATCH_VERSION !== prV.PATCH_VERSION;

    if (!bytecodeChanged) {
      console.log(`${name}: bytecode unchanged`);
      if (reinitChanged) {
        console.error(
          `::error::${name} REINITIALIZER_VERSION bumped (${baseV.REINITIALIZER_VERSION} -> ${prV.REINITIALIZER_VERSION}) but bytecode is unchanged`,
        );
        errors++;
      }
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
      const uncommented = prSrc.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/.*$/gm, "");
      if (!new RegExp(`function\\s+${expectedFn}\\s*\\(`).test(uncommented)) {
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
  } finally {
    console.log("::endgroup::");
  }
}

if (errors > 0) {
  console.error(`::error::Upgrade hygiene check failed with ${errors} error(s)`);
  process.exit(1);
}

console.log("All contracts passed upgrade hygiene checks");
