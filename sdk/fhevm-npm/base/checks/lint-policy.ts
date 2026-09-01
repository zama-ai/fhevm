import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { dirname, join, relative, resolve, sep } from 'node:path';

import type { NpmManifest } from '../../manifest.ts';
import type { Violation } from '../diagnostics.ts';
import { packageDirectory } from '../paths.ts';

const BANNED_TOOL = 'solhint';
const GENERATED_DIRECTORIES = new Set(['broadcast', 'cache', 'dependencies', 'node_modules', 'out', 'tarballs']);
const WORKSPACE_SCAN_EXCLUSIONS = new Set(['.git', 'node_modules', 'tarballs']);

export type LintPolicyInspection = {
  readonly checkedPaths: readonly string[];
  readonly violations: readonly Violation[];
};

export function inspectLintPolicy(workspaceRoot: string, manifest: NpmManifest): LintPolicyInspection {
  const mirrorOnlyRoots = mirrorOnlyPackageRoots(workspaceRoot, manifest);
  const vendoredRoots = vendoredSourceRoots(workspaceRoot, manifest);
  const checkedPaths: string[] = [];
  const violations: Violation[] = [];

  walkDirectory(workspaceRoot, (path, entry) => {
    if (isInsideAny(path, mirrorOnlyRoots)) return 'skip';
    if (entry.isDirectory()) {
      if (WORKSPACE_SCAN_EXCLUSIONS.has(entry.name)) return 'skip';
      if (isForbiddenConfigName(entry.name)) {
        violations.push(violation(workspaceRoot, path, 'forbidden Solidity-linter configuration path exists'));
      }
      return 'continue';
    }

    if (entry.name === 'package.json') {
      checkedPaths.push(pathKey(workspaceRoot, path));
      inspectTextForBannedTool(workspaceRoot, path, 'package.json contains the banned Solidity linter', violations);
    }
    if (isForbiddenConfigName(entry.name)) {
      violations.push(violation(workspaceRoot, path, 'forbidden Solidity-linter configuration file exists'));
    }
    return 'continue';
  });

  inspectInstalledBinaries(workspaceRoot, mirrorOnlyRoots, checkedPaths, violations);

  for (const projectRoot of foundryProjectRoots(workspaceRoot, manifest, mirrorOnlyRoots)) {
    walkDirectory(projectRoot, (path, entry) => {
      if (isInsideAny(path, mirrorOnlyRoots) || isInsideAny(path, vendoredRoots)) return 'skip';
      if (entry.isDirectory()) return GENERATED_DIRECTORIES.has(entry.name) ? 'skip' : 'continue';
      if (!entry.name.endsWith('.sol')) return 'continue';
      checkedPaths.push(pathKey(workspaceRoot, path));
      inspectTextForBannedTool(workspaceRoot, path, 'owned Solidity contains a banned linter directive', violations);
      return 'continue';
    });
  }

  return {
    checkedPaths: [...new Set(checkedPaths)].sort(),
    violations,
  };
}

function inspectInstalledBinaries(
  workspaceRoot: string,
  mirrorOnlyRoots: readonly string[],
  checkedPaths: string[],
  violations: Violation[],
): void {
  walkDirectory(workspaceRoot, (path, entry) => {
    if (isInsideAny(path, mirrorOnlyRoots)) return 'skip';
    if (!entry.isDirectory()) return 'continue';
    if (entry.name === '.git' || entry.name === 'tarballs') return 'skip';
    if (entry.name !== 'node_modules') return 'continue';

    const binary = join(path, '.bin', BANNED_TOOL);
    checkedPaths.push(pathKey(workspaceRoot, join(path, '.bin')));
    if (existsSync(binary)) {
      violations.push(violation(workspaceRoot, binary, 'installed banned Solidity-linter binary is runnable'));
    }
    return 'continue';
  });
}

function inspectTextForBannedTool(workspaceRoot: string, file: string, message: string, violations: Violation[]): void {
  const lines = readFileSync(file, 'utf8').split('\n');
  for (const [index, line] of lines.entries()) {
    if (!line.includes(BANNED_TOOL)) continue;
    violations.push(violation(workspaceRoot, file, `${message} '${BANNED_TOOL}' at line ${index + 1}`));
  }
}

function mirrorOnlyPackageRoots(workspaceRoot: string, manifest: NpmManifest): readonly string[] {
  const ownersByPayload = new Map<string, string>();
  for (const [key, entry] of Object.entries(manifest.packages)) {
    if (entry.kind === 'dev' && entry.publishedRelPath !== undefined) ownersByPayload.set(entry.publishedRelPath, key);
  }

  return Object.entries(manifest.packages)
    .filter(([, entry]) => {
      const channels = entry.distribution ?? ['npm'];
      return channels.includes('mirror') && !channels.includes('npm');
    })
    .map(([key]) => packageDirectory(workspaceRoot, ownersByPayload.get(key) ?? key))
    .sort();
}

function vendoredSourceRoots(workspaceRoot: string, manifest: NpmManifest): readonly string[] {
  return Object.entries(manifest.packages).flatMap(([key, entry]) => {
    const directory = packageDirectory(workspaceRoot, key);
    return (entry.vendored ?? []).map((vendored) => resolve(directory, vendored.relPath));
  });
}

function foundryProjectRoots(
  workspaceRoot: string,
  manifest: NpmManifest,
  mirrorOnlyRoots: readonly string[],
): readonly string[] {
  const roots = Object.entries(manifest.packages)
    .filter(([, entry]) => entry.kind === 'dev')
    .map(([key]) => packageDirectory(workspaceRoot, key))
    .filter((directory) => !isInsideAny(directory, mirrorOnlyRoots) && existsSync(join(directory, 'foundry.toml')))
    .sort((left, right) => left.length - right.length || left.localeCompare(right));

  return roots.filter((root, index) => !roots.slice(0, index).some((parent) => isInside(root, parent)));
}

function walkDirectory(
  directory: string,
  visit: (path: string, entry: import('node:fs').Dirent) => 'continue' | 'skip',
): void {
  let entries: import('node:fs').Dirent[];
  try {
    entries = readdirSync(directory, { withFileTypes: true });
  } catch {
    return;
  }

  for (const entry of entries) {
    const path = join(directory, entry.name);
    const action = visit(path, entry);
    if (action === 'skip' || !entry.isDirectory() || entry.isSymbolicLink()) continue;
    walkDirectory(path, visit);
  }
}

function isForbiddenConfigName(name: string): boolean {
  return name.startsWith(`.${BANNED_TOOL}`) || name.startsWith(`${BANNED_TOOL}.config.`);
}

function isInsideAny(path: string, roots: readonly string[]): boolean {
  return roots.some((root) => isInside(path, root));
}

function isInside(path: string, root: string): boolean {
  const rel = relative(root, path);
  return rel === '' || (rel !== '..' && !rel.startsWith(`..${sep}`));
}

function violation(workspaceRoot: string, file: string, message: string): Violation {
  return { rule: 'lint-policy', packageKey: pathKey(workspaceRoot, file), message };
}

function pathKey(workspaceRoot: string, path: string): string {
  const rel = relative(workspaceRoot, path).split(sep).join('/');
  return rel === '' ? '.' : `./${rel}`;
}
