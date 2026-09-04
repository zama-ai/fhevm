type Env = Record<string, string>;

export const scenario = "four-party-threshold-kms";

export const from = {
  CORE_VERSION: "v0.13.21",
} satisfies Env;

export const to = { ...from, CORE_VERSION: "v0.13.22" } satisfies Env;

// The surrounding stack is not listed here: resolveVersionLock prepends the sources of
// the bundle it resolves, which carry the exact main sha these locks were built from.
export const versionSources = [
  "rollout=v0.13.21-to-v0.13.22-kms-node-by-node",
  "kms-core=v0.13.21->v0.13.22",
];
