/**
 * Checks a single-entry user decrypt returned exactly one scalar cleartext equal to `expected`, and
 * returns it. Kept free of SDK imports so the offline suite can test it.
 */
export const expectUserDecryptValue = (clearValues: readonly { readonly value: unknown }[], expected: bigint): bigint => {
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
  const value = BigInt(decrypted);
  if (value !== expected) {
    throw new Error(`user-decrypt cleartext ${value} != expected ${expected}`);
  }
  return value;
};
