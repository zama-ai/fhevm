import type { ResolvedProfile } from '../../../config/profiles/types.js';
import type { PreflightCheck } from '../../../model/preflight.js';
import { readBuildStamp, stampMismatches } from './build-stamp.js';

/**
 * `contracts.build` (preflight): the artifacts of `--contracts-dir` were compiled for the profile's
 * protocol addresses. No network: it compares the build stamp with the profile.
 */
export function checkBuild(profile: ResolvedProfile, contractsDir: string): PreflightCheck {
  const started = Date.now();
  const result = (status: PreflightCheck['status'], detail: string): PreflightCheck => ({
    name: 'contracts.build',
    target: contractsDir,
    status,
    durationMs: Date.now() - started,
    detail,
  });
  // Nothing to compare with; without a contracts section the profile cannot run FHE steps anyway.
  if (!profile.contracts) {
    return result('skipped', `profile '${profile.name}' has no contracts section to check the artifacts against`);
  }
  let stamp;
  try {
    stamp = readBuildStamp(contractsDir);
  } catch (error) {
    return result('failed', (error as Error).message);
  }
  const mismatches = stampMismatches(stamp, profile.contracts);
  if (mismatches.length > 0) {
    return result(
      'failed',
      `compiled for profile '${stamp.profile}' with another ${mismatches.map((field) => `contracts.${field}`).join(', ')}; recompile with 'test-engine compile --profile ${profile.name}'`,
    );
  }
  return result('passed', `compiled for profile '${stamp.profile}' at ${stamp.compiledAt}`);
}
