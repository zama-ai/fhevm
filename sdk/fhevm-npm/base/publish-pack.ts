// `publish pack`: the tarball npmjs.com receives. The payload is copied to a scratch directory without
// node_modules, the rendered package.json from publish-render.ts is written into the copy, and `npm pack`
// runs there — so the tarball cannot differ from what `publish render` displayed, and the tree is never
// touched. The tarball lands in the manifest's tarballs directory; its path is the command's only output.

import { cpSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { basename, join } from 'node:path';

import type { NpmManifest } from '../manifest.ts';
import { loadPackages } from './npm.ts';
import { packOne, tarballsOutDir } from './pack-tarball.ts';
import { renderPackageJson, resolvePayload } from './publish-render.ts';
import { loadVersions } from './versions.ts';

/** Packs a staged directory into outDir and returns the tarball path; injected so tests never run npm. */
export type Packer = (stagedDirectory: string, outDir: string) => string;

export type PublishPackOptions = {
  readonly outDir?: string;
  readonly pack?: Packer;
};

/** Render, stage, pack. Returns the tarball's absolute path. */
export function publishPack(
  workspaceRoot: string,
  manifest: NpmManifest,
  selector: string,
  options: PublishPackOptions = {},
): string {
  const packages = loadPackages(workspaceRoot, manifest);
  const pkg = resolvePayload(packages, selector);
  const rendered = renderPackageJson(pkg, packages, loadVersions(workspaceRoot));
  const outDir = tarballsOutDir(workspaceRoot, manifest, options.outDir);
  mkdirSync(outDir, { recursive: true });
  const staging = stagePayload(pkg.directory);
  try {
    writeFileSync(join(staging, 'package.json'), `${JSON.stringify(rendered.packageJson, null, 2)}\n`);
    return (options.pack ?? packOne)(staging, outDir);
  } finally {
    rmSync(staging, { recursive: true, force: true });
  }
}

/** A copy of the payload directory in the OS temp dir, minus every node_modules at any depth. */
export function stagePayload(payloadDirectory: string): string {
  const staging = mkdtempSync(join(tmpdir(), 'fhevm-npm-publish-pack-'));
  cpSync(payloadDirectory, staging, { recursive: true, filter: (source) => basename(source) !== 'node_modules' });
  return staging;
}
