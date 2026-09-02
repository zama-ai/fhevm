import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { isAbsolute, join, relative, resolve, sep } from 'node:path';
import { z } from 'zod';

import type { NpmManifest, NpmManifestEntry } from '../../manifest.ts';
import type { Violation } from '../diagnostics.ts';
import { type LoadedPackage, loadPackages } from '../npm.ts';

type VendoredEntry = NonNullable<NpmManifestEntry['vendored']>[number];
type LocalVendoredEntry = VendoredEntry & { readonly source: string };
type PinnedVendoredEntry = VendoredEntry & { readonly source: Exclude<VendoredEntry['source'], string> };

const rewriteSchema = z
  .object({
    file: z.string().min(1),
    from: z.string().min(1),
    to: z.string(),
  })
  .strict();
const destinationSchema = z
  .object({
    to: z.string().min(1),
    files: z.array(z.string().min(1)).min(1),
    rewrites: z.array(rewriteSchema).optional(),
    note: z.string().optional(),
  })
  .strict();
const commonVendoredManifestSchema = z
  .object({
    _readme: z.array(z.string()).optional(),
    source: z.string().min(1),
    destinations: z.array(destinationSchema).min(1),
  })
  .strict();
const publishedVendoredFromSchema = z
  .object({
    repository: z.string().url(),
    tag: z.string().min(1),
    commit: z.string().regex(/^[0-9a-f]{40}$/),
    from: z.string().min(1),
    to: z.string().min(1),
  })
  .strict();

export type CommonVendoredManifest = z.infer<typeof commonVendoredManifestSchema>;
export type CommonDestination = z.infer<typeof destinationSchema>;

/**
 * Where the wall clock goes, by child process.
 *
 * This check is dominated by subprocess startup, not by the comparison: it spawns `git show` and
 * `forge fmt` once per vendored file. Counting both makes that visible instead of inferable.
 */
export type SpendTotals = {
  gitMilliseconds: number;
  gitCalls: number;
  formatMilliseconds: number;
  formatCalls: number;
};

export type VendoredCheckResult = {
  readonly packageKey: string;
  readonly successes: readonly string[];
  readonly violations: readonly Violation[];
  readonly spend: SpendTotals;
  readonly milliseconds: number;
};

export function emptySpend(): SpendTotals {
  return { gitMilliseconds: 0, gitCalls: 0, formatMilliseconds: 0, formatCalls: 0 };
}

export function addSpend(into: SpendTotals, from: SpendTotals): void {
  into.gitMilliseconds += from.gitMilliseconds;
  into.gitCalls += from.gitCalls;
  into.formatMilliseconds += from.formatMilliseconds;
  into.formatCalls += from.formatCalls;
}

/** Runs `run`, adding its wall-clock cost to the named counters. */
function measure<Result>(spend: SpendTotals, kind: 'git' | 'format', run: () => Result): Result {
  const startedAt = performance.now();
  try {
    return run();
  } finally {
    const elapsed = performance.now() - startedAt;
    if (kind === 'git') {
      spend.gitMilliseconds += elapsed;
      spend.gitCalls += 1;
    } else {
      spend.formatMilliseconds += elapsed;
      spend.formatCalls += 1;
    }
  }
}

/** What a destination should hold, or why it could not be produced. Never both. */
export type ExpectedContent = { readonly content: string; readonly error?: undefined } | { readonly error: string };

/**
 * The content a destination file must hold: the source verbatim, plus any rewrites it declares.
 *
 * Shared by the checker and by `sync-vendored`, deliberately — a writer and a checker that each decide
 * this for themselves can disagree, and the disagreement looks like drift in the destination rather
 * than a bug in one of them.
 *
 * A rewrite is an exact string swap, never a regex, and a `from` that is absent is a hard failure: a
 * renamed import would otherwise leave a destination unrewritten but still compiling.
 */
