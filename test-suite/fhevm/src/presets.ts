const DEFAULT_COMPANIONS = {
  CORE_VERSION: "v0.13.0",
  RELAYER_VERSION: "v0.9.0",
  RELAYER_MIGRATE_VERSION: "v0.9.0",
} as const;

export const NON_NETWORK_COMPANIONS = {
  "latest-release": DEFAULT_COMPANIONS,
  "latest-main": DEFAULT_COMPANIONS,
  "sha": DEFAULT_COMPANIONS,
} as const;

export const companionPreset = (target: keyof typeof NON_NETWORK_COMPANIONS) =>
  NON_NETWORK_COMPANIONS[target];
