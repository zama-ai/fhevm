import type { NpmManifest } from '../manifest.ts';
import type { CommandReport } from '../base/diagnostics.ts';
import { inspectPublishedTarball } from '../base/publish-check.ts';

export type PublishCheckOptions = {
  readonly payload: string;
  readonly outDir?: string;
  readonly checkNpmjs: boolean;
  readonly retries: number;
  readonly retryDelaySeconds: number;
};

/** `publish check <payload>`: the packed tarball carries only registry specs and the central version; with --check-npmjs, the registry agrees. */
export async function publishCheck(
  workspaceRoot: string,
  manifest: NpmManifest,
  options: PublishCheckOptions,
): Promise<CommandReport> {
  const registry = options.checkNpmjs
    ? {
        fetchRegistry: (url: string) => fetch(url, { signal: AbortSignal.timeout(15_000) }),
        retries: options.retries,
        retryDelayMs: options.retryDelaySeconds * 1000,
        sleep: (ms: number) => new Promise<void>((resolve) => setTimeout(resolve, ms)),
      }
    : undefined;
  const inspection = await inspectPublishedTarball(workspaceRoot, manifest, options.payload, {
    outDir: options.outDir,
    registry,
  });
  return {
    command: 'publish check',
    checkedPackageKeys: [inspection.tarball],
    checkedItemLabel: 'tarball(s)',
    violations: inspection.violations,
  };
}
