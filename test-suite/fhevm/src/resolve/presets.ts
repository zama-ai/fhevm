/**
 * Defines hardcoded companion version pins layered onto non-network targets during bundle resolution.
 */
export const MAINLINE_COMPANIONS = {
  CORE_VERSION: "v0.13.10-rc.3",
  RELAYER_VERSION: "v0.11.0-rc.2",
  RELAYER_MIGRATE_VERSION: "v0.11.0-rc.1",
} as const;

export const NON_NETWORK_COMPANIONS = {
  "latest-main": MAINLINE_COMPANIONS,
  "sha": MAINLINE_COMPANIONS,
} as const;
