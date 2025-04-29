interface Matchers<T, R1, R2 = R1> {
  some(value: T): R1
  none(): R2
}

export interface Some<T> {
  _tag: 'Some'
  value: T
  /*** Returns the value of the Option if it exists, otherwise throws an error. */
  unwrap(): T
  /*** Returns the value of the Option if it exists, otherwise returns the provided default value. */
  unwrapOr(defaultValue: T): T
  /*** Returns the value of the Option if it exists, otherwise calls the provided function and returns its result. */
  unwrapOrElse(fn: () => T): T

  or(value: T): Option<T>
  orElse(fn: () => T): Option<T>

  /*** Returns true if the Option contains a value, false otherwise. */
  isSome(this: Option<T>): this is Some<T>
  /*** Returns true if the Option does not contain a value, false otherwise. */
  isNone(this: Option<T>): this is None
  /*** Calls the provided function with the value and wrap its returned value in an Option. */
  map: <U>(fn: (value: T) => U) => Option<U>
  /*** Calls the provided function with the value and returns its returned Option. */
  flatMap: <U>(fn: (value: T) => Option<U>) => Option<U>
  /*** Calls the `some` matcher if it contains a value, otherwise the `none` matcher.*/
  match<R1, R2 = R1>(matchers: Matchers<T, R1, R2>): R1 | R2
}
export interface None {
  _tag: 'None'
  /*** Throws an error because None does not contain a value. */
  unwrap(): never
  /*** Returns the provided default value because None does not contains a value. */
  unwrapOr<T>(defaultValue: T): T
  /*** Calls the provided function and returns its result because None does not contains a value. */
  unwrapOrElse<T>(fn: () => T): T
  /*** Returns true if the Option contains a value, false otherwise. */
  or<T>(value: T): Option<T>
  orElse<T>(fn: () => T): Option<T>
  isSome<T>(this: Option<T>): this is Some<T>
  /*** Returns true if the Option does not contain a value, false otherwise. */
  isNone<T>(this: Option<T>): this is None
  /*** Calls the provided function with the value and wrap its returned value in an Option. */
  map: <T, U>(fn: (value: T) => U) => Option<U>
  /*** Calls the provided function with the value and returns its returned Option. */
  flatMap: <T, U>(fn: (value: T) => Option<U>) => Option<U>
  /*** Calls the `none` matcher because None does not contains a value.*/
  match<T, R1, R2 = R1>(matchers: Matchers<T, R1, R2>): R1 | R2
}
export type Option<T> = Some<T> | None

/**
 * Returns true if the Option has a value, false otherwise.
 * @param option The Option to check
 * @returns True if the Option has a value, false otherwise.
 */
export function isSome<T>(option: Option<T>): option is Some<T> {
  console.log(`isSome ${JSON.stringify(option)} ${option._tag}`)
  return option._tag === 'Some'
}

/**
 * Returns true if the Option has no value, false otherwise
 * @param option The Option to check
 * @returns True is the Option has no value, false othersie
 */
export function isNone<T>(option: Option<T>): option is None {
  return option._tag === 'None'
}

class SomeImpl<T> implements Some<T> {
  _tag = 'Some' as const
  constructor(readonly value: T) {}
  unwrap(): T {
    return this.value
  }
  unwrapOr(): T {
    return this.value
  }
  unwrapOrElse(): T {
    return this.value
  }
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  or(_value: T): Option<T> {
    return this
  }
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  orElse(_fn: () => T): Option<T> {
    return this
  }
  isSome(this: Option<T>): this is Some<T> {
    return true
  }
  isNone(this: Option<T>): this is None {
    return false
  }
  map<U>(fn: (value: T) => U) {
    return new SomeImpl<U>(fn(this.value))
  }
  flatMap<U>(fn: (value: T) => Option<U>) {
    return fn(this.value)
  }
  match<R1, R2 = R1>(matchers: Matchers<T, R1, R2>): R1 | R2 {
    return matchers.some(this.value)
  }
}

/**
 * Creates an Option with a value.
 * @param value The value to be wrapped in the Option.
 * @returns An Option with the provided value.
 */
export function some<T>(value: T): Some<T> {
  return new SomeImpl(value)
}

class NoneImpl implements None {
  _tag = 'None' as const
  unwrap(): never {
    throw new Error('Cannot unwrap None')
  }
  unwrapOr<T>(defaultValue: T): T {
    return defaultValue
  }
  unwrapOrElse<T>(fn: () => T): T {
    return fn()
  }
  or<T>(value: T): Option<T> {
    return some(value)
  }

  orElse<T>(fn: () => T): Option<T> {
    return some(fn())
  }
  isSome<T>(this: Option<T>): this is Some<T> {
    return false
  }
  isNone<T>(this: Option<T>): this is None {
    return true
  }
  map() {
    return this
  }
  flatMap() {
    return this
  }
  match<T, R1, R2 = R1>(matchers: Matchers<T, R1, R2>): R1 | R2 {
    return matchers.none()
  }
}
/**
 * Represents an empty Option with no value.
 * @returns An empty option
 */
export function none(): None {
  return new NoneImpl()
}

export function fromNullable<T>(value: T | null | undefined): Option<T> {
  return value === null || value === undefined ? none() : some(value)
}