export function expectedVendoredContent(sourceText: string, mapping: CommonDestination, file: string): ExpectedContent {
  let content = sourceText;
  for (const rewrite of mapping.rewrites ?? []) {
    if (rewrite.file !== file) continue;
    if (!content.includes(rewrite.from)) {
      return { error: `rewrite source ${JSON.stringify(rewrite.from)} was not found` };
    }
    content = content.split(rewrite.from).join(rewrite.to);
    if (content.includes(rewrite.from)) {
      return { error: `rewrite left ${JSON.stringify(rewrite.from)} behind` };
    }
  }
  return { content };
}

/**
 * Every package declaring vendored content, in manifest order.
 *
 * Derived rather than listed, so a package that starts vendoring is checked the day it says so — the
 * whole point of a no-argument run is that it cannot silently skip one.
 */
/** A pinned vendored destination: an external tree, at a commit, written into a published package. */
export type PinnedVendoredTarget = {
  readonly packageKey: string;
  readonly packageDirectory: string;
  readonly directory: string;
  readonly relPath: string;
  readonly source: PinnedVendoredEntry['source'];
};

/** Every pinned vendored destination in the workspace — what `sync-vendored` writes and this file checks. */
export function pinnedVendoredTargets(workspaceRoot: string, manifest: NpmManifest): readonly PinnedVendoredTarget[] {
  return loadPackages(workspaceRoot, manifest).flatMap((pkg) =>
    (pkg.inventory.vendored ?? [])
      .filter((entry): entry is PinnedVendoredEntry => typeof entry.source !== 'string')
      .map((entry) => ({
        packageKey: pkg.key,
        packageDirectory: pkg.directory,
        directory: safeResolve(pkg.directory, entry.relPath, 'vendored destination'),
        relPath: entry.relPath,
        source: entry.source,
      })),
  );
}

/** The `.sol` files the pinned tree holds at its commit, relative to `source.from`. */
export function upstreamVendoredFiles(
  repositoryRoot: string,
  source: PinnedVendoredTarget['source'],
): readonly string[] {
  execFileSync('git', ['-C', repositoryRoot, 'cat-file', '-e', `${source.commit}^{commit}`], { stdio: 'ignore' });
  return execFileSync('git', ['-C', repositoryRoot, 'ls-tree', '-r', '--name-only', source.commit, '--', source.from], {
    encoding: 'utf8',
  })
    .split('\n')
    .filter((file) => file.endsWith('.sol'))
    .map((file) => file.slice(source.from.length + 1))
    .sort();
}

/** One upstream file, normalised the way the destination stores it. */
export function upstreamVendoredContent(
  repositoryRoot: string,
  source: PinnedVendoredTarget['source'],
  file: string,
): string {
  const upstream = execFileSync('git', ['-C', repositoryRoot, 'show', `${source.commit}:${source.from}/${file}`], {
    encoding: 'utf8',
  });
  return execFileSync('forge', ['fmt', '--raw', '-'], { encoding: 'utf8', input: upstream });
}

export function vendoredPackageKeys(workspaceRoot: string, manifest: NpmManifest): readonly string[] {
  return loadPackages(workspaceRoot, manifest)
    .filter((entry) => (entry.inventory.vendored ?? []).length > 0)
    .map((entry) => entry.key);
}

export function validateVendoredPackage(
  workspaceRoot: string,
  manifest: NpmManifest,
  selector: string,
): VendoredCheckResult {
  const packages = loadPackages(workspaceRoot, manifest);
  const published = selectPublishedPackage(packages, selector);
  const entries = published.inventory.vendored;
  if (entries === undefined || entries.length === 0) {
    throw new Error(`Package '${selector}' does not declare vendored content in npm-manifest.json`);
  }

  const startedAt = performance.now();
  const repositoryRoot = gitRepositoryRoot(workspaceRoot);
  let commonManifest: CommonVendoredManifest | undefined;
  const successes: string[] = [];
  const violations: Violation[] = [];
  const spend = emptySpend();

  for (const message of validateVendoredMetadata(published.packageJson, entries)) {
    violation({ successes, violations, spend }, published.key, message);
  }

  for (const entry of entries) {
    if (typeof entry.source === 'string') {
      commonManifest ??= loadCommonVendoredManifest(workspaceRoot);
      validateLocalEntry(repositoryRoot, workspaceRoot, published, entry as LocalVendoredEntry, commonManifest, {
        successes,
        violations,
        spend,
      });
    } else {
      validatePinnedEntry(repositoryRoot, published, entry as PinnedVendoredEntry, { successes, violations, spend });
    }
  }

  return {
    packageKey: published.key,
    successes,
    violations,
    spend,
    milliseconds: performance.now() - startedAt,
  };
}

