import { inspectInventory } from '../base/checks/inventory.ts';
import type { CheckCommand } from '../base/command.ts';

export const checkManifestCoverage: CheckCommand = (context) => {
  const inspection = inspectInventory(context.workspaceRoot, context.manifest);
  return {
    command: 'check-manifest-coverage',
    checkedPackageKeys: inspection.checkedPackageKeys,
    violations: inspection.violations,
  };
};
