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
