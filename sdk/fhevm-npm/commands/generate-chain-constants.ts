import { relative } from 'node:path';

import { generateChainConstants } from '../base/generate-chain-constants.ts';

export function generateChainConstantsCommand(options: {
  readonly workspaceRoot: string;
  readonly check: boolean;
}): void {
  const output = generateChainConstants(options);
  const display = relative(process.cwd(), output.path) || '.';

  if (!options.check) {
    console.log(`✅ Generated ${display}`);
    return;
  }
  if (output.status !== 'identical') {
    console.error(`❌ ${display} (${output.status})`);
    throw new Error('The generated chain constants do not match sdk/fhevm-chains.config.json');
  }
  console.log(`✅ ${display} matches sdk/fhevm-chains.config.json`);
}
