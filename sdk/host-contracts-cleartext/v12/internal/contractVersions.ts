// The expected `getVersion()` string of every contract that reports one, read from the source that
// declares it.
//
// Why this exists: these strings used to be hand-copied into three places — the forge verify script,
// the forge test and the TS deploy test — with nothing comparing them to the contracts they describe.
// A generation sync bumps `MINOR_VERSION` upstream (0.12→0.13 moved four of them), so every copy had to
// be found and edited by hand, and a missed one failed as a confusing string mismatch rather than as
// "you forgot to update the table".
//
// The scan takes no configuration at all: any contract under pkg/src that declares CONTRACT_NAME plus
// the three version constants is picked up. Adding or removing a contract therefore needs no edit here,
// which is the property that makes a new protocol generation a zero-line diff for this file.

import { readdirSync, readFileSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { PKG_DIR_ABS_PATH } from './constants.ts';

////////////////////////////////////////////////////////////////////////////////

/** Root of the Solidity payload — every `.sol` below it is scanned. */
const SRC_DIR_ABS_PATH = join(PKG_DIR_ABS_PATH, 'src');

/**
 * The four constants a reporting contract declares. `CONTRACT_NAME` is deliberately part of the match:
 * it is the name the contract calls *itself*, which is not always the name of the file or the deployed
 * type — `CleartextFHEVMExecutor` inherits `getVersion` and reports `FHEVMExecutor`.
 */
const CONTRACT_NAME_RE = /CONTRACT_NAME\s*=\s*"([^"]+)"/;
const MAJOR_RE = /MAJOR_VERSION\s*=\s*(\d+)/;
const MINOR_RE = /MINOR_VERSION\s*=\s*(\d+)/;
const PATCH_RE = /PATCH_VERSION\s*=\s*(\d+)/;

export type ContractVersion = {
  /** The name the contract reports — the `CONTRACT_NAME` constant, not the file or type name. */
  readonly reportedName: string;
  /** The full string `getVersion()` returns, e.g. `ACL v0.4.0`. */
  readonly version: string;
  /** Path relative to the package root, for the generated file's provenance comment. */
  readonly sourcePath: string;
};

////////////////////////////////////////////////////////////////////////////////

function _solFilesUnder(dir: string): string[] {
  const found: string[] = [];
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) {
      found.push(..._solFilesUnder(path));
    } else if (entry.endsWith('.sol')) {
      found.push(path);
    }
  }
  return found;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Every contract under pkg/src that reports a version, sorted by reported name.
 *
 * A file matching `CONTRACT_NAME` but missing a version constant throws rather than being skipped: that
 * shape means a reporting contract was edited and the scan would otherwise quietly stop covering it,
 * which is the exact failure this module exists to prevent.
 *
 * @throws if a contract declares `CONTRACT_NAME` without all three version constants, or if two
 *         contracts report the same name with different versions.
 */
export function readContractVersions(): readonly ContractVersion[] {
  const byName = new Map<string, ContractVersion>();

  for (const path of _solFilesUnder(SRC_DIR_ABS_PATH)) {
    const source = readFileSync(path, 'utf8');
    const name = CONTRACT_NAME_RE.exec(source)?.[1];
    if (name === undefined) {
      continue;
    }

    const relativePath = path.slice(PKG_DIR_ABS_PATH.length + 1);
    const major = MAJOR_RE.exec(source)?.[1];
    const minor = MINOR_RE.exec(source)?.[1];
    const patch = PATCH_RE.exec(source)?.[1];
    if (major === undefined || minor === undefined || patch === undefined) {
      throw new Error(
        `${relativePath} declares CONTRACT_NAME = "${name}" but not all of ` +
          `MAJOR_VERSION / MINOR_VERSION / PATCH_VERSION. Either it no longer reports a version (drop ` +
          `CONTRACT_NAME) or the constants moved — this scan cannot guess which.`,
      );
    }

    const version = `${name} v${major}.${minor}.${patch}`;
    const existing = byName.get(name);
    if (existing !== undefined && existing.version !== version) {
      throw new Error(
        `Two contracts report the name "${name}" with different versions: ` +
          `${existing.sourcePath} says "${existing.version}", ${relativePath} says "${version}".`,
      );
    }
    byName.set(name, { reportedName: name, version, sourcePath: relativePath });
  }

  if (byName.size === 0) {
    throw new Error(`No versioned contracts found under ${SRC_DIR_ABS_PATH} — the scan is vacuous.`);
  }

  return [...byName.values()].sort((left, right) => left.reportedName.localeCompare(right.reportedName));
}

////////////////////////////////////////////////////////////////////////////////

/** `CleartextArithmetic` → `CLEARTEXT_ARITHMETIC`, matching the Solidity constant style elsewhere. */
export function solidityConstantName(reportedName: string): string {
  return reportedName
    .replace(/([a-z0-9])([A-Z])/g, '$1_$2')
    .replace(/([A-Z]+)([A-Z][a-z])/g, '$1_$2')
    .toUpperCase();
}

/** `CleartextArithmetic` → `cleartextArithmetic`, matching the TS key style elsewhere. */
export function tsKeyName(reportedName: string): string {
  // Leading acronym runs stay together: FHEVMExecutor -> fhevmExecutor, ACL -> acl, KMSVerifier -> kmsVerifier.
  const leadingAcronym = /^([A-Z]+)(?=[A-Z][a-z]|$)/.exec(reportedName);
  if (leadingAcronym?.[1] !== undefined) {
    const run = leadingAcronym[1];
    return run.toLowerCase() + reportedName.slice(run.length);
  }
  return reportedName.charAt(0).toLowerCase() + reportedName.slice(1);
}
