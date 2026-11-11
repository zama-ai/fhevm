/**
 * The engine's own test contracts (`contracts/`, compiled by `npm run build`): a scenario loads an
 * artifact by contract name and deploys it with the profile's coprocessor config as constructor
 * argument. The artifacts hold no protocol address, so the same build serves every profile.
 */
import { readdirSync } from 'node:fs';
import path from 'node:path';

import type { ResolvedContractsSection } from '../../../config/profiles/types.js';
import { MissingProfileSectionError } from '../../world/errors.js';
import { type ContractArtifact, loadArtifact } from '../chain/artifacts.js';

/** The `CoprocessorConfig` struct of `@fhevm/solidity`, the constructor argument of every engine contract. */
export interface CoprocessorConfig {
  ACLAddress: string;
  CoprocessorAddress: string;
  KMSVerifierAddress: string;
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
    /** Absolute path of the artifacts directory (`dist/contracts`). */
    readonly directory: string,
    private readonly profileName: string,
    private readonly section: ResolvedContractsSection | undefined,
  ) {}

  /**
   * The compiled artifact of `contractName`. Throws when no artifact has that name, or when several
   * do (the error lists them).
   */
  artifact(contractName: string): ContractArtifact {
    if (!IDENTIFIER.test(contractName)) throw new Error(`'${contractName}' is not a contract name`);
    let files: string[];
    try {
      files = artifactFiles(this.directory, contractName);
    } catch (error) {
      throw new Error(
        `cannot read the contracts directory ${this.directory} (run 'npm run build'): ${(error as Error).message}`,
      );
    }
    if (files.length === 0) throw new Error(`no compiled contract '${contractName}' in ${this.directory}`);
    if (files.length > 1) {
      const candidates = files.map((file) => path.relative(this.directory, file)).join(', ');
      throw new Error(`several compiled contracts are named '${contractName}': ${candidates}`);
    }
    return loadArtifact(files[0]!);
  }

  /**
   * The constructor argument of the engine contracts: the profile's ACL, FHEVM executor and KMS
   * verifier. Throws a configuration error when the profile has no `contracts` section.
   */
  coprocessorConfig(): CoprocessorConfig {
    if (!this.section) throw new MissingProfileSectionError(this.profileName, ['contracts']);
    return {
      ACLAddress: this.section.acl.value,
      CoprocessorAddress: this.section.executor.value,
      KMSVerifierAddress: this.section.kmsVerifier.value,
    };
  }
}
