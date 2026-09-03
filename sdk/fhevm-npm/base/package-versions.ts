// The version every distributed payload carries, with how it is distributed: to npm, to a mirror
// repository, or both. Read off the manifest's `published` entries and their package.json files.

import type { NpmManifestEntry } from '../manifest.ts';
import type { LoadedPackage } from './npm.ts';

type DistributionChannel = 'npm' | 'mirror';

export type PackageVersionEntry = {
  readonly key: string;
  readonly name: string;
  readonly version: string;
  readonly channels: readonly DistributionChannel[];
  /** The local payload's package.json `repository` URL, as written. */
  readonly repository?: string;
  readonly mirrorRepository?: string;
};

// A payload with no `distribution` field is npm-published; a `mirror` block adds the mirror channel.
export function distributionChannels(entry: NpmManifestEntry): readonly DistributionChannel[] {
  if (entry.distribution !== undefined) return entry.distribution;
  return entry.mirror === undefined ? ['npm'] : ['npm', 'mirror'];
}

export function packageVersionEntries(packages: readonly LoadedPackage[]): readonly PackageVersionEntry[] {
  return packages
    .flatMap((pkg): PackageVersionEntry[] => {
      const entry = pkg.inventory;
      if (entry.kind !== 'published') return [];
      const mirrorRepository = entry.mirror?.repository;
      const repository = repositoryUrl(pkg.packageJson.repository);
      return [
        {
          key: pkg.key,
          ...(repository === undefined ? {} : { repository }),
          name: pkg.packageJson.name ?? entry.name ?? pkg.key,
          version: pkg.packageJson.version ?? '(no version)',
          channels: distributionChannels(entry),
          ...(mirrorRepository === undefined ? {} : { mirrorRepository }),
        },
      ];
    })
    .sort((left, right) => left.key.localeCompare(right.key));
}

export function formatPackageVersions(entries: readonly PackageVersionEntry[]): string {
  const rows = entries.map((entry) => [
    entry.key,
    entry.name,
    entry.version,
    entry.channels.join('+'),
    entry.mirrorRepository ?? '',
  ]);
  return formatTable(['package', 'name', 'version', 'distribution', 'mirror'], rows);
}

function formatTable(header: readonly string[], rows: readonly (readonly string[])[]): string {
  const widths = header.map((title, i) => Math.max(title.length, ...rows.map((row) => row[i]?.length ?? 0)));
  const line = (row: readonly string[]): string =>
    row
      .map((cell, i) => cell.padEnd(widths[i] ?? cell.length))
      .join('  ')
      .trimEnd();
  return [line(header), ...rows.map(line)].join('\n');
}

////////////////////////////////////////////////////////////////////////////////
// --check-npmjs: is the listed version on the public registry?
////////////////////////////////////////////////////////////////////////////////

/** What npm recorded about one publication: the commit it was published from and the repository it declared. */
export type NpmjsPublication = {
  /** `gitHead`, present when `npm publish` ran from a git checkout. */
  readonly gitHead?: string;
  /** The published package.json's repository URL, normalized (no `git+`, no `.git`). */
  readonly repository?: string;
  readonly publishedAt?: string;
};

export type NpmjsStatus =
  /** The listed version is on npmjs; `latest` is the registry's dist-tag. */
  | ({ readonly kind: 'published'; readonly latest: string } & NpmjsPublication)
  /** The package exists on npmjs but not at this version — what an upcoming release looks like. */
  | { readonly kind: 'unpublished'; readonly latest: string }
  /** npmjs has never seen this package name. */
  | { readonly kind: 'unknown-package' }
  | { readonly kind: 'error'; readonly detail: string };

export type NpmjsCheckedEntry = PackageVersionEntry & {
  readonly npmjs?: NpmjsStatus;
  /** The registry's repository differs from the local payload's `repository` field. */
  readonly repositoryMismatch?: boolean;
};

/** What the check needs from the registry: the status code and the JSON body of `GET /<name>`. */
export type RegistryFetch = (
  url: string,
) => Promise<{ readonly status: number; readonly json: () => Promise<unknown> }>;

const NPMJS_REGISTRY = 'https://registry.npmjs.org';

export function npmjsPackageUrl(name: string): string {
  return `${NPMJS_REGISTRY}/${encodeURIComponent(name)}`;
}

function field(holder: unknown, key: string): unknown {
  return typeof holder === 'object' && holder !== null && key in holder
    ? (holder as Record<string, unknown>)[key]
    : undefined;
}

