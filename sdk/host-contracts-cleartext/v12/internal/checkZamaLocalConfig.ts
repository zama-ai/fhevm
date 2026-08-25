// Checks ZAMA_LOCAL_CONFIG against the real `library-solidity/config/ZamaConfig.sol`.
//
// The gap this closes: `localHostAddresses()` in generateLocalHostBytecode.ts already asserts that the
// DERIVED addresses equal ZAMA_LOCAL_CONFIG, and test/templates.test.ts asserts the generated forge
// constants do too. Every one of those checks compares against the same hand-written constant in
// constants.ts. Nothing compared that constant against the file it transcribes — so an upstream edit to
// `_getLocalConfig()` leaves the whole chain internally consistent and collectively wrong, which is the
// one failure mode rules 15 and 17 exist to prevent (those three literals are compiled into every dApp
// inheriting ZamaConfig's localhost config, and cannot be reconfigured after the fact).
//
// Read-only, and deliberately source-level: it parses the Solidity rather than compiling or deploying
// anything, so it costs nothing and runs before any build step that depends on the addresses.
//
// Two things it refuses to do quietly, because both look exactly like success:
//   - pass when ZamaConfig.sol cannot be found. The harness is never published (rule 9) and only ever
//     lives inside the fhevm repo (rule 1), so an absent file means it MOVED and the gate is dead.
//   - pass when `_getLocalConfig()` grew a field. A new address in the local config is one cleartext's
//     stack has to place; ignoring it would silently narrow the check to the fields we happen to know.

import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { PACKAGE_ROOT_ABS_PATH, ZAMA_LOCAL_CONFIG } from './constants.ts';

////////////////////////////////////////////////////////////////////////////////

/** Path of ZamaConfig.sol relative to the fhevm repo root. */
const ZAMA_CONFIG_REPO_PATH = join('library-solidity', 'config', 'ZamaConfig.sol');

/**
 * The chain id `ZamaConfig.getCoprocessorConfig()` gates its local branch on. Checked, not assumed: the
 * point of this script is that the LOCAL config is the one being transcribed, so a dispatch that sent
 * 31337 somewhere else would make a passing field comparison meaningless.
 */
const LOCAL_CHAIN_ID = 31337;

/**
 * `ZamaConfig`'s `CoprocessorConfig` field names, mapped to ours.
 *
 * `CoprocessorAddress` **is** the FHEVMExecutor address — the two names describe one contract, and
 * reading it as some other component is the easiest way to get this wrong (RULES.md rule 17).
 */
const FIELD_NAMES: Readonly<Record<string, keyof typeof ZAMA_LOCAL_CONFIG>> = {
  ACLAddress: 'aclAddress',
  CoprocessorAddress: 'fhevmExecutorAddress',
  KMSVerifierAddress: 'kmsVerifierAddress',
};

////////////////////////////////////////////////////////////////////////////////

export type ZamaLocalConfigEntry = {
  /** Field name as `ZamaConfig.sol` spells it. */
  readonly zamaField: string;
  /** The corresponding key of ZAMA_LOCAL_CONFIG. */
  readonly ourField: keyof typeof ZAMA_LOCAL_CONFIG;
  /** The literal read out of `_getLocalConfig()`. */
  readonly declared: string;
  /** What constants.ts claims it is. */
  readonly ours: string;
  readonly matches: boolean;
};

export type ZamaLocalConfigCheck = {
  /** Absolute path of the ZamaConfig.sol that was read. */
  readonly sourcePath: string;
  /** How to name that file in output — repo-relative where possible. See {@link sourceLabel}. */
  readonly label: string;
  readonly entries: readonly ZamaLocalConfigEntry[];
  readonly mismatches: readonly ZamaLocalConfigEntry[];
};

////////////////////////////////////////////////////////////////////////////////

