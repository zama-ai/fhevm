// `version apply`: reconcile every derived version from sdk/versions.json. The plan is computed from the
// central file alone — the tree is consulted only to learn what differs — and it is refused unless the
// worktree is clean or carries exactly one unstaged change, the central file itself, so every derived diff
// stays attributable to the operation. Writes are the planned package.json lines and, per installation
// root, one lockfile refresh whose diff must be exactly the planned member version lines. No rollback: a
// failure leaves the diff for inspection and the next run refuses until only the central file is modified.

import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';

import type { NpmManifest } from '../manifest.ts';
import { type LoadedPackage, loadPackages } from './npm.ts';
import { type RegistryFetch, distributionChannels, npmjsStatus } from './package-versions.ts';
import { compareVersions, parseVersion } from './semver.ts';
import {
  type InstallationLock,
  centralPayloads,
  inspectVersions,
  memberEntries,
  readInstallationLocks,
} from './version-check.ts';
import { VERSIONS_FILE, type VersionsFile, loadVersions, validateVersionGraph } from './versions.ts';

/** Runs git in a directory and returns stdout; injected so tests never shell out. */
export type GitRunner = (args: readonly string[], cwd: string) => string;

export const runGit: GitRunner = (args, cwd) =>
  execFileSync('git', [...args], { cwd, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });

/** Runs npm in an installation root; injected so tests simulate the lockfile refresh instead of running it. */
export type NpmRunner = (args: readonly string[], cwd: string) => void;

export const runNpm: NpmRunner = (args, cwd) => {
  execFileSync('npm', [...args], { cwd, stdio: ['ignore', 'pipe', 'pipe'] });
};

const LOCK_REFRESH = ['install', '--package-lock-only', '--ignore-scripts', '--no-audit', '--no-fund'] as const;

export type WorktreeState =
  | { readonly kind: 'clean' }
  | { readonly kind: 'central-edit' }
  | { readonly kind: 'dirty'; readonly paths: readonly string[] };

/** One central entry that differs from HEAD: a new payload has no `from`. */
export type CentralTransition = { readonly key: string; readonly from?: string; readonly to: string };

/** One derived field to bring in line with the central version. */
export type DerivedWrite = {
  readonly path: string;
  readonly description: string;
  readonly from: string;
  readonly to: string;
};

export type ApplyPlan = {
  readonly transitions: readonly CentralTransition[];
  readonly writes: readonly DerivedWrite[];
};

/** Clean, the central file alone unstaged, or dirty with the offending repo-relative paths. Scoped to the workspace. */
export function classifyWorktree(workspaceRoot: string, git: GitRunner = runGit): WorktreeState {
  const central = `${repoPrefix(workspaceRoot, git)}${VERSIONS_FILE}`;
  const lines = git(['status', '--porcelain', '--', '.'], workspaceRoot)
    .split('\n')
    .filter((line) => line !== '');
  if (lines.length === 0) return { kind: 'clean' };
  if (lines.length === 1 && lines[0] === ` M ${central}`) return { kind: 'central-edit' };
  return { kind: 'dirty', paths: lines.map((line) => line.slice(3)) };
}

/** Every entry that differs from HEAD's central file; each existing one must strictly increase. */
export function centralDiffAgainstHead(
  workspaceRoot: string,
  versions: VersionsFile,
  git: GitRunner = runGit,
): readonly CentralTransition[] {
  const head = headVersions(workspaceRoot, git);
  return Object.entries(versions.packages).flatMap(([key, to]) => {
    const from = head[key];
    if (from === to) return [];
    if (from !== undefined && !increases(from, to)) {
      throw new Error(`${VERSIONS_FILE}: ${key} goes from ${from} to ${to}; a central version only moves forward`);
    }
    return [from === undefined ? { key, to } : { key, from, to }];
  });
}

/** The derived fields that disagree with the central file: package.json versions, then lockfile member lines. */
export function planDerivedWrites(
  workspaceRoot: string,
  packages: readonly LoadedPackage[],
  versions: VersionsFile,
  locks: readonly InstallationLock[],
): readonly DerivedWrite[] {
  return centralPayloads(packages, versions).flatMap(({ pkg, central }) => [
    ...packageWrite(workspaceRoot, pkg, central),
    ...locks.flatMap((lock) => lockWrites(workspaceRoot, lock, pkg, central)),
  ]);
}

