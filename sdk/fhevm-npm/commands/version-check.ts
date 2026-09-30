import type { CheckCommand } from '../base/command.ts';
import { inspectVersions } from '../base/version-check.ts';

/** `version check`: sdk/versions.json is valid and every derived version agrees with it. Read-only. */
export const versionCheck: CheckCommand = (context) => {
  const inspection = inspectVersions(context.workspaceRoot, context.manifest);
  return {
    command: 'version check',
    checkedPackageKeys: inspection.checkedPackageKeys,
    checkedItemLabel: 'central version(s)',
    violations: inspection.violations,
  };
};
