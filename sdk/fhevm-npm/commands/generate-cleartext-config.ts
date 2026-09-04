import { relative } from 'node:path';

import { generateCleartextConfig } from '../base/generate-cleartext-config.ts';

export function generateCleartextConfigCommand(options: {
  readonly workspaceRoot: string;
  readonly check: boolean;
}): void {
  const statuses = generateCleartextConfig(options);
  const display = (path: string): string => relative(process.cwd(), path) || '.';

  if (!options.check) {
    for (const output of statuses) console.log(`✅ Generated ${display(output.path)}`);
    return;
  }

  const stale = statuses.filter((output) => output.status !== 'identical');
  if (stale.length > 0) {
    for (const output of stale) console.error(`❌ ${display(output.path)} (${output.status})`);
    throw new Error('Generated cleartext-config faces do not match sdk/cleartext-config.json');
  }
  for (const output of statuses) console.log(`✅ ${display(output.path)} matches sdk/cleartext-config.json`);
}
