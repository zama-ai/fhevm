type Env = Record<string, string>;

export const scenario = "four-party-threshold-kms";

export const from = {
  CORE_VERSION: "v0.13.21",
} satisfies Env;

export const to = { ...from, CORE_VERSION: "v0.13.22" } satisfies Env;

export const versionSources = [
  "rollout=v0.13.21-to-v0.13.22-kms-node-by-node",
  "kms-core=v0.13.21->v0.13.22",
  "fhevm=latest-main",
  "sdk=@fhevm/sdk",
];
