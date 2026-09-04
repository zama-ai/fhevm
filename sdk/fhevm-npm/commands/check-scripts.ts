import { validateEslintConfigs, validatePrettierConfigs, validateScripts } from '../base/checks/scripts.ts';
import type { CheckCommand } from '../base/command.ts';
import { loadPackages } from '../base/npm.ts';

export const checkScripts: CheckCommand = (context) => {
  const packages = loadPackages(context.workspaceRoot, context.manifest);
  return {
    command: 'check-scripts',
    checkedPackageKeys: packages.map((pkg) => pkg.key),
    violations: [
      ...validateScripts(packages),
      ...validatePrettierConfigs(context.workspaceRoot, packages),
      ...validateEslintConfigs(packages),
    ],
  };
};
