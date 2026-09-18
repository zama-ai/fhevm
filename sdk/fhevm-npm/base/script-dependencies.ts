import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

import type { LoadedPackage } from './npm.ts';

export type ScriptPackageUses = ReadonlyMap<string, ReadonlySet<string>>;

const KNOWN_BIN_ALIASES: Readonly<Record<string, readonly string[]>> = {
  '@arethetypeswrong/cli': ['attw'],
  typescript: ['tsc', 'tsserver'],
};

export function rootDependencyBinaryIndex(workspaceRoot: string, root: LoadedPackage): ReadonlyMap<string, string> {
  const result = new Map<string, string>();
  for (const packageName of rootDependencyNames(root)) {
    const packageJsonFile = join(workspaceRoot, 'node_modules', packageName, 'package.json');
    const binaryNames = new Set<string>(KNOWN_BIN_ALIASES[packageName] ?? []);

    if (existsSync(packageJsonFile)) {
      for (const binaryName of readInstalledBinaryNames(packageJsonFile, packageName)) binaryNames.add(binaryName);
    } else if (!packageName.startsWith('@')) {
      // Most unscoped CLI packages expose a binary with their package name. This fallback keeps the
      // check useful before installation while avoiding false mappings such as @types/node -> node.
      binaryNames.add(packageName);
    }

    for (const binaryName of binaryNames) result.set(binaryName, packageName);
  }
  return result;
}

export function collectScriptPackageUses(
  scripts: Readonly<Record<string, string>> | undefined,
  binaryPackages: ReadonlyMap<string, string>,
): ScriptPackageUses {
  const result = new Map<string, Set<string>>();
  for (const command of Object.values(scripts ?? {})) {
    for (const [binaryName, packageName] of binaryPackages) {
      if (!commandInvokesBinary(command, binaryName)) continue;
      const binaries = result.get(packageName) ?? new Set<string>();
      binaries.add(binaryName);
      result.set(packageName, binaries);
    }
  }
  return result;
}

export function commandInvokesBinary(command: string, binaryName: string): boolean {
  const escaped = binaryName.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const commandBoundary = String.raw`(?:^|&&|\|\||[;|(\n])\s*`;
  const environmentAssignments = String.raw`(?:[A-Za-z_][A-Za-z0-9_]*=(?:"[^"]*"|'[^']*'|[^\s;&|]+)\s+)*`;
  const runner = String.raw`(?:(?:npx|npm\s+exec)(?:\s+--)?\s+)?`;
  return new RegExp(`${commandBoundary}${environmentAssignments}${runner}${escaped}(?=$|[\\s;&|)])`).test(command);
}

function rootDependencyNames(root: LoadedPackage): readonly string[] {
  return [
    ...Object.keys(root.packageJson.dependencies ?? {}),
    ...Object.keys(root.packageJson.devDependencies ?? {}),
    ...Object.keys(root.packageJson.optionalDependencies ?? {}),
  ].sort();
}

function readInstalledBinaryNames(packageJsonFile: string, packageName: string): readonly string[] {
  try {
    const value = JSON.parse(readFileSync(packageJsonFile, 'utf8')) as { readonly bin?: unknown };
    if (typeof value.bin === 'string') return [unscopedPackageName(packageName)];
    if (value.bin !== null && typeof value.bin === 'object' && !Array.isArray(value.bin)) {
      return Object.keys(value.bin as Record<string, unknown>);
    }
  } catch {
    // The dependency checker still recognizes the conventional package-name binary and known aliases.
  }
  return [];
}

function unscopedPackageName(packageName: string): string {
  return packageName.startsWith('@') ? (packageName.split('/')[1] ?? packageName) : packageName;
}
