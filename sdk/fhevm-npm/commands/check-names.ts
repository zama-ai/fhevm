import { validatePackageNames } from '../base/checks/package-names.ts';
import type { CheckCommand } from '../base/command.ts';
import { loadPackages } from '../base/npm.ts';

export const checkNames: CheckCommand = (context) => {
  const packages = loadPackages(context.workspaceRoot, context.manifest);
  return {
    command: 'check-names',
    checkedPackageKeys: packages.map((pkg) => pkg.key),
    violations: validatePackageNames(packages),
  };
};
