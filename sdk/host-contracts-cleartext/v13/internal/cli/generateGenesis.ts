// Run: npm run generate:genesis
//
// Starts a throwaway anvil on port 8945 (deliberately not 8545, so a developer's own node is never
// touched), runs the real deploy through scripts/deploy.sh — verification included — and snapshots the
// result. Slow: it is a full deploy. See the module for which accounts are included and why.

import { relative } from 'node:path';
import { writeGenesis } from '../generateGenesis.ts';

const summary = await writeGenesis();

console.log('');
console.log(`  ${String(summary.accountCount)} accounts, ${String(summary.codeBytes)} B of code`);
console.log(`  deployer ${summary.deployer} nonce ${String(summary.deployerNonce)}`);
console.log(`  ${relative(process.cwd(), summary.path)} (${String(summary.bytes)} bytes)`);
console.log(`  sha256 ${summary.sha256}`);
