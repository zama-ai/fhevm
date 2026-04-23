/**
 * Defines hardcoded companion version pins layered onto non-network targets during bundle resolution.
 */
export const MODERN_RELAYER_VERSION = "sha-29b0750";
export const MODERN_RELAYER_MIGRATE_VERSION = "sha-29b0750";

export const MAINLINE_COMPANIONS = {
  CORE_VERSION: "v0.13.10-rc.3",
} as const;

export const NON_NETWORK_COMPANIONS = {
  "latest-main": MAINLINE_COMPANIONS,
  "sha": MAINLINE_COMPANIONS,
} as const;