export function validateVendoredMetadata(
  packageJson: LoadedPackage['packageJson'],
  entries: readonly VendoredEntry[],
): readonly string[] {
  const pinnedEntries = entries.filter((entry): entry is PinnedVendoredEntry => typeof entry.source !== 'string');
  const fhevm = packageJson.fhevm;
  const declared =
    typeof fhevm === 'object' && fhevm !== null && 'vendoredFrom' in fhevm
      ? (fhevm as { readonly vendoredFrom?: unknown }).vendoredFrom
      : undefined;

  if (pinnedEntries.length === 0) {
    return declared === undefined
      ? []
      : ['package.json#fhevm.vendoredFrom is declared, but npm-manifest.json has no pinned vendored source'];
  }
  if (pinnedEntries.length > 1) {
    return [
      `npm-manifest.json declares ${pinnedEntries.length} pinned vendored sources, which singular package.json#fhevm.vendoredFrom cannot represent`,
    ];
  }
  if (declared === undefined) {
    return ['package.json must define fhevm.vendoredFrom for its pinned vendored source'];
  }

  const parsed = publishedVendoredFromSchema.safeParse(declared);
  if (!parsed.success) {
    return [`package.json#fhevm.vendoredFrom is invalid: ${z.prettifyError(parsed.error)}`];
  }

  const entry = pinnedEntries[0]!;
  const expected = {
    repository: entry.source.repository,
    tag: entry.source.tag,
    commit: entry.source.commit,
    from: entry.source.from,
    to: entry.relPath.slice(2),
  };
  const mismatches = Object.entries(expected)
    .filter(([field, value]) => parsed.data[field as keyof typeof expected] !== value)
    .map(
      ([field, value]) =>
        `${field}=${JSON.stringify(parsed.data[field as keyof typeof expected])} (expected ${JSON.stringify(value)})`,
    );
  return mismatches.length === 0
    ? []
    : [`package.json#fhevm.vendoredFrom differs from npm-manifest.json: ${mismatches.join(', ')}`];
}

function selectPublishedPackage(packages: readonly LoadedPackage[], selector: string): LoadedPackage {
  const normalized = selector === '.' || selector.startsWith('./') ? selector : `./${selector.replace(/^\//, '')}`;
  const direct = packages.find((pkg) => pkg.key === normalized);
  const candidates = direct === undefined ? packages.filter((pkg) => pkg.packageJson.name === selector) : [direct];
  const published = candidates.flatMap((pkg) => {
    if (pkg.inventory.kind === 'published') return [pkg];
    if (pkg.inventory.kind !== 'dev' || pkg.inventory.publishedRelPath === undefined) return [];
    const payload = packages.find((candidate) => candidate.key === pkg.inventory.publishedRelPath);
    return payload === undefined ? [] : [payload];
  });

  if (published.length === 0) {
    throw new Error(`No published package or dev owner matches '${selector}'`);
  }
  if (published.length > 1) {
    throw new Error(
      `Vendored package selector '${selector}' is ambiguous; use a package path: ${published.map((pkg) => pkg.key).join(', ')}`,
    );
  }
  return published[0]!;
}

