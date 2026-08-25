// Run: npm run generate:compute-addresses
//
// Renders internal/templates/ComputeAddresses.s.sol.template into
// pkg/forge/script/ComputeAddresses.s.sol. Runs as a step of `build:templates`; `test/templates.test.ts`
// fails if the committed output has drifted from the template.

import { relative } from 'node:path';
import { OUTPUT_PATH, TEMPLATE_PATH, writeComputeAddressesScript } from '../generateComputeAddressesScript.ts';

const source = writeComputeAddressesScript();
const lines = source.split('\n').length;

console.log(`  ${relative(process.cwd(), TEMPLATE_PATH)}`);
console.log(`  -> ${relative(process.cwd(), OUTPUT_PATH)} (${String(lines)} lines)`);
