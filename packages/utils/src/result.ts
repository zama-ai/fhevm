import { Task } from './task.js'

/*** Represents a successful computation. */
export interface Ok<T, E> {
  _tag: 'Ok'
  value: T
  /*** Returns the value of the Result if it is successful, otherwise throws an error. */
  unwrap(): T
  /*** Retruns the value of the Result if it is successful, otherwise returns the provided default value. */
  unwrapOr(defaultValue: T): T
  /*** Returns the value of the Result if it is successful, otherwise calls the provided function and returns its result. */
  unwrapOrElse(fn: () => T): T
  /*** Returns true if the Result is successful, false otherwise. */
  isOk(this: Result<T, E>): this is Ok<T, E>
  /*** Returns true if the Result is a failure, false otherwise. */
  isFail(this: Result<T, E>): this is Fail<T, E>
  /*** Calls the provided function with the current value and wraps the result in a Result type.*/
  map<U>(fn: (value: T) => U): Result<U, E>
  /*** Calls the provided function with the current value and wraps the awaited result in a Result type.*/
  asyncMap<U>(fn: (value: T) => U): Task<U, E>
  /*** Calls the provided function with the current value and returns the result. */
  chain<U>(fn: (value: T) => Result<U, E>): Result<U, E>
  /*** Calls the provided function with the current value and returns the awaited result. */
  asyncChain<U = T>(fn: (value: T) => Task<U, E>): Task<U, E>
  /** Convert a Result<T,E> to a Task<T, E> */
  async(): Task<T, E>
  /*** Calls the `ok` matcher if the computation is successful, otherwise it calls the `fail` one. */
  match<R1, R2 = R1>(matchers: Matchers<T, E, R1, R2>): R1 | R2
}

/*** Represent a failed computation. */
export interface Fail<T, E> {
  _tag: 'Fail'
  error: E
  /*** Returns the value of the Result if it is successful, otherwise throws an error. */
  unwrap(): T
  /*** Retruns the value of the Result if it is successful, otherwise returns the provided default value. */
  unwrapOr(defaultValue: T): T
  /*** Returns the value of the Result if it is successful, otherwise calls the provided function and returns its result. */
  unwrapOrElse(fn: () => T): T
  /*** Returns true if the Result is successful, false otherwise. */
  isOk(this: Result<T, E>): this is Ok<T, E>
  /*** Returns true if the Result is a failure, false otherwise. */
  isFail(this: Result<T, E>): this is Fail<T, E>
  /*** Calls the provided function with the current value and wraps the result in a Result type.*/
  map<U = T>(fn: (value: T) => U): Result<U, E>
  /*** Calls the provided function with the current value and wraps the awauted result in a Result type.*/
  asyncMap<U = T>(fn: (value: T) => U): Task<U, E>
  /*** Calls the provided function with the current value and returns the result. */
  chain<U = T>(fn: (value: T) => Result<U, E>): Result<U, E>
  /*** Calls the provided function with the current value and returns the awaited result. */
  asyncChain<U = T>(fn: (value: T) => Task<U, E>): Task<U, E>
  async(): Task<T, E>
  /*** Calls the `ok` matcher if the computation is successful, otherwise it calls the `fail` one. */
  match<R1, R2 = R1>(matchers: Matchers<T, E, R1, R2>): R1 | R2
}

export type Result<T, E> = Ok<T, E> | Fail<T, E>
interface Matchers<T, E, R1, R2 = R1> {
  ok(value: T): R1
  fail(error: E): R2
}

/**
 * Returns true if the Result is successful, false otherwise.
 * @param result The result to check
 * @returns True if result is `Ok`, false otherwise.
 */
export function isOk<T, E>(result: Result<T, E>): result is Ok<T, E> {
  return result._tag === 'Ok'
}

/**
 * Returns true if the Result is a failure, false otherwise.
 * @param result The result to check
 * @returns True if result is `Fail`, false otherwise.
 */
export function isFail<T, E>(result: Result<T, E>): result is Fail<T, E> {
  return result._tag === 'Fail'
}

class OkImpl<T, E> implements Ok<T, E> {
  _tag: 'Ok'
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
  isOk(this: Result<T, E>): this is Ok<T, E> {
    return true
  }
  isFail(this: Result<T, E>): this is Fail<T, E> {
    return false
  }
  map<U>(fn: (value: T) => U) {
    return new OkImpl<U, E>(fn(this.value))
  }
  asyncMap<E, U>(fn: (value: T) => U): Task<U, E> {
    return Task.of(fn(this.value))
  }
  chain<E, U>(fn: (value: T) => Result<U, E>) {
    return fn(this.value)
  }
  asyncChain<E, U>(fn: (value: T) => Task<U, E>): Task<U, E> {
    return fn(this.value)
  }
  async() {
    return Task.of<T, E>(this.value)
  }
  match<E, R1, R2 = R1>(matchers: Matchers<T, E, R1, R2>) {
    return matchers.ok(this.value)
  }
}

/***
 * Creates a successful result.
 * @param value The value to be wrapped in the Result.
 * @returns A successful computation result.
 */
export function ok<T, E>(value: T): Ok<T, E> {
  return new OkImpl(value)
}

class FailImpl<T, E> implements Fail<T, E> {
  _tag: 'Fail'
  constructor(readonly error: E) {}
  unwrap<T>(): T {
    throw this.error
  }
  unwrapOr<T>(defaultValue: T): T {
    return defaultValue
  }
  unwrapOrElse<T>(fn: () => T): T {
    return fn()
  }
  isOk(this: Result<T, E>): this is Ok<T, E> {
    return false
  }
  isFail(this: Result<T, E>): this is Fail<T, E> {
    return true
  }
  map<U>() {
    return this as unknown as Result<U, E>
  }
  asyncMap<U>(): Task<U, E> {
    return new Task((_, reject) => reject(this.error))
  }
  chain<U>() {
    return this as unknown as Result<U, E>
  }
  asyncChain<U>(): Task<U, E> {
    return Task.reject(this.error)
  }
  async() {
    return Task.reject<T, E>(this.error)
  }
  match<T, R1, R2 = R1>(matchers: Matchers<T, E, R1, R2>) {
    return matchers.fail(this.error)
  }
}

/***
 * Creates a failed Result with the given error..
 * @param error The error that caused the computation to fail.
 * @returns A Result with the provided error.
 */
export function fail<T, E>(error: E): Fail<T, E> {
  return new FailImpl(error)
}

export function wrap<T, E, R>(fn: (value: T) => R) {
  return function (result: Result<T, E>): Result<R, E> {
    return isOk(result)
      ? ok(fn(result.value))
      : (result as unknown as Fail<R, E>)
  }
}
export function match<R1, R2, T, E>(matchers: Matchers<T, E, R1, R2>) {
  return function (result: Result<T, E>) {
    return isOk(result)
      ? matchers.ok(result.value)
      : matchers.fail(result.error)
  }
}
