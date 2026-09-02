import { validateMirror } from '../base/checks/mirror.ts';
import type { CommandContext } from '../base/command.ts';
import type { CommandReport } from '../base/diagnostics.ts';

export function checkMirror(context: CommandContext, packageSelector: string): CommandReport {
  const result = validateMirror(context.workspaceRoot, context.manifest, packageSelector);
  return {
    command: 'check-mirror',
    checkedPackageKeys: [result.packageKey],
    checkedItemLabel: 'mirror(s)',
    verboseSuccesses:
      result.violations.length === 0
        ? [`${result.packageKey}: ${result.comparedFiles} tracked files match ${result.repository}`]
        : [],
    violations: result.violations,
  };
}
