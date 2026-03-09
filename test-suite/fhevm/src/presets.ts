export const NON_NETWORK_COMPANIONS = {
  "latest-release": {
    CORE_VERSION: "v0.13.0",
    RELAYER_VERSION: "v0.9.0",
    RELAYER_MIGRATE_VERSION: "v0.9.0",
  },
  "latest-main": {
    CORE_VERSION: "v0.13.0",
    RELAYER_VERSION: "v0.9.0",
    RELAYER_MIGRATE_VERSION: "v0.9.0",
  },
} as const;

export const companionPreset = (target: keyof typeof NON_NETWORK_COMPANIONS) =>
  NON_NETWORK_COMPANIONS[target];
