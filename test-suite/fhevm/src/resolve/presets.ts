/**
 * Defines hardcoded companion version pins layered onto non-network targets during bundle resolution.
 */
export const MAINLINE_COMPANIONS = {
  CORE_VERSION: "v0.13.10-rc.0",
  RELAYER_VERSION: "sha-5e38c70",
  RELAYER_MIGRATE_VERSION: "v0.10.0-rc.1",
} as const;

export const NON_NETWORK_COMPANIONS = {
  "latest-main": MAINLINE_COMPANIONS,
  "sha": MAINLINE_COMPANIONS,
} as const;
