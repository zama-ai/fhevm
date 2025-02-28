// Convert a struct object (or a list of it) to a list of its values
// For example :
// Input: { a: 1, b: 2 }
// Output: [1, 2]
// This is useful for checking emitted events data (ie, when calling `withArgs` in tests), since the
// input structs(defined in TypeScript) given to the Solidity contracts are converted to list of
// values when emitted as events
export function toValues<T extends object>(input: T | T[]): unknown[] | unknown[][] {
  if (Array.isArray(input)) {
    // If input is an array of structs, map each struct to its values
    return input.map((item) => Object.values(item));
  }
  // If input is a single struct, return its values
  return Object.values(input);
}
