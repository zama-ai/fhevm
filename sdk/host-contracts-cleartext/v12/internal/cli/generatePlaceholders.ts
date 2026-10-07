// Run: npm run generate:placeholders
//
// Must run BEFORE `forge build`: solc compiles the markers into the bytecode, so writing them afterwards
// leaves the previous run's values baked in at the offsets generateTemplates.ts records. See the module
// for what the markers are and why they are derived rather than chosen.

import { ADDRESS_NAMES } from '../constants.ts';
import { placeholderFor, writePlaceholders } from '../generatePlaceholders.ts';

writePlaceholders();

for (const name of ADDRESS_NAMES) {
  console.log(`  ${name.padEnd(30)} ${placeholderFor(name)}`);
}
