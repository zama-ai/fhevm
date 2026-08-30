export type Violation = {
  readonly rule: string;
  readonly packageKey: string;
  readonly message: string;
};

/** One measured cost, printed under `--verbose` so an expensive check can say where its time went. */
export type Timing = { readonly label: string; readonly milliseconds: number };

export type CommandReport = {
  readonly command: string;
  readonly checkedPackageKeys: readonly string[];
  readonly checkedItemLabel?: string;
  readonly verboseSuccesses?: readonly string[];
  readonly violations: readonly Violation[];
  readonly timings?: readonly Timing[];
};

export function printReport(report: CommandReport, verbose: boolean): void {
  const violations = [...report.violations].sort(
    (left, right) =>
      left.packageKey.localeCompare(right.packageKey) ||
      left.rule.localeCompare(right.rule) ||
      left.message.localeCompare(right.message),
  );

  for (const violation of violations) {
    console.error(`❌ [${violation.rule}] ${violation.packageKey}: ${violation.message}`);
  }

  if (verbose && report.timings !== undefined) {
    for (const timing of report.timings) {
      console.log(`⏱️  ${timing.label}: ${timing.milliseconds.toFixed(0)}ms`);
    }
  }

  if (verbose) {
    if (report.verboseSuccesses !== undefined) {
      for (const success of report.verboseSuccesses) console.log(`✅ ${success}`);
    } else {
      const failedPackageKeys = new Set(violations.map((violation) => violation.packageKey));
      for (const packageKey of report.checkedPackageKeys) {
        if (!failedPackageKeys.has(packageKey)) console.log(`✅ ${packageKey}`);
      }
    }
  }

  if (violations.length > 0) {
    console.error(
      `❌ ${report.command}: ${violations.length} violation(s) across ${report.checkedPackageKeys.length} ${report.checkedItemLabel ?? 'package(s)'}.`,
    );
  } else if (verbose) {
    console.log(
      `✅ ${report.command}: ${report.checkedPackageKeys.length} ${report.checkedItemLabel ?? 'package(s)'} checked.`,
    );
  }
}
