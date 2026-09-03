// The hardhat v3 fhevm plugin.
//
// Hardhat 3 plugins are declarative OBJECTS (no side-effect extendEnvironment): an id, task
// definitions with LAZY action modules, and hook handlers loaded on demand. `definePlugin` registers
// the id so the CLI can warn when the plugin is imported but missing from the user's `plugins`.
//
// fhevm state lives on the CONNECTION only (`connection.fhevm`, attached by the network hooks), never
// on the HRE — the pattern hardhat 3's own plugins follow (`connection.ethers`). There is no
// `hre.fhevm`: hardhat 3 has no default-connection object to alias.

import { task } from 'hardhat/config';
import { definePlugin } from 'hardhat/plugins';
import type { HardhatPlugin } from 'hardhat/types/plugins';

const plugin: HardhatPlugin = definePlugin({
  id: 'fhevm',
  npmPackage: '@fhevm/hardhat-plugin',
  hookHandlers: {
    network: () => import('./internal/hooks/network.js'),
  },
  tasks: [
    task('hello', 'Print a greeting proving the fhevm plugin is wired into hardhat v3')
      .setAction(() => import('./tasks/hello.js'))
      .build(),
  ],
});

export default plugin;
// The public API, in one module — see types.ts.
export * from './types.js';
export { timestampNow } from './internal/time.js';
export type * from './type-extensions.js';