/** The plan for the operator: what the central edit is, and what would be reconciled from it. */
export function formatPlan(plan: ApplyPlan, packages: readonly LoadedPackage[]): string {
  const name = (key: string): string => packages.find((pkg) => pkg.key === key)?.packageJson.name ?? key;
  const transitions = plan.transitions.map((t) => `  ${name(t.key)}  ${t.from ?? '(new)'} → ${t.to}`);
  const writes = plan.writes.map((w) => `  ${w.path}  ${w.description} ${w.from} → ${w.to}`);
  return [
    ...(transitions.length === 0 ? ['no central edit'] : [`central edit${plural(transitions)}`, ...transitions]),
    ...(writes.length === 0 ? ['nothing to reconcile: derived files already agree'] : ['would reconcile', ...writes]),
  ].join('\n');
}

/** Classify, validate the graph, diff against HEAD, plan. Throws rather than planning on a dirty tree. */
export function planVersionApply(workspaceRoot: string, manifest: NpmManifest, git: GitRunner = runGit): ApplyPlan {
  const state = classifyWorktree(workspaceRoot, git);
  if (state.kind === 'dirty') {
    throw new Error(
      `version apply: only ${VERSIONS_FILE} may be modified; resolve first:\n  ${state.paths.join('\n  ')}`,
    );
  }
  const versions = loadVersions(workspaceRoot);
  const graph = validateVersionGraph(manifest, versions);
  if (graph.length > 0) {
    throw new Error(
      `version apply: ${VERSIONS_FILE} is not a valid graph:\n  ${graph.map((v) => v.message).join('\n  ')}`,
    );
  }
  const packages = loadPackages(workspaceRoot, manifest);
  return {
    transitions: state.kind === 'central-edit' ? centralDiffAgainstHead(workspaceRoot, versions, git) : [],
    writes: planDerivedWrites(workspaceRoot, packages, versions, readInstallationLocks(workspaceRoot, packages)),
  };
}

/** Registry guard: a changed npm-distributed version must not already be on npmjs.com. A registry failure refuses too. */
export async function guardNpmjs(
  transitions: readonly CentralTransition[],
  packages: readonly LoadedPackage[],
  fetchRegistry: RegistryFetch,
): Promise<void> {
  for (const transition of transitions) {
    const pkg = packages.find((candidate) => candidate.key === transition.key);
    if (pkg === undefined || !distributionChannels(pkg.inventory).includes('npm')) continue;
    const name = pkg.packageJson.name ?? transition.key;
    const status = await npmjsStatus(name, transition.to, fetchRegistry);
    if (status.kind === 'published') throw new Error(`version apply: ${name}@${transition.to} is already on npmjs.com`);
    if (status.kind === 'error') throw new Error(`version apply: cannot ask npmjs.com about ${name}: ${status.detail}`);
  }
}

/** Write the plan: package.json lines, then one lockfile refresh per root, each proven to change only its planned lines. */
export function applyPlan(
  workspaceRoot: string,
  manifest: NpmManifest,
  plan: ApplyPlan,
  npm: NpmRunner = runNpm,
  git: GitRunner = runGit,
): void {
  for (const write of plan.writes.filter(isPackageWrite)) writePackageVersion(join(workspaceRoot, write.path), write);
  for (const lock of lockPaths(plan)) {
    assertUnchangedFromHead(workspaceRoot, lock, git);
    npm(LOCK_REFRESH, join(workspaceRoot, lock, '..'));
    assertLockDiffIsPlanned(workspaceRoot, lock, plan, git);
  }
  assertOnlyPlannedChanges(workspaceRoot, plan, git);
  const postflight = inspectVersions(workspaceRoot, manifest).violations;
  if (postflight.length > 0) {
    throw new Error(
      `version apply: derived files still disagree after writing:\n  ${postflight.map((v) => v.message).join('\n  ')}`,
    );
  }
}

/** Replace the one `"version": "<from>"` line by text, so the file's formatting survives untouched. */
export function writePackageVersion(file: string, write: DerivedWrite): void {
  const text = readFileSync(file, 'utf8');
  const line = new RegExp(`^(\\s*"version":\\s*")${escapeRegExp(write.from)}(")`, 'gm');
  const matches = text.match(line) ?? [];
  if (matches.length !== 1) {
    throw new Error(`${file}: expected exactly one "version": "${write.from}" line, found ${String(matches.length)}`);
  }
  writeFileSync(file, text.replace(line, `$1${write.to}$2`));
}

