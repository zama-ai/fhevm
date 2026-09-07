// `publish check`: is the tarball `publish pack` produced fit for npmjs.com? Statically: no `file:` spec
// survived rendering and the version is the central one (rule 5.3.10). With --check-npmjs: every rendered
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
import { parseVersion, satisfiesGenerationRange } from './semver.ts';
import { loadVersions } from './versions.ts';

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
  const central = loadVersions(workspaceRoot).packages[pkg.key];
  if (central === undefined) throw new Error(`publish check: ${pkg.key} has no central version`);
  const tarball = join(tarballsOutDir(workspaceRoot, manifest, options.outDir), tarballFilename(pkg, central));
  if (!existsSync(tarball))
    throw new Error(`publish check: ${tarball} does not exist — run \`publish pack ${pkg.key}\` first`);
  const shipped = tarballPackageJson(tarball);
  const violations = [...validateNoFileSpecs(pkg, shipped), ...validateTarballVersion(pkg, shipped, central)];
  if (options.registry === undefined || violations.length > 0) return { tarball, violations };
  return { tarball, violations: await checkRegistry(pkg, shipped, packages, options.registry) };
}

/** npm's tarball name: scope marker dropped, slash to dash, then the version. */
export function tarballFilename(pkg: LoadedPackage, version: string): string {
  const name = (pkg.packageJson.name ?? pkg.key).replace(/^@/, '').replace('/', '-');
  return `${name}-${version}.tgz`;
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
