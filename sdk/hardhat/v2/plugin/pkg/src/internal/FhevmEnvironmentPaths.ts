import * as path from 'path';
import * as resolve from 'resolve';

import { HardhatFhevmError } from '../error';
import constants from './constants';
import { toUnixRelPath } from './utils/path';

export class FhevmEnvironmentPaths {
  private readonly _root: string;

  constructor(root: string) {
    this._root = root;
  }

  /**
   * Returns `/path/to/user-package` (eq: hre.config.paths.root)
   */
  public get rootDir(): string {
    return this._root;
  }

  /**
   * Returns `/path/to/user-package/.env`
   */
  public get dotEnvFile(): string {
    return path.join(this._root, '.env');
  }

  /**
   * Returns `/path/to/user-package/node_modules`
   */
  public get nodeModulesDir(): string {
    return path.join(this.rootDir, 'node_modules');
  }

  /**
   * Returns `/path/to/user-package/fhevmTemp`
   */
  public get cacheDir(): string {
    return path.join(this.rootDir, 'fhevmTemp');
  }

  /**
   * Returns `/path/to/user-package/fhevmTemp/precompiled-fhevm-host-contracts-addresses.json`
   */
  /**
   * Returns `/path/to/user-package/fhevmTemp/@fhevm/solidity/config`
   */
  public get cacheFhevmSolidityConfigDir(): string {
    return path.join(
      this.cacheDir,
      path.join(constants.FHEVM_SOLIDITY_PACKAGE.name, path.dirname(constants.FHEVM_SOLIDITY_PACKAGE.configFile)),
    );
  }

  /**
   * Returns:
   * - Unix: `fhevmTemp/@fhevm/solidity/config`
   * - Windows: `fhevmTemp\@fhevm\solidity\config`
   */
  public get relCacheFhevmSolidityConfigDir(): string {
    const abs = this.cacheFhevmSolidityConfigDir;
    return path.relative(this.rootDir, abs);
  }

  /**
   * Returns `fhevmTemp/@fhevm/solidity/config` (in Unix format)
   */
  public get relCacheFhevmSolidityConfigDirUnix(): string {
    const abs = this.cacheFhevmSolidityConfigDir;
    return toUnixRelPath(path.relative(this.rootDir, abs));
  }

  /**
   * Returns `/path/to/user-package/fhevmTemp/@fhevm/solidity/config/ZamaConfig.sol`
   */
  public get cacheCoprocessorConfigSol(): string {
    return path.join(this.cacheFhevmSolidityConfigDir, path.basename(constants.FHEVM_SOLIDITY_PACKAGE.configFile));
  }

  /**
   * Returns `/path/to/user-package/node_modules/@fhevm/solidity`
   * This is legit since the user-package must have @fhevm/solidity in its dependencies.
   */
  public get fhevmSolidityDir(): string {
    return path.dirname(this._resolveFromConsumer(path.join(constants.FHEVM_SOLIDITY_PACKAGE.name, 'package.json')));
  }

  /**
   * Returns `/path/to/user-package/node_modules/solidity-coverage`
   */
  public get solidityCoverageDir(): string | undefined {
    try {
      return path.dirname(
        this._resolveFromConsumer(path.join(constants.SOLIDITY_COVERAGE_PACKAGE_NAME, 'package.json')),
      );
    } catch {
      return undefined;
    }
  }

  /**
   * Returns `/path/to/user-package/node_modules/@fhevm/solidity/config`
   */
  public get fhevmSolidityConfigDir(): string {
    return path.join(this.fhevmSolidityDir, path.dirname(constants.FHEVM_SOLIDITY_PACKAGE.configFile));
  }

  /**
   * Returns `/path/to/user-package/node_modules/@fhevm/solidity/config/ZamaConfig.sol`
   */
  public get fhevmSolidityConfigFile(): string {
    return path.join(this.fhevmSolidityDir, constants.FHEVM_SOLIDITY_PACKAGE.configFile);
  }

  /**
   * Returns `/path/to/user-package/node_modules/@fhevm/solidity/lib`
   */
  public get fhevmSolidityLibDir(): string {
    return path.join(this.fhevmSolidityDir, 'lib');
  }

  /**
   * If using npm:
   * - Returns `/path/to/user-package/node_modules/@fhevm/sdk`
   * If using pnpm (strict no hoist):
   * - Returns `/path/to/user-package/node_modules/.pnpm/@fhevm+sdk@...@...@.../node_modules/@fhevm/sdk`
   * If using any other package manager: path to the installed module
   */
  public get fhevmSdkDir(): string {
    return path.dirname(this._resolveFromConsumer(path.join(constants.FHEVM_SDK_PACKAGE.name, 'package.json')));
  }

  private _resolveFromConsumer(modulePathId: string): string {
    return resolveFromConsumer(modulePathId, this._root);
  }
}

export function resolveFromConsumer(modulePathId: string, basedir: string): string {
  try {
    const resolved = resolve.sync(modulePathId, {
      basedir,
    });
    return resolved;
  } catch {
    throw new HardhatFhevmError(`Unable to resolve ${modulePathId} from project at ${basedir}`);
  }
}
