// Run: npm run generate:patch-sites
//
// Run KNOWINGLY, not as part of a build. `test/templates.test.ts` compares the live counts against that
// file, so refreshing it is how you accept a change; doing it automatically would mean the test could
// never fail. That is why this is not wired into `build:templates` alongside its siblings
// (generate:placeholders / generate:signers), which are build steps.
//
// Refresh only after understanding *why* the numbers moved. A count falling to 0 for an address the
// contracts still use means the deploy would bake in a placeholder — the deploy-time post-condition in
// pkg/ts/utils.ts (assertNoPlaceholdersRemain) is what blocks that, but the baseline is what tells you
// it happened.

import { relative } from 'node:path';
import { ADDRESS_NAMES } from '../constants.ts';
import { writePatchSites } from '../generatePatchSites.ts';
import { PATCH_SITES_PATH } from '../generateTemplates.ts';

const sites = writePatchSites();

let total = 0;
for (const contractName of Object.keys(sites).sort()) {
  const counts = sites[contractName];
  if (counts === undefined) {
    continue;
  }
  const patched = ADDRESS_NAMES.filter((name) => (counts[name] ?? 0) > 0);
  const subtotal = patched.reduce((sum, name) => sum + (counts[name] ?? 0), 0);
  total += subtotal;
  console.log(`  ${contractName.padEnd(24)} ${String(subtotal).padStart(4)} sites  ${patched.join(' ')}`);
}
console.log(`\n  ${String(total)} patch sites across ${String(Object.keys(sites).length)} contracts`);
console.log(`  wrote ${relative(process.cwd(), PATCH_SITES_PATH)} — review the diff before committing.`);
