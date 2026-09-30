// Locates sibling npm modules from the USER's project root, never from the plugin's own tree: the
// consumer's @fhevm/solidity is what its contracts compile against, and the consumer's @fhevm/sdk is
// what its tests run — the plugin must see the same copies. Resolution follows Node's own algorithm
// from the project root, so pnpm's nested store (symlinked) resolves to its real path like npm's flat
// node_modules does.

import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';

import { HardhatPluginError } from 'hardhat/plugins';

import {
  FHEVM_SDK_PACKAGE_NAME,
  FHEVM_SOLIDITY_CONFIG_FILE,
  FHEVM_SOLIDITY_PACKAGE_NAME,
  PLUGIN_ID,
} from './constants.js';

/** Resolves a module specifier as the project at `root` would; the named error tells the user what to install. */
export function resolveFromConsumer(specifier: string, root: string): string {
  const require = createRequire(join(root, 'package.json'));
  try {
    return require.resolve(specifier);
  } catch (error) {
    throw new HardhatPluginError(
      PLUGIN_ID,
      `Unable to resolve '${specifier}' from the project at ${root}. Is the package installed there?`,
      error instanceof Error ? error : undefined,
    );
  }
}

/** The installed directory of an npm package, as seen from the project at `root`. */
export function packageDirFromConsumer(packageName: string, root: string): string {
  return dirname(resolveFromConsumer(`${packageName}/package.json`, root));
}

// Getters resolve lazily: a project that never touches the SDK path must not fail on it.
export class FhevmPaths {
  readonly #root: string;

  constructor(root: string) {
    this.#root = root;
  }

  /** `/path/to/user-package` (= the hardhat project root). */
  get root(): string {
    return this.#root;
  }

  /** `/path/to/user-package/node_modules` */
  get nodeModulesDir(): string {
    return join(this.#root, 'node_modules');
  }

  /** `/path/to/user-package/node_modules/@fhevm/solidity` */
  get fhevmSolidityDir(): string {
    return packageDirFromConsumer(FHEVM_SOLIDITY_PACKAGE_NAME, this.#root);
  }

  /** `/path/to/user-package/node_modules/@fhevm/solidity/config/ZamaConfig.sol` */
  get fhevmSolidityConfigFile(): string {
    return join(this.fhevmSolidityDir, FHEVM_SOLIDITY_CONFIG_FILE);
  }

  /** The consumer's `@fhevm/sdk` directory: flat under npm, the real nested-store path under pnpm. */
  get fhevmSdkDir(): string {
    return packageDirFromConsumer(FHEVM_SDK_PACKAGE_NAME, this.#root);
  }
}
