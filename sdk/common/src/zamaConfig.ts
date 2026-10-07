// Checks ZAMA_LOCAL_CONFIG against the real `library-solidity/config/ZamaConfig.sol`. Source-level and
// read-only: it parses the Solidity, compiling and deploying nothing.
//
// Two cases it refuses to pass quietly, because both look exactly like success: ZamaConfig.sol not being
// found, and `_getLocalConfig()` having grown a field this workspace does not place.

import { readFileSync } from 'node:fs';
import { LOCAL_CHAIN_ID, ZAMA_LOCAL_CONFIG } from './constants.ts';
import { sourceLabel, zamaConfigAbsPath } from './paths.ts';

////////////////////////////////////////////////////////////////////////////////

/**
 * `ZamaConfig`'s `CoprocessorConfig` field names, mapped to ours. `CoprocessorAddress` **is** the
 * FHEVMExecutor address — reading it as some other component is the easiest way to get this wrong.
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
  /** What ZAMA_LOCAL_CONFIG claims it is. */
  readonly ours: string;
  readonly matches: boolean;
};

export type ZamaLocalConfigCheck = {
  /** Absolute path of the ZamaConfig.sol that was read. */
  readonly sourcePath: string;
  /** How to name that file in output — repo-relative where possible. */
  readonly label: string;
  readonly entries: readonly ZamaLocalConfigEntry[];
  readonly mismatches: readonly ZamaLocalConfigEntry[];
};

////////////////////////////////////////////////////////////////////////////////

/**
 * Strips Solidity comments so a commented-out copy cannot fool the brace matching or the "exactly one
 * `_getLocalConfig`" count. Block comments first, or a line comment swallows a block's end delimiter.
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
 * Asserts the chain-id dispatch still routes {@link LOCAL_CHAIN_ID} to `_getLocalConfig()` — without it
 * the field comparison could pass against a function nothing calls on that chain any more.
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
      `${label}: _getLocalConfig() declares ${unknown.join(', ')}, which this workspace does not know ` +
        `about. A new address in the localhost config is one the cleartext stack has to place: add it to ` +
        `ZAMA_LOCAL_CONFIG and FIELD_NAMES, and to each generation's ADDRESS_NAMES / NONCE_OFFSET if the ` +
        `deploy creates it.`,
    );
  }

  return fields;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Compares ZAMA_LOCAL_CONFIG field by field against `_getLocalConfig()`. Returns the comparison instead
 * of throwing on a mismatch, so a caller can report every drifted field at once.
 *
 * @param sourcePath ZamaConfig.sol to read. Defaults to {@link zamaConfigAbsPath}.
 * @throws on a structural problem — no file, no function, an unknown field — which means the check itself
 *         is broken rather than that it has a result.
 */
export function checkZamaLocalConfig(sourcePath: string = zamaConfigAbsPath()): ZamaLocalConfigCheck {
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
