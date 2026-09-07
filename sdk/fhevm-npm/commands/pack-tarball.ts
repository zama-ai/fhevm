import { readdirSync, rmSync } from 'node:fs';
import { join } from 'node:path';

import type { NpmManifest } from '../manifest.ts';
import { selectPackTargets, tarballsOutDir } from '../base/pack-tarball.ts';
import { publishPack } from '../base/publish-pack.ts';

export type PackTarballOptions = {
  readonly workspaceRoot: string;
  readonly manifest: NpmManifest;
  readonly packageSelector?: string;
  readonly outDir?: string;
  readonly clean: boolean;
};

// Hidden alias kept for the dev owners' `pack:tarball` scripts until they move to `publish pack`
// (RELEASE_PLAN.md, step 9). Same output shape as before, but every tarball is now rendered.
export function packTarballs(options: PackTarballOptions): void {
  console.error('pack-tarball is now: fhevm-npm publish pack <payload>');
  const outDir = tarballsOutDir(options.workspaceRoot, options.manifest, options.outDir);
  if (options.clean) removeTarballs(outDir);
  for (const target of selectPackTargets(options.workspaceRoot, options.manifest, options.packageSelector)) {
    const tarball = publishPack(options.workspaceRoot, options.manifest, target.payloadKey, { outDir: options.outDir });
    console.log(`📦 ${target.payloadKey} -> ${tarball}`);
  }
}

// Only *.tgz, and only when asked: outDir may be a directory the caller named for other reasons.
function removeTarballs(outDir: string): void {
  for (const entry of readdirSync(outDir)) {
    if (entry.endsWith('.tgz')) rmSync(join(outDir, entry), { force: true });
  }
}
