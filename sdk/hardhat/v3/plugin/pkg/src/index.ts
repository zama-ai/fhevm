// The hardhat v3 fhevm plugin.
//
// Hardhat 3 plugins are declarative OBJECTS (no side-effect extendEnvironment): an id, task
// definitions with LAZY action modules, and hook handlers loaded on demand. Verified against
// hardhat 3.15. The network hooks attach `connection.fhevm` — fhevm state is per CONNECTION in
// hardhat 3, where v2 had a per-process singleton.

import { task } from 'hardhat/config';
import type { HardhatPlugin } from 'hardhat/types/plugins';

const plugin: HardhatPlugin = {
  id: 'fhevm',
  hookHandlers: {
    network: () => import('./internal/hooks/network.js'),
  },
  tasks: [
    task('hello', 'Print a greeting proving the fhevm plugin is wired into hardhat v3')
      .setAction(() => import('./tasks/hello.js'))
      .build(),
  ],
};

export default plugin;
export type { HardhatFhevm } from './internal/FhevmConnection.js';
export type * from './type-extensions.js';
