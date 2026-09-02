import { inspectFoundry } from '../base/checks/foundry.ts';
import type { CheckCommand } from '../base/command.ts';

export const checkFoundry: CheckCommand = (context) => {
  const inspection = inspectFoundry(context.workspaceRoot, context.manifest);
  return {
    command: 'check-foundry',
    checkedPackageKeys: ['.'],
    checkedItemLabel: 'Foundry installation(s)',
    verboseSuccesses:
      inspection.violations.length === 0 && inspection.actualVersion !== undefined
        ? [
            `forge ${inspection.actualVersion} matches npm-manifest.json#foundry.version`,
            "no package-local '.foundry-version' files",
            `${String(inspection.fmtPackageKeys.length)} Foundry project(s) inherit the shared formatting policy`,
          ]
        : undefined,
    violations: inspection.violations,
  };
};
