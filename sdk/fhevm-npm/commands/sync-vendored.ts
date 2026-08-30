import type { CommandReport } from '../base/diagnostics.ts';
import { syncPinnedVendored, syncVendored } from '../base/sync-vendored.ts';
import type { NpmManifest } from '../manifest.ts';

/**
 * Writes both kinds of vendored content, or compares them with `--check`.
 *
 * Two sources, one command: the shared TypeScript comes from `common-vendored/manifest.json`, the
 * pinned Solidity from `npm-manifest.json` at a declared commit. They were separate before — the
 * TypeScript had a writer and the Solidity had only a checker — which is what made a tag bump a manual
 * edit that `check-vendored` graded afterwards.
 */
export function syncVendoredCommand(options: {
  readonly workspaceRoot: string;
  readonly manifest: NpmManifest;
  readonly check: boolean;
  readonly verbose: boolean;
}): CommandReport {
  // Progress only in verbose mode: this walks 40 files across five destinations and two of them cost
  // most of the second, so a quiet run should say nothing until its report.
  const onProgress = options.verbose
    ? (message: string): void => {
        console.log(message);
      }
    : undefined;
  const withProgress = { ...options, onProgress };

  if (options.verbose) {
    console.log(
      options.check ? '🔎 sync-vendored: comparing every destination' : '📄 sync-vendored: writing every destination',
    );
  }

  const local = syncVendored(withProgress);
  const pinned = syncPinnedVendored(withProgress);
  const written = [...local.written, ...pinned.written];
  const inspected = local.inspected + pinned.inspected;

  return {
    command: options.check ? 'sync-vendored --check' : 'sync-vendored',
    checkedPackageKeys: [...local.destinations, ...pinned.destinations],
    checkedItemLabel: 'destination(s)',
    verboseSuccesses: [
      ...written.map((path) => (path.startsWith('removed ') ? path : `wrote ${path}`)),
      `${String(inspected)} vendored file(s) ${options.check ? 'compared' : 'in sync'}`,
    ],
    violations: [...local.violations, ...pinned.violations],
  };
}
