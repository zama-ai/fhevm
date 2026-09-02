import { inspectTscMode } from '../base/checks/tsc-mode.ts';
import type { CheckCommand } from '../base/command.ts';

export const checkTscMode: CheckCommand = (context) => {
  const inspection = inspectTscMode(context.workspaceRoot, context.manifest);
  return {
    command: 'check-tsc-mode',
    checkedPackageKeys: inspection.checkedInvocationKeys,
    checkedItemLabel: 'tsc invocation(s)',
    verboseSuccesses: inspection.successfulInvocations,
    violations: inspection.violations,
  };
};
