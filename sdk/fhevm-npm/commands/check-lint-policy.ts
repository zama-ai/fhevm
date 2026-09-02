import { inspectLintPolicy } from '../base/checks/lint-policy.ts';
import type { CheckCommand } from '../base/command.ts';

export const checkLintPolicy: CheckCommand = (context) => {
  const inspection = inspectLintPolicy(context.workspaceRoot, context.manifest);
  return {
    command: 'check-lint-policy',
    checkedPackageKeys: inspection.checkedPaths,
    checkedItemLabel: 'path(s)',
    violations: inspection.violations,
  };
};