function validatePinnedEntry(
  repositoryRoot: string,
  published: LoadedPackage,
  entry: PinnedVendoredEntry,
  output: MutableOutput,
): void {
  const violationsBefore = output.violations.length;
  const destination = safeResolve(published.directory, entry.relPath, 'vendored destination');
  if (!isDirectory(destination)) {
    violation(output, published.key, `${entry.relPath}: vendored destination directory does not exist`);
    return;
  }

  try {
    execFileSync('git', ['-C', repositoryRoot, 'cat-file', '-e', `${entry.source.commit}^{commit}`], {
      stdio: 'ignore',
    });
  } catch {
    violation(output, published.key, `${entry.relPath}: source commit ${entry.source.commit} is not available locally`);
    return;
  }

  let upstreamFiles: string[];
  try {
    upstreamFiles = execFileSync(
      'git',
      ['-C', repositoryRoot, 'ls-tree', '-r', '--name-only', entry.source.commit, '--', entry.source.from],
      { encoding: 'utf8' },
    )
      .split('\n')
      .filter((file) => file.endsWith('.sol'))
      .sort();
  } catch (error) {
    violation(output, published.key, `${entry.relPath}: unable to list upstream files: ${errorMessage(error)}`);
    return;
  }
  if (upstreamFiles.length === 0) {
    violation(output, published.key, `${entry.relPath}: source extraction contains no Solidity files`);
    return;
  }

  const vendoredFiles = entry.files === undefined ? listFiles(destination, '.sol') : [...entry.files].sort();
  if (vendoredFiles.length === 0) {
    violation(output, published.key, `${entry.relPath}: destination contains no vendored Solidity files`);
    return;
  }

  const upstreamSet = new Set(upstreamFiles);
  for (const relativeFile of vendoredFiles) {
    if (!existsSync(join(destination, relativeFile))) {
      violation(output, published.key, `${entry.relPath}/${relativeFile}: declared vendored file is missing`);
      continue;
    }
    const upstreamPath = `${entry.source.from}/${relativeFile}`;
    if (!upstreamSet.has(upstreamPath)) {
      violation(
        output,
        published.key,
        `${entry.relPath}/${relativeFile}: vendored file is absent upstream at ${entry.source.tag}`,
      );
      continue;
    }

    try {
      const upstream = measure(output.spend, 'git', () =>
        execFileSync('git', ['-C', repositoryRoot, 'show', `${entry.source.commit}:${upstreamPath}`], {
          encoding: 'utf8',
        }),
      );
      const formatted = measure(output.spend, 'format', () =>
        execFileSync('forge', ['fmt', '--raw', '-'], { encoding: 'utf8', input: upstream }),
      );
      const actual = readFileSync(join(destination, relativeFile), 'utf8');
      if (actual !== formatted) {
        violation(
          output,
          published.key,
          `${entry.relPath}/${relativeFile}: differs from forge fmt(upstream at ${entry.source.tag})`,
        );
      }
    } catch (error) {
      violation(output, published.key, `${entry.relPath}/${relativeFile}: comparison failed: ${errorMessage(error)}`);
    }
  }

  if (output.violations.length === violationsBefore) {
    output.successes.push(
      `${published.key} ${entry.relPath}: ${vendoredFiles.length} file(s) match ${entry.source.from} at ${entry.source.tag}`,
    );
  }
}

function validateLocalEntry(
  repositoryRoot: string,
  workspaceRoot: string,
  published: LoadedPackage,
  entry: LocalVendoredEntry,
  manifest: CommonVendoredManifest,
  output: MutableOutput,
): void {
  const violationsBefore = output.violations.length;
  const destination = safeResolve(published.directory, entry.relPath, 'vendored destination');
  const declaredSource = safeResolve(repositoryRoot, entry.source, 'vendored source');
  const manifestSource = safeResolve(workspaceRoot, manifest.source, 'common-vendored source');
  if (declaredSource !== manifestSource) {
    violation(
      output,
      published.key,
      `${entry.relPath}: source '${entry.source}' does not match common-vendored/manifest.json source '${manifest.source}'`,
    );
    return;
  }

  const commonDestination = manifest.destinations.find(
    (candidate) => safeResolve(workspaceRoot, candidate.to, 'common-vendored destination') === destination,
  );
  if (commonDestination === undefined) {
    violation(output, published.key, `${entry.relPath}: destination is not declared in common-vendored/manifest.json`);
    return;
  }

  const declaredFiles = [...(entry.files ?? commonDestination.files)].sort();
  const mappedFiles = [...commonDestination.files].sort();
  if (declaredFiles.join('\0') !== mappedFiles.join('\0')) {
    violation(
      output,
      published.key,
      `${entry.relPath}: npm-manifest.json files (${declaredFiles.join(', ')}) differ from common-vendored/manifest.json (${mappedFiles.join(', ')})`,
    );
    return;
  }

  for (const file of declaredFiles) {
    compareLocalFile(workspaceRoot, published, manifestSource, destination, commonDestination, file, output);
  }

  if (output.violations.length === violationsBefore) {
    output.successes.push(
      `${published.key} ${entry.relPath}: ${declaredFiles.length} file(s) match ${manifest.source}`,
    );
  }
}

