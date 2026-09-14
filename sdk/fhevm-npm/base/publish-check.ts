// `publish check`: is the tarball `publish pack` produced fit for npmjs.com? Statically: no `file:` spec
// survived rendering, the version is the central one, every dependency on another payload carries the
// range that payload's central version renders to, and every entry point the shipped package.json
// declares is actually inside the tarball (rule 5.3.10). With --check-npmjs: every rendered
// dependency range has a satisfying version on the registry — retried, because the CI loop publishes the
// dependency seconds earlier and the registry is eventually consistent (decision 8) — and the payload's own
// version is NOT there yet, never retried, since absence is the answer wanted. Read-only throughout.

import { execFileSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join } from 'node:path';

import type { NpmManifest } from '../manifest.ts';
import type { Violation } from './diagnostics.ts';
import { type LoadedPackage, type PackageJson, dependencyDeclarations, loadPackages } from './npm.ts';
import { tarballsOutDir } from './pack-tarball.ts';
import { type RegistryFetch, npmjsPackageUrl, npmjsStatus } from './package-versions.ts';
import { isNpmDistributedPayload, resolvePayload } from './publish-render.ts';
import { generationRange, parseVersion, satisfiesGenerationRange } from './semver.ts';
import { type VersionsFile, loadVersions } from './versions.ts';

export const RULE = '5.3.10';

export type RegistryOptions = {
  readonly fetchRegistry: RegistryFetch;
  readonly retries: number;
  readonly retryDelayMs: number;
  readonly sleep: (ms: number) => Promise<void>;
};

export type PublishCheckInspection = {
  readonly tarball: string;
  readonly violations: readonly Violation[];
};

/** The whole check for one payload; `registry` undefined means static checks only. */
export async function inspectPublishedTarball(
  workspaceRoot: string,
  manifest: NpmManifest,
  selector: string,
  options: { readonly outDir?: string; readonly registry?: RegistryOptions } = {},
): Promise<PublishCheckInspection> {
  const packages = loadPackages(workspaceRoot, manifest);
  const pkg = resolvePayload(packages, selector);
  const versions = loadVersions(workspaceRoot);
  const central = versions.packages[pkg.key];
  if (central === undefined) throw new Error(`publish check: ${pkg.key} has no central version`);
  const tarball = join(tarballsOutDir(workspaceRoot, manifest, options.outDir), tarballFilename(pkg, central));
  if (!existsSync(tarball))
    throw new Error(`publish check: ${tarball} does not exist — run \`publish pack ${pkg.key}\` first`);
  const shipped = tarballPackageJson(tarball);
  const violations = [
    ...validateNoFileSpecs(pkg, shipped),
    ...validateTarballVersion(pkg, shipped, central),
    ...validateRenderedRanges(pkg, shipped, packages, versions),
    ...validateTarballEntryPoints(pkg, shipped, tarballFiles(tarball)),
  ];
  if (options.registry === undefined || violations.length > 0) return { tarball, violations };
  return { tarball, violations: await checkRegistry(pkg, shipped, packages, options.registry) };
}

/** npm's tarball name: scope marker dropped, slash to dash, then the version. */
export function tarballFilename(pkg: LoadedPackage, version: string): string {
  const name = (pkg.packageJson.name ?? pkg.key).replace(/^@/, '').replace('/', '-');
  return `${name}-${version}.tgz`;
}

/** The tarball's file list, package/ prefix stripped, read with tar so nothing is extracted to disk. */
export function tarballFiles(tarball: string): readonly string[] {
  return execFileSync('tar', ['-tf', tarball], { encoding: 'utf8' })
    .split('\n')
    .filter((entry) => entry.startsWith('package/'))
    .map((entry) => entry.slice('package/'.length))
    .filter((entry) => entry !== '' && !entry.endsWith('/'));
}

/**
 * Every path the shipped package.json points at must be in the tarball. `files` is a whitelist, so an
 * entry point built on disk but not selected leaves a package that resolves to nothing — and the daily
 * `publint`/`attw` gate runs on the payload directory, not on this artifact.
 */
export function validateTarballEntryPoints(
  pkg: LoadedPackage,
  shipped: PackageJson,
  files: readonly string[],
): readonly Violation[] {
  return declaredEntryPoints(shipped)
    .filter((declared) => !isShipped(declared, files))
    .map((declared) => violation(pkg, `package.json points at '${declared}', which the tarball does not contain`));
}

/** `main`, `module`, `types`, `typings`, `browser`, `bin` and every string leaf of `exports`. */
export function declaredEntryPoints(shipped: PackageJson): readonly string[] {
  const record = shipped as unknown as Record<string, unknown>;
  const fields = ['main', 'module', 'types', 'typings', 'browser'].map((field) => record[field]);
  const paths = [...fields, ...leafStrings(record['bin']), ...leafStrings(record['exports'])]
    .filter((value): value is string => typeof value === 'string' && value.startsWith('./'))
    .map((value) => value.slice('./'.length));
  return [...new Set(paths)].sort();
}

// A subpath pattern ships when at least one packed file matches it; a plain path must be there exactly.
function isShipped(declared: string, files: readonly string[]): boolean {
  if (!declared.includes('*')) return files.includes(declared);
  const pattern = subpathPattern(declared);
  return files.some((file) => pattern.test(file));
}

// In `exports`, `*` is not a filesystem glob: it matches any characters, separators included, so
// `./src/*` covers `src/cleartext/Arithmetic.sol`. Everything else in the pattern is literal.
function subpathPattern(declared: string): RegExp {
  const literal = declared.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  return new RegExp(`^${literal.replaceAll('\\*', '.*')}$`);
}

