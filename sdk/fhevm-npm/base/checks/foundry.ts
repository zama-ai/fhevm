import { execFileSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join } from 'node:path';

import type { NpmManifest } from '../../manifest.ts';
import type { Violation } from '../diagnostics.ts';
import { packageDirectory } from '../paths.ts';

export type FoundryInspection = {
  readonly actualVersion?: string;
  readonly violations: readonly Violation[];
};

export type ForgeVersionReader = () => string;

export function inspectFoundry(
  workspaceRoot: string,
  manifest: NpmManifest,
  readForgeVersion: ForgeVersionReader = installedForgeVersion,
): FoundryInspection {
  const violations: Violation[] = [];
  const expectedVersion = manifest.foundry?.version;

  if (expectedVersion === undefined) {
    violations.push({
      rule: '4.1.2',
      packageKey: '.',
      message: "npm-manifest.json must declare the repository Foundry version in 'foundry.version'",
    });
  }

  for (const key of Object.keys(manifest.packages).sort()) {
    const pinFile = join(packageDirectory(workspaceRoot, key), '.foundry-version');
    if (!existsSync(pinFile)) continue;
    violations.push({
      rule: '4.1.2',
      packageKey: key,
      message: "remove '.foundry-version'; the central pin is npm-manifest.json#foundry.version",
    });
  }

  let actualVersion: string | undefined;
  try {
    actualVersion = parseForgeVersion(readForgeVersion());
  } catch (error) {
    violations.push({
      rule: '4.1.2',
      packageKey: '.',
      message: `unable to read the installed forge version${expectedVersion === undefined ? '' : ` (expected '${expectedVersion}')`}: ${errorMessage(error)}`,
    });
    return { violations };
  }

  if (expectedVersion !== undefined && actualVersion !== expectedVersion) {
    violations.push({
      rule: '4.1.2',
      packageKey: '.',
      message: `installed forge is '${actualVersion}'; npm-manifest.json requires '${expectedVersion}' (run 'foundryup --install ${expectedVersion}')`,
    });
  }

  return { actualVersion, violations };
}

export function parseForgeVersion(output: string): string {
  const match = /^forge Version:\s*(\S+)\s*$/m.exec(output);
  if (match?.[1] === undefined) throw new Error("unexpected output from 'forge --version'");
  return match[1];
}

function installedForgeVersion(): string {
  return execFileSync('forge', ['--version'], { encoding: 'utf8' });
}

function errorMessage(error: unknown): string {
  if (error instanceof Error && 'code' in error && error.code === 'ENOENT')
    return "'forge' is not installed or not on PATH";
  return error instanceof Error ? error.message : String(error);
}
