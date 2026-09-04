import { validateLockfiles } from '../base/checks/lockfiles.ts';
import type { CheckCommand } from '../base/command.ts';
import { loadPackages } from '../base/npm.ts';

export const checkLockfiles: CheckCommand = (context) => {
  const packages = loadPackages(context.workspaceRoot, context.manifest);
  return {
    command: 'check-lockfiles',
    checkedPackageKeys: packages.map((pkg) => pkg.key),
    violations: validateLockfiles(packages),
  };
};
