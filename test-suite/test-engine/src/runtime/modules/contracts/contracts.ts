/**
 * Compiled contracts of the run (`run --contracts-dir`, built by `test-engine compile`): a scenario
 * loads its artifacts by contract name instead of shipping them.
 */
import { readdirSync } from 'node:fs';
import path from 'node:path';

import { MISSING_CONTRACTS_DIR_ERROR } from '../../../model/execution.js';
import { type ContractArtifact, loadArtifact } from '../chain/artifacts.js';

/**
 * A step loaded a compiled contract, but the run was given no `--contracts-dir`. The engine
 * reports it as a configuration error.
 */
export class MissingContractsDirError extends Error {
  constructor(contractName: string) {
    super(
      `contract '${contractName}' needs compiled contracts, but the run has no --contracts-dir; compile them with 'test-engine compile --profile <name>' and pass the directory it prints`,
    );
    this.name = MISSING_CONTRACTS_DIR_ERROR;
  }
}

const IDENTIFIER = /^[A-Za-z_$][A-Za-z0-9_$]*$/;

/** Hardhat artifact files below `dir`: `<source>.sol/<Name>.json`, without `build-info` and debug files. */
function artifactFiles(dir: string, name: string): string[] {
  const found: string[] = [];
  const walk = (current: string) => {
    for (const entry of readdirSync(current, { withFileTypes: true })) {
      const full = path.join(current, entry.name);
      if (entry.isDirectory()) {
        if (entry.name !== 'build-info') walk(full);
      } else if (entry.name === `${name}.json` && path.basename(current).endsWith('.sol')) {
        found.push(full);
      }
    }
  };
  walk(dir);
  return found.sort();
}

export class Contracts {
  constructor(
    /** Absolute path of the artifacts directory; undefined when the run has no `--contracts-dir`. */
    readonly directory: string | undefined,
  ) {}

  /**
   * The compiled artifact of `contractName`. Throws when the run has no `--contracts-dir`
   * (configuration error), when no artifact has that name, or when several do (the error lists
   * them).
   */
  artifact(contractName: string): ContractArtifact {
    if (!IDENTIFIER.test(contractName)) throw new Error(`'${contractName}' is not a contract name`);
    if (this.directory === undefined) throw new MissingContractsDirError(contractName);
    let files: string[];
    try {
      files = artifactFiles(this.directory, contractName);
    } catch (error) {
      throw new Error(`cannot read the contracts directory ${this.directory}: ${(error as Error).message}`);
    }
    if (files.length === 0) throw new Error(`no compiled contract '${contractName}' in ${this.directory}`);
    if (files.length > 1) {
      const candidates = files.map((file) => path.relative(this.directory!, file)).join(', ');
      throw new Error(`several compiled contracts are named '${contractName}': ${candidates}`);
    }
    return loadArtifact(files[0]!);
  }
}
