import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { dirname, join, posix, relative, resolve, sep } from 'node:path';
import { flattenDiagnosticMessageText, parseConfigFileTextToJson } from 'typescript';

import type { NpmManifest } from '../../manifest.ts';
import type { Violation } from '../diagnostics.ts';
import { packageDirectory } from '../paths.ts';

const INTENTIONAL_MISSING = new Set([
  'node_modules',
  'artifacts',
  'cache',
  'typechain',
  'typechain-types',
  'out',
  'dependencies',
  'tarballs',
]);

const SKIP_DIRECTORIES = new Set(['node_modules', '_cjs', '_esm', '_types', '.next', 'out', 'cache', 'dependencies']);

type TsConfig = {
  readonly extends?: unknown;
  readonly include?: unknown;
  readonly exclude?: unknown;
  readonly files?: unknown;
  readonly references?: unknown;
};

type PathClaim = {
  readonly field: string;
  readonly path: string;
  readonly exists: boolean;
};

export type TsconfigPathInspection = {
  readonly checkedConfigKeys: readonly string[];
  readonly successfulClaims: readonly string[];
  readonly violations: readonly Violation[];
};

export function inspectTsconfigPaths(workspaceRoot: string, manifest: NpmManifest): TsconfigPathInspection {
  const configs = ownedTsconfigs(workspaceRoot, manifest);
  const successfulClaims: string[] = [];
  const violations: Violation[] = [];

  if (configs.length === 0) {
    violations.push({
      rule: '2.1.5',
      packageKey: '.',
      message: 'found no owned tsconfig to inspect; refusing to pass vacuously',
    });
  }

  for (const config of configs) {
    const configKey = pathKey(workspaceRoot, config);
    const parsed = parseConfigFileTextToJson(config, readFileSync(config, 'utf8'));
    if (parsed.error !== undefined) {
      violations.push({
        rule: '2.1.5',
        packageKey: configKey,
        message: `could not parse: ${flattenDiagnosticMessageText(parsed.error.messageText, '\n')}`,
      });
      continue;
    }

    for (const claim of collectClaims(dirname(config), parsed.config as TsConfig)) {
      if (claim.exists || isIntentionalMissing(claim.path)) {
        const intentional = claim.exists ? '' : ' (intentional build-output path)';
        successfulClaims.push(`${configKey} [${claim.field}] ${claim.path}${intentional}`);
        continue;
      }
      violations.push({
        rule: '2.1.5',
        packageKey: configKey,
        message: `'${claim.field}' path '${claim.path}' does not exist`,
      });
    }
  }

  return {
    checkedConfigKeys: configs.map((config) => pathKey(workspaceRoot, config)),
    successfulClaims,
    violations,
  };
}

export function collectClaims(baseDirectory: string, config: TsConfig): readonly PathClaim[] {
  const claims: PathClaim[] = [];
  for (const field of ['include', 'exclude', 'files'] as const) {
    for (const entry of stringArray(config[field])) {
      if (entry.includes('*') || entry.includes('?')) continue;
      claims.push({ field, path: entry, exists: existsSync(resolve(baseDirectory, entry)) });
    }
  }

  if (Array.isArray(config.references)) {
    for (const reference of config.references) {
      if (!isRecord(reference) || typeof reference.path !== 'string') continue;
      const target = resolve(baseDirectory, reference.path);
      claims.push({ field: 'references', path: reference.path, exists: referenceExists(target) });
    }
  }

  const extensions = typeof config.extends === 'string' ? [config.extends] : stringArray(config.extends);
  for (const entry of extensions) {
    if (!entry.startsWith('.')) continue;
    claims.push({ field: 'extends', path: entry, exists: existsSync(resolve(baseDirectory, entry)) });
  }
  return claims;
}

function ownedTsconfigs(workspaceRoot: string, manifest: NpmManifest): readonly string[] {
  const found = tsconfigsUnder(workspaceRoot, [], false);
  for (const [key, entry] of Object.entries(manifest.packages)) {
    if (key === '.' || (!entry.member && entry.kind !== 'non-package')) continue;
    const directory = packageDirectory(workspaceRoot, key);
    if (existsSync(directory)) tsconfigsUnder(directory, found, true);
  }
  return [...new Set(found)].sort();
}

function tsconfigsUnder(directory: string, found: string[], recurse: boolean): string[] {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (entry.isDirectory()) {
      if (recurse && !SKIP_DIRECTORIES.has(entry.name)) tsconfigsUnder(join(directory, entry.name), found, true);
    } else if (/^tsconfig.*\.json$/.test(entry.name)) {
      found.push(join(directory, entry.name));
    }
  }
  return found;
}

function referenceExists(target: string): boolean {
  return existsSync(target) || existsSync(`${target}.json`) || existsSync(join(target, 'tsconfig.json'));
}

function isIntentionalMissing(path: string): boolean {
  return INTENTIONAL_MISSING.has(path.replace(/^\.\//, ''));
}

function stringArray(value: unknown): readonly string[] {
  return Array.isArray(value) ? value.filter((entry): entry is string => typeof entry === 'string') : [];
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function pathKey(workspaceRoot: string, file: string): string {
  const rel = relative(workspaceRoot, file).split(sep).join(posix.sep);
  return `./${rel}`;
}
