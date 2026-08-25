// Generates pkg/forge/script/ComputeAddresses.s.sol from a Solidity template.
//
// That script is the third copy of the deploy's nonce layout — after pkg/ts/addresses.ts and
// NONCE_OFFSET in constants.ts — and the only one that used to spell the offsets out as literals,
// because Solidity cannot import TypeScript. Generating it removes the copy: the offsets and the config
// remapping prefix now come from constants.ts, and the layout table in its doc comment is rendered from
// the same numbers as the code below it, so the two cannot disagree.
//
// Deliberately a substitution, not a renderer: the template is valid-looking Solidity with `{{NAME}}`
// holes, so it stays readable and diffable as Solidity. Everything the deploy order does NOT decide is
// literal text in the template.
//
// Module only — importing this writes nothing. The command line lives in
// internal/cli/generateComputeAddressesScript.ts (`npm run generate:compute-addresses`).
//
// DEFERRED — reviewed and consciously left literal in the template for now. Each is a value duplicated
// somewhere else in the repo, so each is a candidate placeholder; none has bitten yet:
//
//   * `internal/.deploy-config` (ADDRESSES_DIR / ADDRESSES_FILE) is a fourth copy — scripts/deploy.sh
//     (CONFIG_DIR), scripts/anvil-local-v1.sh (BUILD_OUT), package.json's `clean`, and
//     internal/generateGenesis.ts all name it. Cheap to substitute, no readability cost.
//   * `DEPLOYER_PRIVATE_KEY` is shared with scripts/deploy.sh, which exports it. A stable env-var
//     contract, so low value.
//   * `pragma solidity ^0.8.24` appears twice — this script's own pragma and the one it writes into the
//     generated addresses.sol. It is the RULES.md rule 16 compile floor, duplicated repo-wide; a
//     placeholder here would fix one instance of many.

import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import {
  ADDRESSED_NONCE_COUNT,
  ADDRESS_NAMES,
  FHEVM_CONFIG_REMAPPING_PREFIX,
  NONCE_LABEL,
  NONCE_OFFSET,
  PACKAGE_ROOT_ABS_PATH,
  PKG_DIR_ABS_PATH,
  UNNAMED_NONCE_CONTRACTS,
  type AddressName,
} from './constants.ts';

////////////////////////////////////////////////////////////////////////////////

export const TEMPLATE_PATH = join(PACKAGE_ROOT_ABS_PATH, 'internal', 'templates', 'ComputeAddresses.s.sol.template');

export const OUTPUT_PATH = join(PKG_DIR_ABS_PATH, 'forge', 'script', 'ComputeAddresses.s.sol');

/** Address name created at each offset, inverted from NONCE_OFFSET. */
const NAME_AT_OFFSET = new Map<bigint, AddressName>(ADDRESS_NAMES.map((name) => [NONCE_OFFSET[name], name]));

/**
 * Local variable holding each address inside the generated `run()`.
 *
 * Written out rather than derived from the address name, for the same reason CONSTANT_NAMES is: no
 * transformation produces these. `ACL_ADDRESS` becomes `aclProxy` (loses "Address", gains "Proxy") and
 * `PAUSER_SET_ADDRESS` becomes `pauserSetAddr` — a different suffix, because PauserSet is not a proxy.
 *
 * Keyed by AddressName, so this map is the ONE place a new address has to be named. Before, the same
 * name appeared in three textual sites in the script and omitting one failed nowhere near the edit; now
 * omitting it here is a compile error.
 */
const SCRIPT_VARIABLE: Readonly<Record<AddressName, string>> = {
  ACL_ADDRESS: 'aclProxy',
  FHEVM_EXECUTOR_ADDRESS: 'fhevmExecutorProxy',
  KMS_VERIFIER_ADDRESS: 'kmsVerifierProxy',
  INPUT_VERIFIER_ADDRESS: 'inputVerifierProxy',
  HCU_LIMIT_ADDRESS: 'hcuLimitProxy',
  CLEARTEXT_ARITHMETIC_ADDRESS: 'cleartextArithmeticProxy',
  CLEARTEXT_DB_ADDRESS: 'cleartextDbProxy',
  PAUSER_SET_ADDRESS: 'pauserSetAddr',
};

/** Indentation of a statement inside `run()`. */
const BODY_INDENT = ' '.repeat(8);

/**
 * The named addresses in nonce order.
 *
 * Not ADDRESS_NAMES order: that list is a schema and puts PAUSER_SET_ADDRESS before the cleartext pair,
 * whereas it is created after them. Every block below uses nonce order, so the script reads in the order
 * the deploy actually happens — the hand-written original mixed the two orders, which said nothing.
 */
function namesInNonceOrder(): readonly AddressName[] {
  return [...ADDRESS_NAMES].sort((left, right) => (NONCE_OFFSET[left] < NONCE_OFFSET[right] ? -1 : 1));
}

/** `address <var> = vm.computeCreateAddress(deployer, nonce + <offset>);` per address. */
export function renderAddressDeclarations(): string {
  return namesInNonceOrder()
    .map(
      (name) =>
        `${BODY_INDENT}address ${SCRIPT_VARIABLE[name]} = vm.computeCreateAddress(deployer, nonce + ${NONCE_OFFSET[name]});`,
    )
    .join('\n');
}

/**
 * The `console.log` block, including the two fixed rows.
 *
 * They are generated rather than left literal because they share the alignment: the labels are padded to
 * one past the longest, so a new address with a longer name has to re-pad `Deployer:` too. Leaving those
 * two in the template would mean a table that goes crooked on the next address.
 */
