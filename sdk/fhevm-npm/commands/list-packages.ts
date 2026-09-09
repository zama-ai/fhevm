import { printPackageList } from '../base/package-list.ts';
import type { NpmManifest } from '../manifest.ts';

export function listPackages(manifest: NpmManifest): void {
  printPackageList(manifest);
}
