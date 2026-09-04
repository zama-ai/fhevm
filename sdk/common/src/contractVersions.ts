// Reading the `getVersion()` string of every contract that reports one, from the Solidity that declares
// it. Takes no configuration: a contract is picked up by declaring the four constants, so adding or
// removing one needs no edit here.

import { readdirSync, readFileSync, statSync } from 'node:fs';
import { join } from 'node:path';

/**
 * The four constants a reporting contract declares. `CONTRACT_NAME` is part of the match because it is
 * the name the contract calls *itself*, which is not always its file or type name.
 */
const CONTRACT_NAME_RE = /CONTRACT_NAME\s*=\s*"([^"]+)"/;
const MAJOR_RE = /MAJOR_VERSION\s*=\s*(\d+)/;
const MINOR_RE = /MINOR_VERSION\s*=\s*(\d+)/;
const PATCH_RE = /PATCH_VERSION\s*=\s*(\d+)/;

export type ContractVersion = {
  /** The name the contract reports — its `CONTRACT_NAME` constant, not the file or type name. */
  readonly reportedName: string;
  /** The full string `getVersion()` returns, e.g. `ACL v0.4.0`. */
  readonly version: string;
  /** Path of the declaring file, relative to the caller's chosen base. */
  readonly sourcePath: string;
};

////////////////////////////////////////////////////////////////////////////////

/** Every `.sol` file under `dir`, recursively. */
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
 * Every contract under `srcDirAbsPath` that reports a version, sorted by reported name.
 *
 * @param srcDirAbsPath Directory to scan recursively for `.sol` files.
 * @param relativeToAbsPath Base that `sourcePath` is reported against. Defaults to `srcDirAbsPath`.
 * @throws if the scan finds nothing, if a contract declares `CONTRACT_NAME` without all three version
 *         constants, or if two contracts report the same name with different versions.
 * @example
 * readContractVersions('/pkg/src', '/pkg');
 * // [{ reportedName: 'ACL', version: 'ACL v0.4.0', sourcePath: 'src/contracts/ACL.sol' }, …]
 */
export function readContractVersions(
  srcDirAbsPath: string,
  relativeToAbsPath: string = srcDirAbsPath,
): readonly ContractVersion[] {
  const byName = new Map<string, ContractVersion>();

  for (const path of _solFilesUnder(srcDirAbsPath)) {
    const source = readFileSync(path, 'utf8');
    const name = CONTRACT_NAME_RE.exec(source)?.[1];
    if (name === undefined) {
      continue;
    }

    const relativePath = path.slice(relativeToAbsPath.length + 1);
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
    throw new Error(`No versioned contracts found under ${srcDirAbsPath} — the scan is vacuous.`);
  }

  return [...byName.values()].sort((left, right) => left.reportedName.localeCompare(right.reportedName));
}

////////////////////////////////////////////////////////////////////////////////

/**
 * A reported contract name as a Solidity constant name.
 *
 * @example
 * solidityConstantName('CleartextArithmetic'); // 'CLEARTEXT_ARITHMETIC'
 */
export function solidityConstantName(reportedName: string): string {
  return reportedName
    .replace(/([a-z0-9])([A-Z])/g, '$1_$2')
    .replace(/([A-Z]+)([A-Z][a-z])/g, '$1_$2')
    .toUpperCase();
}

/**
 * A reported contract name as a TypeScript key. A leading acronym run stays together.
 *
 * @example
 * tsKeyName('CleartextArithmetic'); // 'cleartextArithmetic'
 * tsKeyName('KMSVerifier'); // 'kmsVerifier'
 */
export function tsKeyName(reportedName: string): string {
  const leadingAcronym = /^([A-Z]+)(?=[A-Z][a-z]|$)/.exec(reportedName);
  if (leadingAcronym?.[1] !== undefined) {
    const run = leadingAcronym[1];
    return run.toLowerCase() + reportedName.slice(run.length);
  }
  return reportedName.charAt(0).toLowerCase() + reportedName.slice(1);
}
