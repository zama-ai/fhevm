import { inspectPublishedFiles } from '../base/checks/published-files.ts';
import type { CheckCommand } from '../base/command.ts';

export const checkPublishedFiles: CheckCommand = (context) => {
  const inspection = inspectPublishedFiles(context.workspaceRoot, context.manifest);
  return {
    command: 'check published-files',
    checkedPackageKeys: inspection.checkedPackageKeys,
    checkedItemLabel: 'published payload(s)',
    violations: inspection.violations,
  };
};
