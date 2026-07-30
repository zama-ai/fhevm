/**
 * Resolves a relayer API route prefix the same way the client does, kept in a
 * dependency-free leaf module so both the client and env resolution share one
 * source of truth (e.g. when deciding whether two A/B targets are identical).
 *
 * An explicit empty string opts out of any prefix (routes served at root, e.g.
 * `/input-proof`); only an undefined value falls back to `/v2`.
 */
export const normalizeApiPrefix = (value: string | undefined): string => {
  if (value !== undefined) {
    const raw = value.trim();
    if (raw === "") return "";
    const prefixed = raw.startsWith("/") ? raw : `/${raw}`;
    return prefixed.replace(/\/+$/, "");
  }
  return "/v2";
};
