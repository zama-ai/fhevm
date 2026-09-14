import type { NpmManifest } from '../manifest.ts';
import { loadPackages } from '../base/npm.ts';
import { applyPlan, formatPlan, guardNpmjs, planVersionApply } from '../base/version-apply.ts';

export type VersionApplyOptions = { readonly dryRun: boolean; readonly checkNpmjs: boolean };

// `version apply`: plan from sdk/versions.json, optionally prove the new versions are not on npmjs.com,
// then reconcile the derived files unless --dry-run. Writes nothing when there is nothing to reconcile.
export async function versionApply(
  workspaceRoot: string,
  manifest: NpmManifest,
  options: VersionApplyOptions,
): Promise<void> {
  const plan = planVersionApply(workspaceRoot, manifest);
  const packages = loadPackages(workspaceRoot, manifest);
  console.log(formatPlan(plan, packages));
  if (options.checkNpmjs) await guardNpmjs(plan.transitions, packages, registryFetch);
  if (options.dryRun || plan.writes.length === 0) return;
  applyPlan(workspaceRoot, manifest, plan);
  console.log(
    `✅ ${String(plan.transitions.length)} central edit(s); ${String(plan.writes.length)} derived write(s) ` +
      'reconciled; version check passed.',
  );
}

const registryFetch = (url: string) => fetch(url, { signal: AbortSignal.timeout(15_000) });
