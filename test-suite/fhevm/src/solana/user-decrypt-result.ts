// Kept free of SDK imports so the offline suite can test these checks.

/** Checks a single-entry user decrypt returned exactly one scalar cleartext, and returns it. */
export const userDecryptScalar = (clearValues: readonly { readonly value: unknown }[]): bigint => {
  if (clearValues.length !== 1) {
    throw new Error(`user-decrypt returned ${clearValues.length} clear values; expected exactly 1`);
  }
  const decrypted = clearValues[0]!.value;
  if (
    typeof decrypted !== 'bigint' &&
    typeof decrypted !== 'number' &&
    typeof decrypted !== 'boolean' &&
    typeof decrypted !== 'string'
  ) {
    throw new Error('user-decrypt returned a non-scalar cleartext');
  }
  return BigInt(decrypted);
};

/** Returns `value` when it equals `expected`. */
export const expectCleartext = (value: bigint, expected: bigint): bigint => {
  if (value !== expected) {
    throw new Error(`user-decrypt cleartext ${value} != expected ${expected}`);
  }
  return value;
};
