import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, relative, sep } from 'node:path';

import type { NpmManifest } from '../../manifest.ts';
import type { Violation } from '../diagnostics.ts';
import { hardhatTemplateV2PackageKey, patchHardhatTemplateV2Manifest } from '../mirrors/hardhat-template-v2.ts';
import { loadPackages } from '../npm.ts';
import { gitRepositoryRoot } from '../repository.ts';

export type MirrorValidation = {
  readonly packageKey: string;
  readonly repository: string;
  readonly comparedFiles: number;
  readonly violations: readonly Violation[];
};

export function validateMirror(
  workspaceRoot: string,
  manifest: NpmManifest,
  selector: string,
): MirrorValidation {
  const pkg = selectMirrorPackage(workspaceRoot, manifest, selector);
  const repository = pkg.inventory.mirror!.repository;
  if (pkg.key !== hardhatTemplateV2PackageKey) {
    throw new Error(`${pkg.key}: no package-specific mirror comparison is implemented`);
  }

  const temporaryRoot = mkdtempSync(join(tmpdir(), 'fhevm-npm-mirror-'));
  const upstreamDirectory = join(temporaryRoot, 'upstream');
  try {
    execFileSync('git', ['clone', '--depth', '1', '--quiet', repository, upstreamDirectory], { stdio: 'inherit' });
    return compareHardhatTemplate(workspaceRoot, pkg.directory, upstreamDirectory, pkg.key, repository);
  } finally {
    rmSync(temporaryRoot, { recursive: true, force: true });
  }
}

function compareHardhatTemplate(
  workspaceRoot: string,
  localDirectory: string,
  upstreamDirectory: string,
  packageKey: string,
  repository: string,
): MirrorValidation {
  const ignored = (path: string): boolean => path === 'package-lock.json' || path.startsWith('.github/');
  const upstreamFiles = gitFiles(upstreamDirectory).filter((path) => !ignored(path));
  const repositoryRoot = gitRepositoryRoot(workspaceRoot);
  const localPrefix = `${relative(repositoryRoot, localDirectory).split(sep).join('/')}/`;
  const localFiles = execFileSync(
    'git',
    ['-C', repositoryRoot, 'ls-files', '--cached', '--others', '--exclude-standard', '--', localPrefix],
    { encoding: 'utf8' },
  )
    .split('\n')
    .filter(Boolean)
    .map((path) => path.slice(localPrefix.length))
    .filter((path) => !ignored(path))
    .sort();

  const violations: Violation[] = [];
  for (const path of upstreamFiles.filter((path) => !localFiles.includes(path))) {
    violations.push({ rule: '5.1.3', packageKey, message: `mirror is missing upstream file '${path}'` });
  }
  for (const path of localFiles.filter((path) => !upstreamFiles.includes(path))) {
    violations.push({ rule: '5.1.3', packageKey, message: `mirror has unexpected file '${path}'` });
  }

  for (const path of upstreamFiles.filter((path) => localFiles.includes(path))) {
    if (path === 'package.json') {
      const upstreamManifest = JSON.parse(readFileSync(join(upstreamDirectory, path), 'utf8')) as Record<string, unknown>;
      const expected = `${JSON.stringify(patchHardhatTemplateV2Manifest(upstreamManifest), null, 2)}\n`;
      if (readFileSync(join(localDirectory, path), 'utf8') !== expected) {
        violations.push({ rule: '5.1.3', packageKey, message: `mirror file '${path}' differs from its expected workspace transformation` });
      }
      continue;
    }
    const local = readFileSync(join(localDirectory, path));
    const upstream = readFileSync(join(upstreamDirectory, path));
    if (!local.equals(upstream)) {
      violations.push({ rule: '5.1.3', packageKey, message: `mirror file '${path}' differs from upstream` });
    }
  }

  return { packageKey, repository, comparedFiles: upstreamFiles.length, violations };
}

function selectMirrorPackage(workspaceRoot: string, manifest: NpmManifest, selector: string) {
  const packages = loadPackages(workspaceRoot, manifest).filter((pkg) => pkg.inventory.mirror !== undefined);
  const normalized = selector === '.' || selector.startsWith('./') ? selector : `./${selector.replace(/^\//, '')}`;
  const matches = packages.filter((pkg) => pkg.key === normalized || pkg.packageJson.name === selector);
  if (matches.length === 0) throw new Error(`No manifest mirror matches '${selector}'`);
  if (matches.length > 1) {
    throw new Error(`Mirror selector '${selector}' is ambiguous; use a package path: ${matches.map((pkg) => pkg.key).join(', ')}`);
  }
  return matches[0]!;
}

function gitFiles(directory: string): readonly string[] {
  return execFileSync('git', ['-C', directory, 'ls-files'], { encoding: 'utf8' })
    .split('\n')
    .filter(Boolean)
    .sort();
}