function _gitRepoRoot(): string | undefined {
  try {
    return execFileSync('git', ['rev-parse', '--show-toplevel'], {
      cwd: PACKAGE_ROOT_ABS_PATH,
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'ignore'],
    }).trim();
  } catch {
    return undefined;
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Locates `library-solidity/config/ZamaConfig.sol`.
 *
 * Two candidates rather than one hardcoded path: the layout-relative guess (`sdk/host-contracts-cleartext/
 * <generation>/`, which holds for v11, v12 and v13 alike) and the git repo root, so moving this package
 * within the repo does not silently disable the check. Throws when neither exists — see the header.
 */
export function zamaConfigPath(): string {
  const layoutRelative = resolve(PACKAGE_ROOT_ABS_PATH, '..', '..', '..', ZAMA_CONFIG_REPO_PATH);
  const repoRoot = _gitRepoRoot();
  const candidates =
    repoRoot === undefined ? [layoutRelative] : [...new Set([layoutRelative, join(repoRoot, ZAMA_CONFIG_REPO_PATH)])];

  const found = candidates.find((candidate) => existsSync(candidate));
  if (found === undefined) {
    throw new Error(
      `ZamaConfig.sol not found. Tried:\n${candidates.map((candidate) => `     ${candidate}`).join('\n')}\n` +
        `   It is the source of truth for the localhost address set (RULES.md rules 15 and 17), so this ` +
        `check cannot be skipped: fix the path in internal/checkZamaLocalConfig.ts if the file moved.`,
    );
  }

  return found;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * How to name `sourcePath` in output: relative to the fhevm repo root when it sits inside one, absolute
 * otherwise. Package-relative would be worse — this file lives three levels below the package, so every
 * mention would open with `../../../`, and a path outside the tree entirely degenerates into a wall of
 * them.
 */
export function sourceLabel(sourcePath: string): string {
  const repoRoot = _gitRepoRoot();
  if (repoRoot === undefined) {
    return sourcePath;
  }

  const fromRoot = relative(repoRoot, sourcePath);
  return fromRoot.startsWith('..') ? sourcePath : fromRoot;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Strips Solidity comments so neither the brace matching below nor the "exactly one `_getLocalConfig`"
 * count can be fooled by a commented-out copy. Block comments first, then line comments — the other order
 * would let a line comment swallow the delimiter that ends a block one.
 */
function _stripComments(source: string): string {
  return source.replace(/\/\*[\s\S]*?\*\//g, ' ').replace(/\/\/[^\n]*/g, ' ');
}

/** The `{ … }` body of `_getLocalConfig()`, by brace matching from its signature. */
function _localConfigBody(source: string, label: string): string {
  const declarations = [...source.matchAll(/function\s+_getLocalConfig\s*\(/g)];
  if (declarations.length !== 1) {
    throw new Error(
      `${label} declares _getLocalConfig() ${String(declarations.length)} times, expected exactly 1. ` +
        `The localhost address set has moved or been duplicated — resolve it by hand.`,
    );
  }

  const [declaration] = declarations;
  if (declaration === undefined) {
    throw new Error(`${label}: unreachable — matchAll returned a hole`);
  }

  const open = source.indexOf('{', declaration.index + declaration[0].length);
  if (open === -1) {
    throw new Error(`${label}: _getLocalConfig() has no body`);
  }

  let depth = 0;
  for (let index = open; index < source.length; index++) {
    const character = source[index];
    if (character === '{') {
      depth++;
    } else if (character === '}') {
      depth--;
      if (depth === 0) {
        return source.slice(open + 1, index);
      }
    }
  }

  throw new Error(`${label}: _getLocalConfig() body is unterminated`);
}

/**
 * Asserts the chain-id dispatch still routes {@link LOCAL_CHAIN_ID} to `_getLocalConfig()`.
 *
 * Without this the field comparison could pass against a function nothing calls on 31337 any more.
 * `[^}]*?` keeps the match inside the branch rather than letting it run on into a later one.
 */
function _assertLocalBranch(source: string, label: string): void {
  const dispatch = new RegExp(`block\\.chainid\\s*==\\s*${String(LOCAL_CHAIN_ID)}[^}]*?_getLocalConfig\\s*\\(\\s*\\)`);
  if (!dispatch.test(source)) {
    throw new Error(
      `${label} has no \`block.chainid == ${String(LOCAL_CHAIN_ID)}\` branch returning _getLocalConfig(). ` +
        `The localhost config is reached some other way now, so this check no longer proves anything.`,
    );
  }
}

/** Every `Name: 0x<40 hex>` field of the struct literal, keyed by field name. */
function _parseFields(body: string, label: string): Map<string, string> {
  const fields = new Map<string, string>();
  for (const match of body.matchAll(/(\w+)\s*:\s*(0x[0-9a-fA-F]{40})\b/g)) {
    const [, name, address] = match;
    if (name === undefined || address === undefined) {
      throw new Error(`${label}: unreachable — field match returned a hole`);
    }
    if (fields.has(name)) {
      throw new Error(`${label}: _getLocalConfig() assigns ${name} twice`);
    }
    fields.set(name, address);
  }

  const expected = Object.keys(FIELD_NAMES);
  const missing = expected.filter((name) => !fields.has(name));
  if (missing.length > 0) {
    throw new Error(
      `${label}: _getLocalConfig() has no address literal for ${missing.join(', ')}. ` +
        `Either the field was renamed, or it is no longer a literal — both need a decision, not a default.`,
    );
  }

  const unknown = [...fields.keys()].filter((name) => !expected.includes(name));
  if (unknown.length > 0) {
    throw new Error(
      `${label}: _getLocalConfig() declares ${unknown.join(', ')}, which this package does not know about. ` +
        `A new address in the localhost config is one the cleartext stack has to place: add it to ` +
        `ZAMA_LOCAL_CONFIG and FIELD_NAMES, and to ADDRESS_NAMES / NONCE_OFFSET if the deploy creates it ` +
        `(README step 4).`,
    );
  }

  return fields;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Compares ZAMA_LOCAL_CONFIG field by field against `_getLocalConfig()`.
 *
 * Returns the comparison rather than throwing on a mismatch, so a caller can report every drifted field
 * at once — one address moving usually means all three did, and seeing one at a time is misleading.
 * Structural problems (no file, no function, unknown field) still throw: they mean the check itself is
 * broken, which is not a result.
 */
export function checkZamaLocalConfig(sourcePath: string = zamaConfigPath()): ZamaLocalConfigCheck {
  const label = sourceLabel(sourcePath);
  const source = _stripComments(readFileSync(sourcePath, 'utf8'));

  _assertLocalBranch(source, label);
  const fields = _parseFields(_localConfigBody(source, label), label);

  const entries: ZamaLocalConfigEntry[] = Object.entries(FIELD_NAMES).map(([zamaField, ourField]) => {
    const declared = fields.get(zamaField);
    if (declared === undefined) {
      throw new Error(`${label}: unreachable — ${zamaField} passed the presence check but is absent`);
    }
    const ours = ZAMA_LOCAL_CONFIG[ourField];
    return { zamaField, ourField, declared, ours, matches: declared.toLowerCase() === ours.toLowerCase() };
  });

  return { sourcePath, label, entries, mismatches: entries.filter((entry) => !entry.matches) };
}
