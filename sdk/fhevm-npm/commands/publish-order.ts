import type { NpmManifest } from '../manifest.ts';
import { publishOrder as order } from '../base/publish-render.ts';

/** `publish order`: the npm-distributed payloads, dependencies first, one manifest key per line. */
export function publishOrder(workspaceRoot: string, manifest: NpmManifest): void {
  console.log(order(workspaceRoot, manifest).join('\n'));
}