// `exports` nests conditions arbitrarily deep, and `bin` may be a string or a map; both bottom out in paths.
function leafStrings(value: unknown): readonly string[] {
  if (typeof value === 'string') return [value];
  if (typeof value !== 'object' || value === null) return [];
  return Object.values(value).flatMap((nested) => leafStrings(nested));
}

/** The package.json inside a tarball, read with tar so nothing is extracted to disk. */
export function tarballPackageJson(tarball: string): PackageJson {
  const text = execFileSync('tar', ['-xOf', tarball, 'package/package.json'], { encoding: 'utf8' });
  return JSON.parse(text) as PackageJson;
}

export function validateNoFileSpecs(pkg: LoadedPackage, shipped: PackageJson): readonly Violation[] {
  return dependencyDeclarations(shipped)
    .filter((declaration) => declaration.spec.startsWith('file:'))
    .map((declaration) =>
      violation(pkg, `tarball ships '${declaration.name}' as "${declaration.spec}" in ${declaration.field}; render it`),
    );
}

export function validateTarballVersion(
  pkg: LoadedPackage,
  shipped: PackageJson,
  central: string,
): readonly Violation[] {
  if (shipped.version === central) return [];
  return [violation(pkg, `tarball version is ${shipped.version ?? 'missing'}; central version is ${central}`)];
}

/**
 * Every dependency on another payload carries exactly the range that payload's central version renders
 * to (rule 4.3.4). `publish pack` produces those ranges, so this re-derives them rather than trusting
 * the artifact: a hand-edited or stale range is what an independent gate exists to catch.
 */
export function validateRenderedRanges(
  pkg: LoadedPackage,
  shipped: PackageJson,
  packages: readonly LoadedPackage[],
  versions: VersionsFile,
): readonly Violation[] {
  return renderedDependencies(shipped, packages).flatMap((declaration) => {
    // A surviving `file:` spec is one defect, reported once, by validateNoFileSpecs.
    if (declaration.spec.startsWith('file:')) return [];
    const expected = expectedRange(declaration.name, packages, versions);
    if (expected === undefined || declaration.spec === expected) return [];
    return [
      violation(
        pkg,
        `tarball asks for '${declaration.name}' as "${declaration.spec}"; its central version renders to "${expected}"`,
      ),
    ];
  });
}

// The payload that publishes under this name, then the range its central version renders to. A name
// shared by two generations (host-contracts-cleartext) resolves to whichever generation is the member.
function expectedRange(name: string, packages: readonly LoadedPackage[], versions: VersionsFile): string | undefined {
  const candidates = packages.filter((pkg) => isNpmDistributedPayload(pkg) && pkg.packageJson.name === name);
  const target = candidates.find((pkg) => pkg.inventory.member) ?? candidates[0];
  const central = target === undefined ? undefined : versions.packages[target.key];
  const parsed = central === undefined ? undefined : parseVersion(central);
  return parsed === undefined ? undefined : generationRange(parsed);
}

/** Rendered dependencies must be satisfiable on npmjs.com (retried); the payload's own version must be absent. */
export async function checkRegistry(
  pkg: LoadedPackage,
  shipped: PackageJson,
  packages: readonly LoadedPackage[],
  registry: RegistryOptions,
): Promise<readonly Violation[]> {
  const violations: Violation[] = [];
  for (const declaration of renderedDependencies(shipped, packages)) {
    const satisfied = await withRetry(registry, () =>
      rangeSatisfied(declaration.name, declaration.spec, registry.fetchRegistry),
    );
    if (!satisfied)
      violations.push(violation(pkg, `npmjs.com has no version of ${declaration.name} satisfying ${declaration.spec}`));
  }
  const name = shipped.name ?? pkg.key;
  const own = await npmjsStatus(name, shipped.version ?? '', registry.fetchRegistry);
  if (own.kind === 'published')
    violations.push(violation(pkg, `${name}@${shipped.version ?? ''} is already on npmjs.com`));
  if (own.kind === 'error') violations.push(violation(pkg, `cannot ask npmjs.com about ${name}: ${own.detail}`));
  return violations;
}

// Only the dependencies rendering produced — those naming another npm-distributed payload — are ours to prove.
function renderedDependencies(shipped: PackageJson, packages: readonly LoadedPackage[]) {
  const payloadNames = new Set(packages.filter(isNpmDistributedPayload).map((candidate) => candidate.packageJson.name));
  return dependencyDeclarations(shipped).filter((declaration) => payloadNames.has(declaration.name));
}

async function rangeSatisfied(name: string, range: string, fetchRegistry: RegistryFetch): Promise<boolean> {
  const response = await fetchRegistry(npmjsPackageUrl(name));
  if (response.status !== 200) return false;
  const versions = (await response.json()) as { versions?: Record<string, unknown> };
  return Object.keys(versions.versions ?? {}).some((published) => {
    const parsed = parseVersion(published);
    return parsed !== undefined && satisfiesGenerationRange(parsed, range);
  });
}

// True as soon as the probe says so; otherwise wait and ask again, `retries` more times.
async function withRetry(registry: RegistryOptions, probe: () => Promise<boolean>): Promise<boolean> {
  for (let attempt = 0; ; attempt += 1) {
    if (await probe()) return true;
    if (attempt >= registry.retries) return false;
    await registry.sleep(registry.retryDelayMs);
  }
}

function violation(pkg: LoadedPackage, message: string): Violation {
  return { rule: RULE, packageKey: pkg.key, message };
}
