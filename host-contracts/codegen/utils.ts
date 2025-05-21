/**
 * Generates a string representing a uint type with the specified number of bits.
 *
 * @param bits - The number of bits for the uint type. Must be greater than 8.
 * @returns A string representing the uint type with the specified number of bits.
 * @throws Will throw an error if the number of bits is less than or equal to 8.
 */
export const getUint = (bits: number) => {
  if (bits < 8) throw new Error('Bits must be greater than 8');
  return `uint${bits}`;
};
/**
 * Finds the minimum value among a list of `bigint` numbers.
 *
 * @param args - An array of `bigint` values to compare.
 * @returns The smallest `bigint` value from the provided arguments.
 */
export const findMinimumValueInBigIntArray = (...args: bigint[]) => {
  return args.reduce((min, e) => (e < min ? e : min), args[0]);
};

/**
 * Finds the maximum value among a list of `bigint` numbers.
 *
 * @param args - An array of `bigint` numbers to compare.
 * @returns The largest `bigint` value from the provided arguments.
 */
export const findMaximumValueInBigIntArray = (...args: bigint[]) => {
  return args.reduce((max, e) => (e > max ? e : max), args[0]);
};

/**
 * Generates a random number within a specified bit range.
 *
 * @param bits - The number of bits to determine the range of the generated number.
 * @param minValue - The minimum value to return.
 * @returns A random BigInt number within the range defined by the number of bits and at least minValue.
 */
export const generateRandomNumber = (bits: number, minValue: bigint = 5n) => {
  // @dev minValue is set at 5 to prevent underflows since tests would use smallest - 4n.
  const power = BigInt(Math.pow(2, bits) - 1);
  const maxRange = findMinimumValueInBigIntArray(power, BigInt(Number.MAX_SAFE_INTEGER));
  const subtract = findMaximumValueInBigIntArray(BigInt(Math.floor(Math.random() * Number(maxRange))), minValue);
  return findMaximumValueInBigIntArray(power - subtract, minValue);
};
