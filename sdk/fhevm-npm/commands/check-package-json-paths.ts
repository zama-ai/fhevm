import { inspectPackageJsonPaths } from '../base/checks/package-json-paths.ts';
import type { CheckCommand } from '../base/command.ts';
import { loadPackages } from '../base/npm.ts';

export const checkPackageJsonPaths: CheckCommand = (context) => {
  const inspection = inspectPackageJsonPaths(loadPackages(context.workspaceRoot, context.manifest));
  return {
    command: 'check-package-json-paths',
    checkedPackageKeys: inspection.checkedPackageKeys,
    checkedItemLabel: 'package.json file(s)',
    verboseSuccesses: inspection.successfulClaims,
    violations: inspection.violations,
  };
};
