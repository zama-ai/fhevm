import {
  type VendoredCheckResult,
  addSpend,
  emptySpend,
  validateVendoredPackage,
  vendoredPackageKeys,
} from '../base/checks/vendored.ts';
import type { CommandContext } from '../base/command.ts';
import type { CommandReport, Timing } from '../base/diagnostics.ts';

export function checkVendored(context: CommandContext, packageSelector?: string): CommandReport {
  const selectors =
    packageSelector === undefined ? vendoredPackageKeys(context.workspaceRoot, context.manifest) : [packageSelector];
  const results = selectors.map((selector) =>
    validateVendoredPackage(context.workspaceRoot, context.manifest, selector),
  );

  return {
    command: 'check-vendored',
    checkedPackageKeys: results.map((result) => result.packageKey),
    checkedItemLabel: 'package(s)',
    verboseSuccesses: results.flatMap((result) => result.successes),
    violations: results.flatMap((result) => result.violations),
    timings: timings(results),
  };
}

/**
 * Per-package wall clock, then the two child processes that account for nearly all of it.
 *
 * The check is subprocess-bound: `git show` and `forge fmt` run once per vendored file, so the call
 * counts matter as much as the totals — they are what a future batching change would reduce.
 */
function timings(results: readonly VendoredCheckResult[]): readonly Timing[] {
  const total = emptySpend();
  for (const result of results) addSpend(total, result.spend);

  return [
    ...results.map((result) => ({ label: result.packageKey, milliseconds: result.milliseconds })),
    { label: `git show (${String(total.gitCalls)} calls)`, milliseconds: total.gitMilliseconds },
    { label: `forge fmt (${String(total.formatCalls)} calls)`, milliseconds: total.formatMilliseconds },
    {
      label: 'total',
      milliseconds: results.reduce((sum, result) => sum + result.milliseconds, 0),
    },
  ];
}
