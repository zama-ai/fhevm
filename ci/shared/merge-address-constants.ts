#!/usr/bin/env bun
//
// Merges Solidity address-constant files from a baseline and a PR so that both
// sides can compile against the same unified set of constants.
//
// WHY THIS IS NEEDED
// ──────────────────
// We compare compiled bytecode between a baseline tag (last deployed release)
// and the PR to detect contract changes.  Both sides must compile with
// *identical* address constants because those constants are embedded in
// bytecode — any difference would cause a false "bytecode changed" signal.
//
// The naive approach (generate addresses on the PR side, copy to baseline)
// breaks when contracts are added or removed between versions.  For example,
// if the PR deletes MultichainACL, the generated addresses file no longer
// contains `multichainACLAddress`.  But the baseline still has source files
// that import it, so forge compilation fails for the *entire* project —
// including unrelated contracts like GatewayConfig that haven't changed.
//
// The fix: generate addresses on BOTH sides, then merge.  PR values win for
// shared constants (so both sides embed the same values).  Constants that only
// exist in the baseline (removed contracts) are preserved so the baseline
// compiles.  Constants that only exist in the PR (new contracts) are preserved
// so the PR compiles.  The merged file is copied to both sides.
//
// USAGE
//   bun ci/shared/merge-address-constants.ts <baseline-addresses-dir> <pr-addresses-dir>
//
// For each .sol file present in either directory, writes a merged version to
// BOTH directories.  Exits 0 on success, 1 on error.

import { readFileSync, writeFileSync, existsSync, readdirSync } from "fs";
import { join } from "path";

const ADDRESS_RE = /^address\s+constant\s+(\w+)\s*=\s*(0x[0-9a-fA-F]+)\s*;/;

interface AddressConstant {
  name: string;
  value: string;
  line: string; // original line for faithful reproduction
}

/**
 * Parse a Solidity address-constants file into its header (SPDX + pragma) and
 * an ordered list of address constants.
 */
function parseAddressFile(content: string): { header: string; constants: AddressConstant[] } {
  const lines = content.split("\n");
  const constants: AddressConstant[] = [];
  const headerLines: string[] = [];
  let inHeader = true;

  for (const line of lines) {
    const match = line.match(ADDRESS_RE);
    if (match) {
      inHeader = false;
      constants.push({ name: match[1], value: match[2], line });
    } else if (inHeader) {
      headerLines.push(line);
    }
    // Skip blank lines between constants — we regenerate spacing
  }

  return { header: headerLines.join("\n"), constants };
}

/**
 * Merge two parsed address files.  PR constants take precedence for shared
 * names.  Baseline-only constants are appended at the end.
 */
function mergeConstants(
  baseline: AddressConstant[],
  pr: AddressConstant[],
): AddressConstant[] {
  const seen = new Set<string>();
  const merged: AddressConstant[] = [];

  // PR constants first, in PR order — these values win for shared names
  for (const c of pr) {
    merged.push(c);
    seen.add(c.name);
  }

  // Baseline-only constants (removed in PR) — appended so baseline compiles
  for (const c of baseline) {
    if (!seen.has(c.name)) {
      merged.push(c);
    }
  }

  return merged;
}

/**
 * Render merged constants back to a Solidity file.
 */
function renderAddressFile(header: string, constants: AddressConstant[]): string {
  const lines = constants.map((c) => c.line);
  return header.trimEnd() + "\n\n" + lines.join("\n") + "\n";
}

// --- Main ---

const [baselineDir, prDir] = process.argv.slice(2);
if (!baselineDir || !prDir) {
  console.error("Usage: bun ci/shared/merge-address-constants.ts <baseline-addresses-dir> <pr-addresses-dir>");
  process.exit(1);
}

// Collect all .sol filenames from both directories
const baselineFiles = existsSync(baselineDir)
  ? readdirSync(baselineDir).filter((f) => f.endsWith(".sol"))
  : [];
const prFiles = existsSync(prDir)
  ? readdirSync(prDir).filter((f) => f.endsWith(".sol"))
  : [];
const allFiles = [...new Set([...baselineFiles, ...prFiles])];

for (const file of allFiles) {
  const baselinePath = join(baselineDir, file);
  const prPath = join(prDir, file);

  const hasBaseline = existsSync(baselinePath);
  const hasPR = existsSync(prPath);

  if (hasBaseline && hasPR) {
    // Merge: PR values win for shared constants, baseline-only constants preserved
    const baselineParsed = parseAddressFile(readFileSync(baselinePath, "utf-8"));
    const prParsed = parseAddressFile(readFileSync(prPath, "utf-8"));
    const merged = mergeConstants(baselineParsed.constants, prParsed.constants);
    const output = renderAddressFile(prParsed.header, merged);

    console.log(`${file}: merged (${prParsed.constants.length} PR + ${baselineParsed.constants.length} baseline → ${merged.length} total)`);
    writeFileSync(baselinePath, output);
    writeFileSync(prPath, output);
  } else if (hasBaseline) {
    // File only in baseline (removed in PR) — copy to PR so baseline imports resolve
    console.log(`${file}: baseline-only, copying to PR`);
    writeFileSync(prPath, readFileSync(baselinePath, "utf-8"));
  } else {
    // File only in PR (new) — copy to baseline so PR imports resolve
    console.log(`${file}: PR-only, copying to baseline`);
    writeFileSync(baselinePath, readFileSync(prPath, "utf-8"));
  }
}

console.log("Address constants merged successfully");
