import type { NpmManifest } from '../manifest.ts';

export type PackageListEntry = {
  readonly path: string;
  readonly kind: NpmManifest['packages'][string]['kind'];
};

export function packageListEntries(manifest: NpmManifest): readonly PackageListEntry[] {
  return Object.entries(manifest.packages)
    .map(([path, entry]) => ({ path, kind: entry.kind }))
    .sort((left, right) => left.path.localeCompare(right.path));
}

export function printPackageList(manifest: NpmManifest): void {
  const entries = packageListEntries(manifest);
  const pathWidth = Math.max(...entries.map((entry) => entry.path.length));
  for (const entry of entries) console.log(`${entry.path.padEnd(pathWidth)}  ${entry.kind}`);
}
