import type { NpmManifest } from '../manifest.ts';
import { formatPackedFiles, formatRenderedDiff, publishRender as render } from '../base/publish-render.ts';

export type PublishRenderOptions = { readonly payload: string; readonly json: boolean };

/** `publish render <payload>`: the package.json npmjs.com will see, as a diff (or in full with --json), then the packed files. */
export function publishRender(workspaceRoot: string, manifest: NpmManifest, options: PublishRenderOptions): void {
  const { pkg, rendered, files } = render(workspaceRoot, manifest, options.payload);
  console.log(options.json ? JSON.stringify(rendered.packageJson, null, 2) : formatRenderedDiff(pkg, rendered));
  console.log(formatPackedFiles(files));
}
