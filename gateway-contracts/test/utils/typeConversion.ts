// Convert a struct object (or a list of it) to a list of its values
// For example :
// Input: { a: 1, b: 2 }
// Output: [1, 2]
// This is useful for checking emitted events data (ie, when calling `withArgs` in tests), since the
// input structs(defined in TypeScript) given to the Solidity contracts are converted to list of
// values when emitted as events
export function toValues<T extends object>(input: T | T[]): unknown[] | unknown[][] {
  if (Array.isArray(input)) {
    // If input is an array of structs, map each struct to its values recursively
    return input.map((item) => toValues(item) as unknown[]);
  }
  // If input is a single struct, return its values, recursively converting nested (non-null) objects
  return Object.values(input).map((value) => {
    if (typeof value === "object" && value !== null) {
      return toValues(value);
    }
    return value;
  });
}
