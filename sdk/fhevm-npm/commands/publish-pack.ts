import type { NpmManifest } from '../manifest.ts';
import { publishPack as pack } from '../base/publish-pack.ts';

export type PublishPackOptions = { readonly payload: string; readonly outDir?: string };

/** `publish pack <payload>`: render, stage, `npm pack`; prints the tarball path alone so a shell can capture it. */
export function publishPack(workspaceRoot: string, manifest: NpmManifest, options: PublishPackOptions): void {
  console.log(pack(workspaceRoot, manifest, options.payload, { outDir: options.outDir }));
}
