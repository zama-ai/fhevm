// The hardhat v3 fhevm plugin.
//
// Hardhat 3 plugins are declarative OBJECTS (no side-effect extendEnvironment): an id, task
// definitions with LAZY action modules, and hook handlers loaded on demand. `definePlugin` registers
// the id so the CLI can warn when the plugin is imported but missing from the user's `plugins`.
//
// fhevm state lives on the CONNECTION only (`connection.fhevm`, attached by the network hooks), never
// on the HRE — the pattern hardhat 3's own plugins follow (`connection.ethers`). There is no
// `hre.fhevm`: hardhat 3 has no default-connection object to alias.

import { emptyTask, task } from 'hardhat/config';
import { definePlugin } from 'hardhat/plugins';
import { ArgumentType } from 'hardhat/types/arguments';
import type { HardhatPlugin } from 'hardhat/types/plugins';

const plugin: HardhatPlugin = definePlugin({
  id: 'fhevm',
  npmPackage: '@fhevm/hardhat-plugin',
  hookHandlers: {
    network: () => import('./internal/hooks/network.js'),
  },
  // `hardhat fhevm <task>`: an empty scope root, then one lazily-loaded action per task. Required
  // inputs are positional (hardhat 3 options always carry a default).
  tasks: [
    emptyTask(['fhevm'], 'FHEVM related commands').build(),
    task(['fhevm', 'public-decrypt'], 'Performs a public decryption of the specified byte-32 handle')
      .addPositionalArgument({
        name: 'type',
        description: 'The FHEVM primitive type name (ebool, euint8, …, eaddress)',
      })
      .addPositionalArgument({ name: 'handle', description: 'The byte-32 handle to decrypt' })
      .setAction(() => import('./tasks/publicDecrypt.js'))
      .build(),
    task(['fhevm', 'user-decrypt'], 'Performs a user decryption of the specified byte-32 handle')
      .addPositionalArgument({
        name: 'type',
        description: 'The FHEVM primitive type name (ebool, euint8, …, eaddress)',
      })
      .addPositionalArgument({ name: 'handle', description: 'The byte-32 handle to decrypt' })
      .addPositionalArgument({ name: 'contract', description: 'The contract address the handle is allowed for' })
      .addOption({
        name: 'user',
        description: 'The decrypting account, by index',
        type: ArgumentType.INT,
        defaultValue: 0,
      })
      .setAction(() => import('./tasks/userDecrypt.js'))
      .build(),
  ],
});

export default plugin;
// The public API, in one module — see types.ts.
export * from './types.js';
export { timestampNow } from './internal/time.js';
// HCU price lookup by executor event name, e.g. `getHCU('FheAdd', 'Uint8')`.
export { getHCU } from './internal/hcu/prices.js';
export type * from './type-extensions.js';