/** `git+https://github.com/x/y.git` and `https://github.com/x/y` are the same repository. */
export function normalizeRepositoryUrl(url: string | undefined): string | undefined {
  if (url === undefined) return undefined;
  return url
    .replace(/^git\+/, '')
    .replace(/^git:\/\//, 'https://')
    .replace(/\.git$/, '')
    .replace(/\/$/, '');
}

function registryVersions(
  body: unknown,
  version: string,
):
  | { readonly versions: readonly string[]; readonly latest: string; readonly publication: NpmjsPublication }
  | undefined {
  const versions = field(body, 'versions');
  if (typeof versions !== 'object' || versions === null) return undefined;
  const latest = field(field(body, 'dist-tags'), 'latest');
  const meta = field(versions, version);
  const gitHead = field(meta, 'gitHead');
  const repositoryField = field(meta, 'repository');
  const repository = typeof repositoryField === 'string' ? repositoryField : field(repositoryField, 'url');
  const publishedAt = field(field(body, 'time'), version);
  return {
    versions: Object.keys(versions),
    latest: typeof latest === 'string' ? latest : '?',
    publication: {
      ...(typeof gitHead === 'string' ? { gitHead } : {}),
      ...(typeof repository === 'string' ? { repository: normalizeRepositoryUrl(repository) } : {}),
      ...(typeof publishedAt === 'string' ? { publishedAt } : {}),
    },
  };
}

export async function npmjsStatus(name: string, version: string, fetchRegistry: RegistryFetch): Promise<NpmjsStatus> {
  try {
    const response = await fetchRegistry(npmjsPackageUrl(name));
    if (response.status === 404) return { kind: 'unknown-package' };
    if (response.status !== 200) return { kind: 'error', detail: `HTTP ${String(response.status)}` };
    const registry = registryVersions(await response.json(), version);
    if (registry === undefined) return { kind: 'error', detail: 'unexpected registry payload' };
    return registry.versions.includes(version)
      ? { kind: 'published', latest: registry.latest, ...registry.publication }
      : { kind: 'unpublished', latest: registry.latest };
  } catch (error) {
    return { kind: 'error', detail: error instanceof Error ? error.message : String(error) };
  }
}

/** Checks every npm-distributed entry against npmjs; mirror-only payloads are left as they are. */
export async function checkNpmjs(
  entries: readonly PackageVersionEntry[],
  fetchRegistry: RegistryFetch = (url) => fetch(url, { signal: AbortSignal.timeout(15_000) }),
): Promise<readonly NpmjsCheckedEntry[]> {
  return Promise.all(
    entries.map(async (entry): Promise<NpmjsCheckedEntry> => {
      if (!entry.channels.includes('npm')) return entry;
      const npmjs = await npmjsStatus(entry.name, entry.version, fetchRegistry);
      const registryRepository = npmjs.kind === 'published' ? npmjs.repository : undefined;
      const mismatch =
        registryRepository !== undefined &&
        entry.repository !== undefined &&
        registryRepository !== normalizeRepositoryUrl(entry.repository);
      return { ...entry, npmjs, ...(mismatch ? { repositoryMismatch: true } : {}) };
    }),
  );
}

export function formatNpmjsStatus(status: NpmjsStatus | undefined): string {
  if (status === undefined) return '-';
  switch (status.kind) {
    case 'published':
      return `published (latest ${status.latest})`;
    case 'unpublished':
      return `NOT published (latest ${status.latest})`;
    case 'unknown-package':
      return 'NOT on npmjs (unknown package)';
    case 'error':
      return `error: ${status.detail}`;
  }
}

export function formatCheckedPackageVersions(entries: readonly NpmjsCheckedEntry[]): string {
  const rows = entries.map((entry) => {
    const publication = entry.npmjs?.kind === 'published' ? entry.npmjs : undefined;
    return [
      entry.key,
      entry.name,
      entry.version,
      entry.channels.join('+'),
      formatNpmjsStatus(entry.npmjs),
      publication?.publishedAt?.slice(0, 10) ?? '',
      `${publication?.repository ?? ''}${entry.repositoryMismatch === true ? ' (!= local)' : ''}`,
      publication?.gitHead ?? '',
      entry.mirrorRepository ?? '',
    ];
  });
  return formatTable(
    ['package', 'name', 'version', 'distribution', 'npmjs', 'published', 'npmjs repository', 'gitHead', 'mirror'],
    rows,
  );
}

// package.json allows `repository` as a string or as `{ type, url }`.
function repositoryUrl(repository: unknown): string | undefined {
  if (typeof repository === 'string') return repository;
  const url = field(repository, 'url');
  return typeof url === 'string' ? url : undefined;
}
