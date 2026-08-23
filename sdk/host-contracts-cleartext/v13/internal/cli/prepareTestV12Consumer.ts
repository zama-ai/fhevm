// Run: node internal/cli/prepareTestV12Consumer.ts
//
// No npm script of its own: internal/runUpgradeE2e.ts invokes it as one step of the upgrade-e2e flow.
// Exits 0 even when the v12 package is unavailable — the upgrade e2e is optional and self-skips.

import { prepareV12Consumer } from '../prepareTestV12Consumer.ts';

prepareV12Consumer();
