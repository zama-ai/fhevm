// Run: npm run prepare:tarball-consumer

import { prepareTarballConsumer } from '../prepareTestTarballConsumer.ts';

const tarballPath = prepareTarballConsumer();
console.log(`[tarball-consumer] prepared fixture from ${tarballPath}`);

// Editors cache the previous resolution failure for this fixture, so the freshly installed package
// stays "missing" until their language servers are restarted. Say so explicitly — the error text
// ("Cannot find module '@fhevm/host-contracts-cleartext/ts'") gives no hint that it is stale.
console.log(
  [
    '',
    '────────────────────────────────────────────────────────────────────────────',
    ' ⚠️ EDITOR STILL SHOWING ERRORS IN test/ts ?',
    '',
    ' The fixture was just re-created, so the language servers are holding a',
    ' stale "Cannot find module \'@fhevm/host-contracts-cleartext/ts\'".',
    '',
    ' Fix it from the Command Palette:',
    '   🚚 → ESLint: Restart ESLint Server',
    '   🍔 → TypeScript: Restart TS Server',
    '────────────────────────────────────────────────────────────────────────────',
    '',
  ].join('\n'),
);
