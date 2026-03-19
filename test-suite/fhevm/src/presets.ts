export const MAINLINE_COMPANIONS = {
  CORE_VERSION: "v0.13.0",
  RELAYER_VERSION: "sha-29b0750",
  RELAYER_MIGRATE_VERSION: "sha-29b0750",
} as const;

export const NON_NETWORK_COMPANIONS = {
  "latest-main": MAINLINE_COMPANIONS,
  "sha": MAINLINE_COMPANIONS,
} as const;
