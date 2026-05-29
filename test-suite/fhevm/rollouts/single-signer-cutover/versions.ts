// The single-signer-cutover runbook reuses the exact v0.12.5 -> v0.13.0-6 version
// matrix and the two-of-three coprocessor topology from the v0.12-to-v0.13
// rollout. It is re-exported (not copied) so the two runbooks can never drift
// apart on versions, while keeping #2404's files untouched on this experiment.
export { from, to, phaseVersions, scenario, versionSources } from "../v0.12-to-v0.13/versions";
