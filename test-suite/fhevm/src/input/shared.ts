/**
 * Normalizes primitive CLI arg values before command-specific validation.
 */
export const asString = (value: unknown) => (typeof value === "string" && value.length ? value : undefined);

/** Returns true only for an explicit boolean true flag value. */
export const asBool = (value: unknown) => value === true;

/** Normalizes a string or string array argument into a string list. */
export const asStringList = (value: unknown) =>
  Array.isArray(value) ? value.map(String) : typeof value === "string" && value.length ? [value] : [];
