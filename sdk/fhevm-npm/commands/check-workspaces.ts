import { validateWorkspaces } from '../base/checks/workspaces.ts';
import type { CheckCommand } from '../base/command.ts';
import { loadPackages } from '../base/npm.ts';

export const checkWorkspaces: CheckCommand = (context) => {
  const packages = loadPackages(context.workspaceRoot, context.manifest);
  return {
    command: 'check-workspaces',
    checkedPackageKeys: packages.map((pkg) => pkg.key),
    violations: validateWorkspaces(packages),
  };
};
