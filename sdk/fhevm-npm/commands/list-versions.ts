import type { NpmManifest } from '../manifest.ts';
import { loadPackages } from '../base/npm.ts';
import {
  type NpmjsCheckedEntry,
  checkNpmjs,
  formatCheckedPackageVersions,
  formatPackageVersions,
  packageVersionEntries,
} from '../base/package-versions.ts';

export type ListVersionsOptions = {
  readonly checkNpmjs: boolean;
  /** Machine-readable output: the entries as a JSON array, with the npmjs fields when they were fetched. */
  readonly json: boolean;
};

export async function listVersions(
  workspaceRoot: string,
  manifest: NpmManifest,
  options: ListVersionsOptions,
): Promise<void> {
  const entries = packageVersionEntries(loadPackages(workspaceRoot, manifest));
  const checked: readonly NpmjsCheckedEntry[] = options.checkNpmjs ? await checkNpmjs(entries) : entries;
  if (options.json) {
    console.log(JSON.stringify(checked, null, 2));
    return;
  }
  console.log(options.checkNpmjs ? formatCheckedPackageVersions(checked) : formatPackageVersions(checked));
}
