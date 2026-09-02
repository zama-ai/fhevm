// Renders sdk/cleartext-config.json — the source of truth for the cleartext stack's shared values —
// into every file generated FROM it. Three faces:
//
//   common-vendored/src/cleartext-config.ts                    ONE copy; `sync-vendored` fans it out to
//                                                              each generation's pkg/ts (a published
//                                                              package cannot import the private helper).
//   <gen>/create2-deploy/script/FhevmCleartextConfig.sol       written PER GENERATION, directly — a .sol
//                                                              in common-vendored would make it a
//                                                              Solidity-owning package with no forge.
//   <gen>/scripts/cleartext-config.sh                          per generation likewise; sourced by the
//                                                              launchers, never executed.
//
// The generations come from the JSON's own `appliesTo.generations`, so adding one extends the fan-out.

import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

export type GeneratedCleartextConfigStatus = {
  readonly path: string;
  readonly status: 'identical' | 'missing' | 'different';
};

export type GenerateCleartextConfigOptions = {
  readonly workspaceRoot: string;
  readonly check: boolean;
};

type RenderedOutput = { readonly path: string; readonly content: string };

type ConstantEntry = {
  readonly value?: string;
  readonly alias?: string;
  readonly ts: 'bigint' | 'number' | 'string';
  readonly tsEmit?: 'bigint';
  readonly solidity: string;
  readonly formula?: string;
};

type LocalhostBlock = {
  readonly MNEMONIC: { readonly value: string };
  readonly DEPLOYER_ADDRESS_INDEX: { readonly value: string };
  readonly DEPLOYER_ADDRESS: { readonly value: string };
  readonly DEPLOYER_START_NONCE: { readonly value: string };
  readonly zamaConfigLocal: Readonly<Record<string, string>>;
};

type CleartextConfig = {
  readonly constants: ReadonlyMap<string, ConstantEntry>;
  readonly localhost: LocalhostBlock;
  readonly generations: readonly string[];
};

const CONFIG_FILE = 'cleartext-config.json';
const TS_FACE_PATH = ['common-vendored', 'src', 'cleartext-config.ts'];

export function generateCleartextConfig(
  options: GenerateCleartextConfigOptions,
): readonly GeneratedCleartextConfigStatus[] {
  const outputs = renderCleartextConfigFaces(options.workspaceRoot);
  if (options.check) return outputs.map(compareOutput);

  for (const output of outputs) {
    mkdirSync(dirname(output.path), { recursive: true });
    writeFileSync(output.path, output.content);
  }
  return outputs.map((output) => ({ path: output.path, status: 'identical' as const }));
}

export function renderCleartextConfigFaces(workspaceRoot: string): readonly RenderedOutput[] {
  const config = loadCleartextConfig(join(workspaceRoot, CONFIG_FILE));
  const generation = (gen: string, ...segments: readonly string[]): string =>
    join(workspaceRoot, 'host-contracts-cleartext', gen, ...segments);

  return [
    { path: join(workspaceRoot, ...TS_FACE_PATH), content: renderTsFace(config.constants) },
    ...config.generations.flatMap((gen) => [
      {
        path: generation(gen, 'create2-deploy', 'script', 'FhevmCleartextConfig.sol'),
        content: renderSolFace(config.constants),
      },
      { path: generation(gen, 'scripts', 'cleartext-config.sh'), content: renderShFace(config) },
    ]),
  ];
}

function compareOutput(output: RenderedOutput): GeneratedCleartextConfigStatus {
  if (!existsSync(output.path)) return { path: output.path, status: 'missing' };
  const status = readFileSync(output.path, 'utf8') === output.content ? 'identical' : 'different';
  return { path: output.path, status };
}

function loadCleartextConfig(configFile: string): CleartextConfig {
  if (!existsSync(configFile)) throw new Error(`${configFile} not found — it is the cleartext source of truth.`);
  const parsed = JSON.parse(readFileSync(configFile, 'utf8')) as {
    appliesTo?: { generations?: readonly string[] };
    constants?: Record<string, ConstantEntry>;
    localhost?: LocalhostBlock;
  };
  // Object key order is declaration order; the faces preserve it, so aliases follow what they alias.
  const constants = new Map(Object.entries(parsed.constants ?? {}));
  if (constants.size === 0) throw new Error(`${configFile} declares no "constants" — refusing to emit an empty face.`);
  for (const [name, entry] of constants) validateEntry(name, entry, constants);

  const generations = parsed.appliesTo?.generations ?? [];
  if (generations.length === 0) throw new Error(`${configFile} declares no "appliesTo.generations".`);
  for (const gen of generations) {
    if (!/^v\d+$/.test(gen)) throw new Error(`appliesTo.generations: '${gen}' is not a generation key like 'v13'`);
  }

  return { constants, localhost: validateLocalhost(configFile, parsed.localhost), generations };
}

