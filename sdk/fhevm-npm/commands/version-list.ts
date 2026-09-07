import type { NpmManifest } from '../manifest.ts';
import { loadPackages } from '../base/npm.ts';
import {
  type NpmjsCheckedEntry,
  checkNpmjs,
  formatCheckedPackageVersions,
  formatPackageVersions,
  packageVersionEntries,
  withCentralVersions,
} from '../base/package-versions.ts';
import { loadVersions } from '../base/versions.ts';

export type VersionListOptions = {
  readonly checkNpmjs: boolean;
  /** Machine-readable output: the entries as a JSON array, with the npmjs fields when they were fetched. */
  readonly json: boolean;
};

// `version list`: every published payload with its central version (sdk/versions.json) beside the
// derived package.json one, its channels, and with --check-npmjs what the registry holds.
export async function versionList(
  workspaceRoot: string,
  manifest: NpmManifest,
  options: VersionListOptions,
): Promise<void> {
  const entries = withCentralVersions(
    packageVersionEntries(loadPackages(workspaceRoot, manifest)),
    loadVersions(workspaceRoot),
  );
  const checked: readonly NpmjsCheckedEntry[] = options.checkNpmjs ? await checkNpmjs(entries) : entries;
  if (options.json) {
    console.log(JSON.stringify(checked, null, 2));
    return;
  }
  console.log(options.checkNpmjs ? formatCheckedPackageVersions(checked) : formatPackageVersions(checked));
}
