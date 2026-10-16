import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';

import type { NpmManifest } from '../../manifest.ts';
import type { Violation } from '../diagnostics.ts';
import { type ForgeConfigReader, forgeFmtSettings, memoizedForgeConfigReader } from '../forge-config.ts';
import { packageDirectory } from '../paths.ts';

export type FoundryInspection = {
  readonly actualVersion?: string;
  readonly fmtPackageKeys: readonly string[];
  readonly violations: readonly Violation[];
};

export type ForgeVersionReader = () => string;

export function inspectFoundry(
  workspaceRoot: string,
  manifest: NpmManifest,
  readForgeVersion: ForgeVersionReader = installedForgeVersion,
  readForgeConfig: ForgeConfigReader = memoizedForgeConfigReader(),
): FoundryInspection {
  const violations: Violation[] = [];
  const expectedVersion = manifest.foundry?.version;
  const fmtPackageKeys = Object.keys(manifest.packages)
    .sort()
    .filter((key) => existsSync(join(packageDirectory(workspaceRoot, key), 'foundry.toml')));

  for (const key of Object.keys(manifest.packages).sort()) {
    const directory = packageDirectory(workspaceRoot, key);
    const packageJsonFile = join(directory, 'package.json');
    for (const script of ['forge:fmt', 'forge:lint'] as const) {
      if (!declaresScript(packageJsonFile, script) || manifest.packages[key]?.kind !== 'published') continue;
      violations.push({
        rule: '2.1.2',
        packageKey: key,
        message: `published package must not declare '${script}'; the script and foundry.toml belong on its dev owner`,
      });
    }

    if (!declaresScript(packageJsonFile, 'forge:fmt')) continue;
    if (manifest.packages[key]?.kind === 'published') {
      continue;
    }
    if (existsSync(join(directory, 'foundry.toml'))) continue;
    violations.push({
      rule: '4.1.3',
      packageKey: key,
      message: "package declares 'forge:fmt' but has no 'foundry.toml' in the same directory",
    });
  }

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
    return { fmtPackageKeys, violations };
  }

  if (expectedVersion !== undefined && actualVersion !== expectedVersion) {
    violations.push({
      rule: '4.1.2',
      packageKey: '.',
      message: `installed forge is '${actualVersion}'; npm-manifest.json requires '${expectedVersion}' (run 'foundryup --install ${expectedVersion}')`,
    });
  }

  validateForgeFmtConfig(workspaceRoot, fmtPackageKeys, violations, readForgeConfig);

  return { actualVersion, fmtPackageKeys, violations };
}

function validateForgeFmtConfig(
  workspaceRoot: string,
  packageKeys: readonly string[],
  violations: Violation[],
  readForgeConfig: ForgeConfigReader,
): void {
  if (packageKeys.length === 0) return;

  const sharedFile = join(workspaceRoot, 'foundry.base.toml');
  if (!existsSync(sharedFile)) {
    violations.push({
      rule: '4.1.3',
      packageKey: '.',
      message: "Foundry projects require the shared 'foundry.base.toml' configuration",
    });
    return;
  }

  // The expected side is resolved by forge too, via FOUNDRY_CONFIG — not by parsing the shared file.
  // Both sides then come from the same resolver, so they cannot disagree about defaults or syntax.
  let expected: ReadonlyMap<string, string>;
  try {
    expected = forgeFmtSettings(readForgeConfig(workspaceRoot, sharedFile));
  } catch (error) {
    violations.push({
      rule: '4.1.3',
      packageKey: '.',
      message: `unable to resolve 'foundry.base.toml' through forge: ${errorMessage(error)}`,
    });
    return;
  }

  for (const key of packageKeys) {
    const directory = packageDirectory(workspaceRoot, key);
    const configFile = join(directory, 'foundry.toml');

    if (!extendsSharedFoundryConfig(configFile, sharedFile)) {
      violations.push({
        rule: '4.1.3',
        packageKey: key,
        message: "foundry.toml must set '[profile.default].extends' to the workspace 'foundry.base.toml'",
      });
    }

    let effective: ReadonlyMap<string, string>;
    try {
      effective = forgeFmtSettings(readForgeConfig(directory));
    } catch (error) {
      violations.push({
        rule: '4.1.3',
        packageKey: key,
        message: `unable to read effective Forge configuration: ${errorMessage(error)}`,
      });
      continue;
    }

    for (const [setting, expectedValue] of expected) {
      // `ignore` is per-package by nature: each project ignores its own fixture paths, so the shared
      // file deliberately does not set it and a comparison here would fail on every package.
      if (setting === 'ignore') continue;
      const actualValue = effective.get(setting);
      if (actualValue === undefined) {
        violations.push({
          rule: '4.1.3',
          packageKey: key,
          message: `effective '[fmt].${setting}' is missing; foundry.base.toml requires ${expectedValue}`,
        });
      } else if (actualValue !== expectedValue) {
        violations.push({
          rule: '4.1.3',
          packageKey: key,
          message: `effective '[fmt].${setting}' is ${actualValue}; foundry.base.toml requires ${expectedValue}`,
        });
      }
    }
  }
}

function declaresScript(packageJsonFile: string, script: string): boolean {
  if (!existsSync(packageJsonFile)) return false;
  try {
    const parsed = JSON.parse(readFileSync(packageJsonFile, 'utf8')) as { scripts?: unknown };
    const scripts = parsed.scripts;
    if (typeof scripts !== 'object' || scripts === null || Array.isArray(scripts)) return false;
    const command = (scripts as Record<string, unknown>)[script];
    return typeof command === 'string' && command.trim().length > 0;
  } catch {
    // package-json validation owns malformed manifests.
    return false;
  }
}

function extendsSharedFoundryConfig(configFile: string, sharedFile: string): boolean {
  const lines = readFileSync(configFile, 'utf8').split(/\r?\n/);
  let inDefaultProfile = false;
  for (const line of lines) {
    if (/^\s*\[/.test(line)) {
      inDefaultProfile = /^\s*\[profile\.default\]\s*(?:#.*)?$/.test(line);
      continue;
    }
    if (!inDefaultProfile) continue;
    const match = /^\s*extends\s*=\s*(["'])(.*?)\1\s*(?:#.*)?$/.exec(line);
    if (match?.[2] === undefined) continue;
    return resolve(dirname(configFile), match[2]) === resolve(sharedFile);
  }
  return false;
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
