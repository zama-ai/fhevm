import type { InterfaceAbi } from 'ethers';
import { readFileSync } from 'node:fs';

/** The fields of a compiled contract artifact (Hardhat format) the Chain module uses. */
export interface ContractArtifact {
  contractName: string;
  abi: InterfaceAbi;
  bytecode: string;
}

const BYTECODE = /^0x(?:[0-9a-fA-F]{2})+$/;

/**
 * Checks an artifact object. An empty bytecode (`0x`) is rejected: it is an interface or an
 * abstract contract, which cannot be deployed.
 *
 * @param source where the artifact came from, used in error messages
 */
export function parseArtifact(value: unknown, source = 'artifact'): ContractArtifact {
  const record = (value ?? {}) as Record<string, unknown>;
  const problems: string[] = [];
  if (typeof record.contractName !== 'string' || record.contractName === '') problems.push("missing 'contractName'");
  if (!Array.isArray(record.abi)) problems.push("'abi' must be an array");
  if (typeof record.bytecode !== 'string' || !BYTECODE.test(record.bytecode)) {
    problems.push("'bytecode' must be non-empty 0x-prefixed hex (unlinked libraries are not supported)");
  }
  if (problems.length > 0) throw new Error(`${source}: invalid contract artifact: ${problems.join('; ')}`);
  return {
    contractName: record.contractName as string,
    abi: record.abi as InterfaceAbi,
    bytecode: record.bytecode as string,
  };
}

/** Reads a Hardhat artifact JSON file (`contractName`, `abi`, `bytecode`). */
export function loadArtifact(filePath: string): ContractArtifact {
  let parsed: unknown;
  try {
    parsed = JSON.parse(readFileSync(filePath, 'utf8'));
  } catch (error) {
    throw new Error(`${filePath}: cannot read contract artifact: ${(error as Error).message}`);
  }
  return parseArtifact(parsed, filePath);
}
