import { inspectTsconfigPaths } from '../base/checks/tsconfig-paths.ts';
import type { CheckCommand } from '../base/command.ts';

export const checkTsconfigPaths: CheckCommand = (context) => {
  const inspection = inspectTsconfigPaths(context.workspaceRoot, context.manifest);
  return {
    command: 'check-tsconfig-paths',
    checkedPackageKeys: inspection.checkedConfigKeys,
    checkedItemLabel: 'tsconfig(s)',
    verboseSuccesses: inspection.successfulClaims,
    violations: inspection.violations,
  };
};
