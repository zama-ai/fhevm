// Run: npm run list:upgrade-ops -- ../v12
//
// Read-only: writes nothing, compiles nothing, reads only committed JSON.
//
// The verdicts mirror README step 7: a bytecode change without a version bump means an
// upgrade of that proxy would carry no replay guard; a bump without a bytecode change is a wasted version.
// Those two are flagged, because they are always a manual decision and never a mechanical fix.

import { basename, resolve } from 'node:path';
import { PACKAGE_ROOT_ABS_PATH } from '../constants.ts';
import { listUpgradeOps, type UpgradeOp, type Verdict } from '../listUpgradeOps.ts';

/** Verdicts that mean the two signals disagree — always a manual decision, never a mechanical fix. */
const SUSPECT: ReadonlySet<Verdict> = new Set<Verdict>(['CHANGED, NOT BUMPED', 'BUMPED, UNCHANGED']);

/**
 * The initializer column. A materialization has no "before", so the useful thing to show is which calls
 * are available — `initializeFrom{EmptyProxy,Migration}()` when both are, since choosing between them is
 * a decision. A non-target has nothing to show at all.
 */
function _initializerCell(op: UpgradeOp): string {
  if (op.verdict === 'not a proxy target' || op.verdict === 'removed upstream') {
    return '';
  }
  if (op.verdict === 'materialize') {
    const suffixes = op.materializers.map((name) => name.replace('initializeFrom', ''));
    return suffixes.length > 1 ? `initializeFrom{${suffixes.join(',')}}()` : `${op.materializers[0] ?? '?'}()`;
  }
  return `${op.previousReinitializer ?? '-'} -> ${op.currentReinitializer ?? '-'}`;
}

const previous = process.argv[2];
if (previous === undefined || previous === '') {
  console.error('usage: npm run list:upgrade-ops -- <path to previous generation package>');
  console.error('   eg: npm run list:upgrade-ops -- ../v12');
  process.exit(1);
}

// PACKAGE_ROOT_ABS_PATH rather than a path relative to this file: this file sits one directory deeper
// than the module it drives, so `import.meta.dirname/..` would resolve to internal/, not the root.
const currentRoot = PACKAGE_ROOT_ABS_PATH;
const previousRoot = resolve(previous);
console.log(`  ${basename(previousRoot)} -> ${basename(currentRoot)}\n`);
console.log(`  ${'contract'.padEnd(24)} ${'bytecode'.padEnd(9)} ${'initializer'.padEnd(40)} verdict`);

const ops = listUpgradeOps(previousRoot, currentRoot);
for (const op of ops) {
  const bytecode = op.bytecodeChanged === undefined ? '-' : op.bytecodeChanged ? 'CHANGED' : 'same';
  const call = _initializerCell(op);
  const flag = SUSPECT.has(op.verdict) ? '⚠ ' : '';
  console.log(`  ${op.contractName.padEnd(24)} ${bytecode.padEnd(9)} ${call.padEnd(40)} ${flag}${op.verdict}`);
}

const count = (verdict: Verdict): string => String(ops.filter((op) => op.verdict === verdict).length);
const suspect = ops.filter((op) => SUSPECT.has(op.verdict));
console.log(`\n  ${count('materialize')} materializations, ${count('reinitialize')} reinitializations`);
if (suspect.length > 0) {
  console.log(`  ⚠ ${String(suspect.length)} contract(s) need a look — see README step 7.`);
}
