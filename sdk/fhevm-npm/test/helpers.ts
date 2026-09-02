import { consumerModuleKinds } from '../base/module-kind.ts';
import type { LoadedPackage, PackageJson } from '../base/npm.ts';
import { type NpmManifest, type NpmManifestEntry, parseNpmManifest } from '../manifest.ts';

type TestManifestEntry = Omit<NpmManifestEntry, 'type' | 'browser'> &
  Partial<Pick<NpmManifestEntry, 'type' | 'browser'>>;

export function loadedPackage(key: string, inventory: TestManifestEntry, packageJson: PackageJson): LoadedPackage {
  const moduleKinds = consumerModuleKinds(packageJson);
  return {
    key,
    directory: `/workspace/${key === '.' ? '' : key.slice(2)}`,
    inventory: {
      type: moduleKinds.length === 2 ? 'dual' : (moduleKinds[0] ?? 'cjs'),
      browser: false,
      ...inventory,
    },
    packageJson,
  };
}

export function parseTestNpmManifest(
  value: Record<string, unknown> & { readonly packages: Readonly<Record<string, Record<string, unknown>>> },
): NpmManifest {
  return parseNpmManifest({
    ...value,
    packages: Object.fromEntries(
      Object.entries(value.packages).map(([key, entry]) => [key, { type: 'esm', browser: false, ...entry }]),
    ),
  });
}
