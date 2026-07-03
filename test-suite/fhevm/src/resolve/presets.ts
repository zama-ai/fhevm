/**
 * Defines hardcoded companion version pins layered onto non-network targets during bundle resolution.
 */
export const MAINLINE_COMPANIONS = {
  CORE_VERSION: "b9087af",
} as const;

export const NON_NETWORK_COMPANIONS = {
  "latest-main": MAINLINE_COMPANIONS,
  "sha": MAINLINE_COMPANIONS,
} as const;
