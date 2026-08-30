import { relative } from 'node:path';

import { generateExports } from '../base/generate-exports.ts';

export function generateExportsCommand(options: { readonly manifestFile: string; readonly check: boolean }): void {
  const statuses = generateExports(options);
  const display = (path: string): string => relative(process.cwd(), path) || '.';

  if (!options.check) {
    for (const output of statuses) console.log(`✅ Generated ${display(output.path)}`);
    return;
  }

  const stale = statuses.filter((output) => output.status !== 'identical');
  if (stale.length > 0) {
    for (const output of stale) console.error(`❌ ${display(output.path)} (${output.status})`);
    throw new Error(`Generated exports do not match ${display(options.manifestFile)}`);
  }
  for (const output of statuses) console.log(`✅ ${display(output.path)} matches the export manifest`);
}
