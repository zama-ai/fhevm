// CLI: bring up the two cleartext anvils and keep them running until Ctrl-C.
// Useful for local dev and for non-Playwright platforms (e.g. node-server).
//
//   PATH=~/.foundry/bin:$PATH npx tsx test/infra/anvil/spawn-anvils.ts
//
// Playwright globalSetup imports startAnvils/stopAnvils directly instead.

import { startAnvils, stopAnvils } from './anvils.js';
import { anvilSpecs } from '../topology.js';

// Single source of truth: derives ports, distinct chain ids, per-slot deployer
// mnemonics and ACL addresses from topology (so this CLI brings the anvils up
// identically to Playwright's globalSetup).
const handles = await startAnvils(anvilSpecs());

console.log(
  '✅ anvils ready:',
  handles.map((h) => `${h.spec.slot}@${h.rpcUrl}${h.reused ? ' (reused)' : ''}`).join(', '),
);
console.log('Press Ctrl-C to stop.');

let stopping = false;
const shutdown = async (): Promise<void> => {
  if (stopping) {
    return;
  }
  stopping = true;
  console.log('\nStopping anvils...');
  await stopAnvils(handles);
  process.exit(0);
};

process.on('SIGINT', () => void shutdown());
process.on('SIGTERM', () => void shutdown());
