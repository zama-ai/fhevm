import { relative } from 'node:path';

import type { CheckCommand } from '../base/command.ts';
import { generateCleartextConfig } from '../base/generate-cleartext-config.ts';

// Read-only: every face of sdk/cleartext-config.json is re-rendered in memory and compared against the
// committed file. The check-* spelling of `generate-cleartext-config --check`, so the audit surface is
// uniform — same report shape, same exit codes, same verbosity handling as every other check.
export const checkCleartextConfig: CheckCommand = (context) => {
  const statuses = generateCleartextConfig({ workspaceRoot: context.workspaceRoot, check: true });
  const faceKey = (path: string): string => `./${relative(context.workspaceRoot, path)}`;

  return {
    command: 'check-cleartext-config',
    checkedPackageKeys: statuses.map((output) => faceKey(output.path)),
    checkedItemLabel: 'generated face(s)',
    violations: statuses
      .filter((output) => output.status !== 'identical')
      .map((output) => ({
        rule: 'cleartext-config-face',
        packageKey: faceKey(output.path),
        message:
          output.status === 'missing'
            ? `generated face is missing — run \`fhevm-npm generate-cleartext-config\` (a step of \`make generate\`)`
            : `generated face differs from sdk/cleartext-config.json — regenerate with ` +
              `\`fhevm-npm generate-cleartext-config\` (never edit the face)`,
      })),
  };
};
