import { inspectCommitScope } from '../base/checks/commit-scope.ts';
import type { CheckCommand } from '../base/command.ts';

export const checkCommitScope: CheckCommand = (context) => {
  const inspection = inspectCommitScope(context.workspaceRoot);
  return {
    command: 'check-commit-scope',
    checkedPackageKeys: inspection.checkedFileKeys,
    checkedItemLabel: 'changed file(s)',
    violations: inspection.violations,
  };
};
