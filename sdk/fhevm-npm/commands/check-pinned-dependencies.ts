import { packageJsonKey } from '../base/checks/package-names.ts';
import { validatePinnedDependencies } from '../base/checks/pinned-dependencies.ts';
import type { CheckCommand } from '../base/command.ts';
import { loadPackages } from '../base/npm.ts';

export const checkPinnedDependencies: CheckCommand = (context) => {
  const packages = loadPackages(context.workspaceRoot, context.manifest);
  return {
    command: 'check pinned-dependencies',
    checkedPackageKeys: packages.map(packageJsonKey),
    checkedItemLabel: 'package.json file(s)',
    violations: validatePinnedDependencies(context.manifest, packages),
  };
};