function validateEntry(name: string, entry: ConstantEntry, declared: ReadonlyMap<string, ConstantEntry>): void {
  if (!/^[A-Z][A-Z0-9_]*$/.test(name)) throw new Error(`${name}: not a CONSTANT_CASE identifier`);
  if ((entry.value === undefined) === (entry.alias === undefined)) {
    throw new Error(`${name}: exactly one of "value" or "alias" must be present`);
  }
  if (entry.alias !== undefined && !declared.has(entry.alias)) {
    throw new Error(`${name}: aliases ${entry.alias}, which is not declared`);
  }
  if (!['bigint', 'number', 'string'].includes(entry.ts)) throw new Error(`${name}: unknown "ts" type ${entry.ts}`);
  if (entry.tsEmit !== undefined && (entry.tsEmit !== 'bigint' || entry.ts === 'string')) {
    throw new Error(`${name}: "tsEmit" may only widen a numeric "ts" to bigint`);
  }
  if (entry.value !== undefined && entry.ts !== 'string' && !/^\d+$/.test(entry.value)) {
    throw new Error(`${name}: numeric value must be decimal digits, got ${entry.value}`);
  }
  if (!/^(string|address|bytes32|u?int\d*|bool)$/.test(entry.solidity)) {
    throw new Error(`${name}: unknown "solidity" type ${entry.solidity}`);
  }
}

const ZAMA_LOCAL_FIELDS = ['ACLAddress', 'CoprocessorAddress', 'KMSVerifierAddress'] as const;

function validateLocalhost(configFile: string, localhost: LocalhostBlock | undefined): LocalhostBlock {
  if (localhost === undefined) throw new Error(`${configFile} declares no "localhost" block.`);
  for (const key of ['MNEMONIC', 'DEPLOYER_ADDRESS_INDEX', 'DEPLOYER_ADDRESS', 'DEPLOYER_START_NONCE'] as const) {
    if (typeof localhost[key]?.value !== 'string') throw new Error(`localhost.${key}: missing "value"`);
  }
  for (const field of ZAMA_LOCAL_FIELDS) {
    if (typeof localhost.zamaConfigLocal[field] !== 'string') {
      throw new Error(`localhost.zamaConfigLocal.${field}: missing`);
    }
  }
  return localhost;
}

////////////////////////////////////////////////////////////////////////////////
// TypeScript face
////////////////////////////////////////////////////////////////////////////////

function renderTsFace(constants: ReadonlyMap<string, ConstantEntry>): string {
  const entries = [...constants].map(([name, entry]) => renderTsEntry(name, entry));
  return `${TS_FACE_HEADER}\n${entries.join('\n\n')}\n`;
}

function renderTsEntry(name: string, entry: ConstantEntry): string {
  const comment = entry.formula === undefined ? '' : `// ${entry.formula}\n`;
  return `${comment}export const ${name} = ${tsLiteral(entry)};`;
}

function tsLiteral(entry: ConstantEntry): string {
  if (entry.alias !== undefined) return entry.alias;
  const value = entry.value ?? '';
  if (entry.ts === 'string') return quote(value);
  return (entry.tsEmit ?? entry.ts) === 'bigint' ? `${value}n` : value;
}

// Prettier's rule: single quotes unless the value contains one (the HD paths do). No value holds both.
function quote(value: string): string {
  if (value.includes("'") && value.includes('"')) throw new Error(`cannot quote a value holding both quote kinds`);
  return value.includes("'") ? `"${value}"` : `'${value}'`;
}

const TS_FACE_HEADER = `// AUTO-GENERATED by \`fhevm-npm generate-cleartext-config\` from sdk/cleartext-config.json — DO NOT EDIT.
//
// The JSON is THE source of truth for every value the cleartext stack's languages must agree on: it
// records the keccak FORMULA behind each derived value rather than only the hex, and each generation's
// \`test/cleartext-config-mirror.test.ts\` checks its copy of this module against it — name for name, in
// declaration order, value for value, bigint-vs-number literal shape included. Each generation's
// \`create2-deploy/script/FhevmCleartextConfig.sol\` is the same JSON's Solidity face.
//
// \`fhevm-npm sync-vendored\` copies this file into each generation's pkg/ts/ — a published package cannot
// depend on the private @fhevm/sdk-vendored-dev, so it compiles a byte-identical copy instead. The module
// is import-free and browser-safe by construction: the generator emits nothing but \`export const\` literals.
`;

////////////////////////////////////////////////////////////////////////////////
// Solidity face
////////////////////////////////////////////////////////////////////////////////

function renderSolFace(constants: ReadonlyMap<string, ConstantEntry>): string {
  const entries = [...constants].map(([name, entry]) => renderSolEntry(name, entry));
  return `${SOL_FACE_HEADER}library FhevmCleartextConfig {\n${entries.join('\n\n')}\n}\n`;
}

