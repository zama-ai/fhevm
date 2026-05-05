/**
 * Defines hardcoded companion version pins layered onto non-network targets during bundle resolution.
 */
export const MAINLINE_COMPANIONS = {
  CORE_VERSION: "c57f52f",
} as const;

export const NON_NETWORK_COMPANIONS = {
  "latest-main": MAINLINE_COMPANIONS,
  "sha": MAINLINE_COMPANIONS,
} as const;
