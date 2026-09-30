import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

import { PACKAGE_ROOT_ABS_PATH } from './constants.ts';

export const EIP_170_RUNTIME_SIZE_LIMIT = 24_576;

export const CONTRACT_SIZE_EXCEPTIONS = new Set([
  'pkg/src/cleartext/CleartextForgeArithmetic.sol',
  'pkg/src/cleartext/CleartextForgeFHEVMExecutor.sol',
]);

export type DeployableContractSize = {
  readonly sourcePath: string;
  readonly contractName: string;
  readonly runtimeSize: number;
};

export type ContractSizeReport = {
  readonly contracts: readonly DeployableContractSize[];
  readonly violations: readonly DeployableContractSize[];
  readonly allowedOverflows: readonly DeployableContractSize[];
};

type ForgeArtifact = {
  readonly metadata?: {
    readonly settings?: {
      readonly compilationTarget?: Readonly<Record<string, string>>;
    };
  };
  readonly deployedBytecode?: { readonly object?: unknown };
};

export function checkContractSizes(outDirectory = join(PACKAGE_ROOT_ABS_PATH, 'out')): ContractSizeReport {
  if (!existsSync(outDirectory)) {
    throw new Error(`Forge output directory does not exist: ${outDirectory}. Run 'forge build --skip test' first.`);
  }

  const contracts = readDeployableContractSizes(outDirectory);
  if (contracts.length === 0) {
    throw new Error(`No deployable contracts under 'pkg/src' were found in ${outDirectory}`);
  }
  return assessContractSizes(contracts);
}

export function assessContractSizes(contracts: readonly DeployableContractSize[]): ContractSizeReport {
  const violations: DeployableContractSize[] = [];
  const allowedOverflows: DeployableContractSize[] = [];

  for (const contract of contracts) {
    if (contract.runtimeSize <= EIP_170_RUNTIME_SIZE_LIMIT) continue;
    if (CONTRACT_SIZE_EXCEPTIONS.has(contract.sourcePath)) {
      allowedOverflows.push(contract);
    } else {
      violations.push(contract);
    }
  }

  return { contracts, violations, allowedOverflows };
}

function readDeployableContractSizes(outDirectory: string): readonly DeployableContractSize[] {
  const contracts = new Map<string, DeployableContractSize>();

  for (const artifactPath of jsonFiles(outDirectory)) {
    const artifact = JSON.parse(readFileSync(artifactPath, 'utf8')) as ForgeArtifact;
    const compilationTarget = artifact.metadata?.settings?.compilationTarget;
    if (compilationTarget === undefined) continue;

    for (const [rawSourcePath, contractName] of Object.entries(compilationTarget)) {
      const sourcePath = rawSourcePath.replaceAll('\\', '/').replace(/^\.\//, '');
      if (!sourcePath.startsWith('pkg/src/')) continue;

      const bytecode = artifact.deployedBytecode?.object;
      if (typeof bytecode !== 'string' || !bytecode.startsWith('0x') || bytecode.length % 2 !== 0) {
        throw new Error(`Invalid deployed bytecode in ${artifactPath}`);
      }
      const runtimeSize = (bytecode.length - 2) / 2;
      if (runtimeSize === 0) continue;

      contracts.set(`${sourcePath}:${contractName}`, { sourcePath, contractName, runtimeSize });
    }
  }

  return [...contracts.values()].sort((left, right) => {
    const sourceOrder = left.sourcePath.localeCompare(right.sourcePath);
    return sourceOrder !== 0 ? sourceOrder : left.contractName.localeCompare(right.contractName);
  });
}

function jsonFiles(directory: string): readonly string[] {
  const files: string[] = [];
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) files.push(...jsonFiles(path));
    else if (entry.isFile() && entry.name.endsWith('.json')) files.push(path);
  }
  return files;
}