export function renderAddressLogs(): string {
  const names = namesInNonceOrder();
  const labels = ['Deployer:', 'Starting nonce:', ...names.map((name) => `${name}:`)];
  const width = Math.max(...labels.map((label) => label.length)) + 1;

  const line = (label: string, value: string): string =>
    `${BODY_INDENT}console.log("${label.padEnd(width)}", ${value});`;

  return [
    line('Deployer:', 'deployer'),
    line('Starting nonce:', 'nonce'),
    ...names.map((name) => line(`${name}:`, SCRIPT_VARIABLE[name])),
  ].join('\n');
}

/** `content = string.concat(content, _constant("<NAME>", <var>));` per address. */
export function renderAddressConstants(): string {
  return namesInNonceOrder()
    .map((name) => `${BODY_INDENT}content = string.concat(content, _constant("${name}", ${SCRIPT_VARIABLE[name]}));`)
    .join('\n');
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The `N+k` layout table for the doc comment, rendered from NONCE_OFFSET.
 *
 * Emitted as ` *   N+0  <what>  → <ADDRESS_NAME>` lines with the arrow column aligned, matching the table
 * that used to be maintained by hand. The trailing entry is ACLOwner at ADDRESSED_NONCE_COUNT: it is
 * deployed, but no bytecode refers to its address, so it is documented and not pinned.
 */
export function renderNonceLayoutComment(): string {
  type Row = { readonly nonce: bigint; readonly what: string; readonly name: string };
  const rows: Row[] = [];

  for (let nonce = 0n; nonce < ADDRESSED_NONCE_COUNT; nonce++) {
    const name = NAME_AT_OFFSET.get(nonce);
    if (name === undefined) {
      const unnamed = UNNAMED_NONCE_CONTRACTS[Number(nonce)];
      if (unnamed === undefined) {
        throw new Error(
          `nonce ${nonce} has neither a named address nor an UNNAMED_NONCE_CONTRACTS entry — the deploy ` +
            `layout has a hole, so the generated table would silently skip it`,
        );
      }
      rows.push({ nonce, what: unnamed, name: '' });
      continue;
    }
    rows.push({ nonce, what: NONCE_LABEL[name], name });
  }

  rows.push({ nonce: ADDRESSED_NONCE_COUNT, what: 'ACLOwner', name: '(address referenced by nothing)' });

  // Align on the widest cell of each column, so adding a longer contract name reflows the whole table
  // rather than leaving one row out of line.
  const nonceWidth = Math.max(...rows.map((row) => `N+${row.nonce}`.length));
  const whatWidth = Math.max(...rows.map((row) => (row.name === '' ? 0 : row.what.length)));

  return rows
    .map((row) => {
      const label = `N+${row.nonce}`.padEnd(nonceWidth);
      if (row.name === '') {
        return ` *   ${label} ${row.what}`;
      }
      // The last row documents rather than maps, so it gets no arrow — and no column for one either.
      if (row.name.startsWith('(')) {
        return ` *   ${label} ${row.what.padEnd(whatWidth)}  ${row.name}`;
      }
      return ` *   ${label} ${row.what.padEnd(whatWidth)} → ${row.name}`;
    })
    .join('\n');
}

////////////////////////////////////////////////////////////////////////////////

/** Every `{{NAME}}` the template may contain, and what it is replaced with. */
export function substitutions(): ReadonlyMap<string, string> {
  return new Map<string, string>([
    ['CONFIG_REMAPPING_PREFIX', FHEVM_CONFIG_REMAPPING_PREFIX],
    ['NONCE_LAYOUT_COMMENT', renderNonceLayoutComment()],
    // The nonce offsets reach the output through these three blocks now, not as individual holes: the
    // set of addresses is itself derived, so the template cannot carry one line per address.
    ['ADDRESS_DECLARATIONS', renderAddressDeclarations()],
    ['ADDRESS_LOGS', renderAddressLogs()],
    ['ADDRESS_CONSTANTS', renderAddressConstants()],
  ]);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Substitutes every `{{NAME}}` in `template`.
 *
 * Throws on a hole with no value AND on a value that is never used: an unknown placeholder would ship as
 * literal `{{...}}` inside Solidity (a compile error, but only for whoever compiles it next), and an
 * unused value means a placeholder was renamed in the template and the generator was not updated.
 */
export function render(template: string, values: ReadonlyMap<string, string>): string {
  const used = new Set<string>();
  const output = template.replace(/\{\{([A-Z0-9_]+)\}\}/g, (_match, name: string) => {
    const value = values.get(name);
    if (value === undefined) {
      throw new Error(`${TEMPLATE_PATH} uses {{${name}}}, which the generator has no value for`);
    }
    used.add(name);
    return value;
  });

  const unused = [...values.keys()].filter((name) => !used.has(name));
  if (unused.length > 0) {
    throw new Error(
      `the generator provides ${unused.join(', ')}, which the template never uses — a placeholder was ` +
        `renamed or removed there without updating internal/generateComputeAddressesScript.ts`,
    );
  }

  return output;
}

////////////////////////////////////////////////////////////////////////////////

/** What the generated script should contain, without writing it. */
export function computeAddressesScript(): string {
  return render(readFileSync(TEMPLATE_PATH, 'utf8'), substitutions());
}

/** Writes pkg/forge/script/ComputeAddresses.s.sol. */
export function writeComputeAddressesScript(): string {
  const source = computeAddressesScript();
  writeFileSync(OUTPUT_PATH, source, 'utf8');
  return source;
}
