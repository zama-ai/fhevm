export const ensureArray = <T>(value: T | ReadonlyArray<T>): T[] =>
    Array.isArray(value) ? [...value] : [value as T]