function renderSolEntry(name: string, entry: ConstantEntry): string {
  const comment = entry.formula === undefined ? '' : `    // ${entry.formula}\n`;
  return `${comment}    ${entry.solidity} internal constant ${name} = ${solLiteral(entry)};`;
}

function solLiteral(entry: ConstantEntry): string {
  if (entry.alias !== undefined) return entry.alias;
  const value = entry.value ?? '';
  // An address is a bare (checksummed) literal; everything non-numeric is a double-quoted string.
  if (entry.solidity === 'address') return value;
  if (/^u?int\d*$/.test(entry.solidity)) return value;
  if (value.includes('"')) throw new Error(`cannot emit a Solidity string holding a double quote`);
  return `"${value}"`;
}

const SOL_FACE_HEADER = `// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title  FhevmCleartextConfig
 * @notice AUTO-GENERATED by \`fhevm-npm generate-cleartext-config\` from sdk/cleartext-config.json —
 *         DO NOT EDIT.
 *
 * The JSON, at the repository's \`sdk/\` root, is where every value the harness, the payload and the
 * js-sdk cleartext relayer must agree on is DECIDED. This library is its Solidity face and
 * \`pkg/ts/cleartext-config.ts\` is the TypeScript face: same names verbatim, same declaration order,
 * equal values — \`test/cleartext-config-mirror.test.ts\` checks the mirror and re-derives every value
 * the JSON records a formula for. The \`note\` explaining each value lives in the JSON, once.
 *
 * Generated AND checked in, so an operator running \`forge script\` from an unbuilt checkout is never
 * blocked on a generator. Regenerate with \`make generate\` after editing the JSON.
 *
 * @dev A library rather than an abstract contract, so a script that needs one value can use it without
 *      inheriting, and so nothing here can be overridden by a subclass. \`internal constant\` costs no
 *      bytecode, which is why the mirror is COMPLETE rather than trimmed to what today's scripts use — a
 *      partial mirror is an invitation to declare the missing half somewhere else.
 */
`;

////////////////////////////////////////////////////////////////////////////////
// Shell face
////////////////////////////////////////////////////////////////////////////////

function renderShFace(config: CleartextConfig): string {
  const constants = [...config.constants].map(([name, entry]) => renderShEntry(name, entry)).join('\n');
  const l = config.localhost;
  return `${SH_FACE_HEADER}
${constants}

# The localhost deploy recipe. MNEMONIC is the DEPLOY mnemonic — NOT FHEVM_MNEMONIC, which derives the
# KMS and coprocessor signer pools. Two different strings with two different jobs: swap them and the
# stack's addresses look right while its signatures never verify.
MNEMONIC=${shQuote(l.MNEMONIC.value)}
DEPLOYER_ADDRESS_INDEX=${shQuote(l.DEPLOYER_ADDRESS_INDEX.value)}
DEPLOYER_ADDRESS=${shQuote(l.DEPLOYER_ADDRESS.value)}
DEPLOYER_START_NONCE=${shQuote(l.DEPLOYER_START_NONCE.value)}

# The three addresses ZamaConfig._getLocalConfig() compiles into consumer bytecode, under the names the
# launchers use. Note ZAMA_LOCAL_COPROCESSOR (ZamaConfig's CoprocessorAddress) IS the FHEVMExecutor.
ZAMA_LOCAL_ACL=${shQuote(l.zamaConfigLocal['ACLAddress'] ?? '')}
ZAMA_LOCAL_COPROCESSOR=${shQuote(l.zamaConfigLocal['CoprocessorAddress'] ?? '')}
ZAMA_LOCAL_KMS_VERIFIER=${shQuote(l.zamaConfigLocal['KMSVerifierAddress'] ?? '')}
`;
}

function renderShEntry(name: string, entry: ConstantEntry): string {
  const comment = entry.formula === undefined ? '' : `# ${entry.formula}\n`;
  const literal = entry.alias !== undefined ? `"$${entry.alias}"` : shQuote(entry.value ?? '');
  return `${comment}${name}=${literal}`;
}

// Everything is double-quoted (the HD paths hold single quotes), so nothing the shell expands may appear.
function shQuote(value: string): string {
  if (/["$\\`]/.test(value)) throw new Error(`cannot emit a shell value holding ", $, \\ or a backtick`);
  return `"${value}"`;
}

const SH_FACE_HEADER = `#!/usr/bin/env bash
# AUTO-GENERATED by \`fhevm-npm generate-cleartext-config\` from sdk/cleartext-config.json — DO NOT EDIT.
#
# Sourced, never executed: the shell face of the cleartext stack's source of truth. Every \`constants\`
# entry verbatim, then the localhost deploy recipe and the ZamaConfig trio under the names the launchers
# use. Values are baked in and CHECKED IN, so a script works from a checkout of one generation alone,
# with no jq and no reach above the package. Regenerate with \`make generate\` after editing the JSON.
# shellcheck disable=SC2034  # consumed by the scripts that source this file
`;
