/**
 * The build stamp `test-engine compile` writes next to the compiled artifacts: which profile and
 * which coprocessor addresses the contracts were compiled for. The preflight compares it with the
 * profile of the run, so that artifacts compiled for another environment are never deployed.
 */
import { readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';

import type { ResolvedContractsSection } from '../../../config/profiles/types.js';

/** File name of the stamp, at the root of the artifacts directory. */
export const BUILD_STAMP_FILE = 'test-engine-build.json';

/** The addresses compiled into `E2ECoprocessorConfig`, in the order of the Solidity struct. */
export const COMPILED_ADDRESSES = ['acl', 'executor', 'kmsVerifier'] as const;

export type CompiledAddresses = Record<(typeof COMPILED_ADDRESSES)[number], string>;

export interface BuildStamp {
  profile: string;
  addresses: CompiledAddresses;
  compiledAt: string;
}

/** The addresses of a `contracts` section that `test-engine compile` compiles in. */
export function compiledAddressesOf(contracts: ResolvedContractsSection): CompiledAddresses {
  return {
    acl: contracts.acl.value,
    executor: contracts.executor.value,
    kmsVerifier: contracts.kmsVerifier.value,
  };
}

export function writeBuildStamp(artifactsDir: string, stamp: BuildStamp): string {
  const file = path.join(artifactsDir, BUILD_STAMP_FILE);
  writeFileSync(file, `${JSON.stringify(stamp, null, 2)}\n`);
  return file;
}

/** Reads the stamp of `artifactsDir`; throws a readable error when it is missing or malformed. */
export function readBuildStamp(artifactsDir: string): BuildStamp {
  const file = path.join(artifactsDir, BUILD_STAMP_FILE);
  let parsed: unknown;
  try {
    parsed = JSON.parse(readFileSync(file, 'utf8'));
  } catch (error) {
    const missing = (error as { code?: string }).code === 'ENOENT';
    throw new Error(
      missing
        ? `no ${BUILD_STAMP_FILE} in ${artifactsDir}: compile the contracts with 'test-engine compile'`
        : `${file}: ${(error as Error).message}`,
    );
  }
  const stamp = parsed as Partial<BuildStamp>;
  const addresses = stamp.addresses as Partial<CompiledAddresses> | undefined;
  if (
    typeof stamp.profile !== 'string' ||
    typeof stamp.compiledAt !== 'string' ||
    !COMPILED_ADDRESSES.every((field) => typeof addresses?.[field] === 'string')
  ) {
    throw new Error(`${file}: not a test-engine build stamp`);
  }
  return stamp as BuildStamp;
}

/** The fields whose compiled address differs from the profile's (case-insensitive). */
export function stampMismatches(stamp: BuildStamp, contracts: ResolvedContractsSection): string[] {
  const expected = compiledAddressesOf(contracts);
  return COMPILED_ADDRESSES.filter((field) => stamp.addresses[field].toLowerCase() !== expected[field].toLowerCase());
}
