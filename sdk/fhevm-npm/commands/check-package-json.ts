import { sortPackageJson, validatePackageJson } from '../base/checks/package-json.ts';
import type { CheckCommand } from '../base/command.ts';
import { loadPackages } from '../base/npm.ts';

export const checkPackageJson: CheckCommand = (context) => {
  let packages = loadPackages(context.workspaceRoot, context.manifest);
  if (context.sortPackageJson === true) {
    for (const file of sortPackageJson(packages)) console.log(`✅ Sorted ${file}`);
    packages = loadPackages(context.workspaceRoot, context.manifest);
  }
  return {
    command: 'check-package-json',
    checkedPackageKeys: packages.map((pkg) => (pkg.key === '.' ? './package.json' : `${pkg.key}/package.json`)),
    checkedItemLabel: 'package.json file(s)',
    violations: validatePackageJson(packages, context.manifest.packageJson),
  };
};
