import { validateOwnership } from '../base/checks/ownership.ts';
import type { CheckCommand } from '../base/command.ts';
import { loadPackages } from '../base/npm.ts';

export const checkOwnership: CheckCommand = (context) => {
  const packages = loadPackages(context.workspaceRoot, context.manifest);
  return {
    command: 'check-ownership',
    checkedPackageKeys: packages.map((pkg) => pkg.key),
    violations: validateOwnership(packages),
  };
};