function isPackageWrite(write: DerivedWrite): boolean {
  return write.path.endsWith('package.json');
}

function lockPaths(plan: ApplyPlan): readonly string[] {
  return [...new Set(plan.writes.filter((write) => !isPackageWrite(write)).map((write) => write.path))];
}

// Refreshing a lock that already differs from HEAD would hide someone else's change inside this operation.
function assertUnchangedFromHead(workspaceRoot: string, lock: string, git: GitRunner): void {
  if (git(['diff', '--name-only', 'HEAD', '--', lock], workspaceRoot).trim() !== '') {
    throw new Error(`version apply: ${lock} already differs from HEAD; commit or restore it first`);
  }
}

// The lock's diff must be exactly the planned member version lines: the removed values are the `from`s,
// the added values the `to`s, and nothing else moved. Anything more is unrelated drift, which fails.
function assertLockDiffIsPlanned(workspaceRoot: string, lock: string, plan: ApplyPlan, git: GitRunner): void {
  const planned = plan.writes.filter((write) => write.path === lock);
  const changed = git(['diff', '--unified=0', '--', lock], workspaceRoot)
    .split('\n')
    .filter((line) => /^[-+][^-+]/.test(line));
  const values = (sign: string) =>
    changed
      .filter((line) => line.startsWith(sign))
      .map(versionValue)
      .sort();
  const expected = (pick: (write: DerivedWrite) => string) => planned.map(pick).sort();
  const asPlanned =
    changed.length === planned.length * 2 &&
    values('-').join(',') === expected((w) => w.from).join(',') &&
    values('+').join(',') === expected((w) => w.to).join(',');
  if (!asPlanned) {
    throw new Error(`version apply: ${lock} changed beyond the planned version lines; inspect \`git diff -- ${lock}\``);
  }
}

function versionValue(diffLine: string): string {
  return /"version":\s*"([^"]+)"/.exec(diffLine)?.[1] ?? diffLine;
}

// Everything modified in the workspace is the central file or a planned path; nothing else, nothing untracked.
function assertOnlyPlannedChanges(workspaceRoot: string, plan: ApplyPlan, git: GitRunner): void {
  const prefix = repoPrefix(workspaceRoot, git);
  const allowed = new Set([`${prefix}${VERSIONS_FILE}`, ...plan.writes.map((write) => `${prefix}${write.path}`)]);
  const stray = git(['status', '--porcelain', '--', '.'], workspaceRoot)
    .split('\n')
    .filter((line) => line !== '' && !allowed.has(line.slice(3)));
  if (stray.length > 0) throw new Error(`version apply: unplanned changes appeared:\n  ${stray.join('\n  ')}`);
}

function escapeRegExp(text: string): string {
  return text.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

// The workspace's path inside the repository ('sdk/'), which porcelain paths are relative to.
function repoPrefix(workspaceRoot: string, git: GitRunner): string {
  return git(['rev-parse', '--show-prefix'], workspaceRoot).trim();
}

function headVersions(workspaceRoot: string, git: GitRunner): Readonly<Record<string, string>> {
  const shown = git(['show', `HEAD:${repoPrefix(workspaceRoot, git)}${VERSIONS_FILE}`], workspaceRoot);
  return (JSON.parse(shown) as { packages?: Record<string, string> }).packages ?? {};
}

function increases(from: string, to: string): boolean {
  const [a, b] = [parseVersion(from), parseVersion(to)];
  return a !== undefined && b !== undefined && compareVersions(a, b) < 0;
}

function packageWrite(workspaceRoot: string, pkg: LoadedPackage, central: string): readonly DerivedWrite[] {
  const from = pkg.packageJson.version ?? '';
  if (from === central) return [];
  return [
    { path: relative(workspaceRoot, join(pkg.directory, 'package.json')), description: 'version', from, to: central },
  ];
}

function lockWrites(
  workspaceRoot: string,
  lock: InstallationLock,
  pkg: LoadedPackage,
  central: string,
): readonly DerivedWrite[] {
  return memberEntries(lock, pkg.directory)
    .filter(([, entry]) => entry.version !== central)
    .map(([path, entry]) => ({
      path: relative(workspaceRoot, join(lock.rootDirectory, 'package-lock.json')),
      description: `${path} version`,
      from: entry.version ?? '',
      to: central,
    }));
}

function plural(items: readonly unknown[]): string {
  return items.length === 1 ? '' : 's';
}
