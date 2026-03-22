export function simpleDeepFreeze<T extends object>(obj: T): Readonly<T> {
  Object.freeze(obj);
  for (const value of Object.values(obj)) {
    if (
      value !== null &&
      typeof value === "object" &&
      !Array.isArray(value) &&
      !Object.isFrozen(value)
    ) {
      simpleDeepFreeze(value);
    }
  }
  return obj;
}

/**
 * Defines a non-enumerable, non-writable, non-configurable property on the target object.
 * The property is hidden from `Object.keys()` / `Object.entries()` and cannot be
 * overwritten or reconfigured after creation.
 *
 * @param target - The object to define the property on.
 * @param fnName - The property name.
 * @param fn - The value to assign.
 * @returns The target object, typed with the added property.
 */
export function addInternalFunction<
  T extends object,
  FnName extends string,
  Fn extends (...args: never[]) => unknown,
>(target: T, fnName: FnName, fn: Fn): T & Readonly<Record<FnName, Fn>> {
  Object.defineProperty(target, fnName, {
    value: fn,
    writable: false,
    configurable: false,
    enumerable: false,
  });
  return target as T & Readonly<Record<FnName, Fn>>;
}