function compareLocalFile(
  workspaceRoot: string,
  published: LoadedPackage,
  sourceDirectory: string,
  destination: string,
  mapping: CommonDestination,
  file: string,
  output: MutableOutput,
): void {
  const sourceFile = safeResolve(sourceDirectory, file, 'common-vendored file');
  const destinationFile = safeResolve(destination, file, 'vendored destination file');
  if (!existsSync(sourceFile)) {
    violation(output, published.key, `${workspacePath(workspaceRoot, destinationFile)}: source file is missing`);
    return;
  }
  if (!existsSync(destinationFile)) {
    violation(output, published.key, `${workspacePath(workspaceRoot, destinationFile)}: vendored file is missing`);
    return;
  }

  const expectation = expectedVendoredContent(readFileSync(sourceFile, 'utf8'), mapping, file);
  if (expectation.error !== undefined) {
    violation(output, published.key, `${workspacePath(workspaceRoot, destinationFile)}: ${expectation.error}`);
    return;
  }
  const expected = expectation.content;

  if (readFileSync(destinationFile, 'utf8') !== expected) {
    violation(
      output,
      published.key,
      `${workspacePath(workspaceRoot, destinationFile)}: differs from ${workspacePath(workspaceRoot, sourceFile)}`,
    );
  }
}

export function loadCommonVendoredManifest(workspaceRoot: string): CommonVendoredManifest {
  const file = join(workspaceRoot, 'common-vendored', 'manifest.json');
  let value: unknown;
  try {
    value = JSON.parse(readFileSync(file, 'utf8')) as unknown;
  } catch (error) {
    throw new Error(`Unable to parse ${file}: ${errorMessage(error)}`);
  }
  const result = commonVendoredManifestSchema.safeParse(value);
  if (!result.success) throw new Error(`Invalid ${file}: ${z.prettifyError(result.error)}`);
  return result.data;
}

export function gitRepositoryRoot(workspaceRoot: string): string {
  try {
    return execFileSync('git', ['-C', workspaceRoot, 'rev-parse', '--show-toplevel'], { encoding: 'utf8' }).trim();
  } catch (error) {
    throw new Error(`Unable to locate the Git repository containing ${workspaceRoot}: ${errorMessage(error)}`);
  }
}

function listFiles(root: string, extension: string, current = root): string[] {
  const result: string[] = [];
  for (const entry of readdirSync(current, { withFileTypes: true })) {
    const absolute = join(current, entry.name);
    if (entry.isDirectory()) result.push(...listFiles(root, extension, absolute));
    else if (entry.isFile() && entry.name.endsWith(extension))
      result.push(relative(root, absolute).split(sep).join('/'));
  }
  return result.sort();
}

function safeResolve(root: string, value: string, label: string): string {
  const candidate = resolve(root, value);
  const rel = relative(resolve(root), candidate);
  if (rel === '..' || rel.startsWith(`..${sep}`) || isAbsolute(rel)) {
    throw new Error(`${label} '${value}' resolves outside ${root}`);
  }
  return candidate;
}

function workspacePath(workspaceRoot: string, file: string): string {
  const rel = relative(workspaceRoot, file).split(sep).join('/');
  return rel === '' ? '.' : `./${rel}`;
}

function isDirectory(directory: string): boolean {
  try {
    return statSync(directory).isDirectory();
  } catch {
    return false;
  }
}

type MutableOutput = { readonly successes: string[]; readonly violations: Violation[]; readonly spend: SpendTotals };

function violation(output: MutableOutput, packageKey: string, message: string): void {
  output.violations.push({ rule: '5.1.3', packageKey, message });
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}
