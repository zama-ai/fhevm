import { validateDependencies } from '../base/checks/dependencies.ts';
import type { CheckCommand } from '../base/command.ts';
import { loadPackages } from '../base/npm.ts';

export const checkDependencies: CheckCommand = (context) => {
  const packages = loadPackages(context.workspaceRoot, context.manifest);
  return {
    command: 'check-dependencies',
    checkedPackageKeys: packages.map((pkg) => pkg.key),
    violations: validateDependencies(context.workspaceRoot, context.manifest, packages),
  };
};
