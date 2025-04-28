import { AppError, timeoutError, validationError } from './app-error.js'

/**
 * Task can either resolve with a value of type `A`
 * or reject with an error of type `E`.
 *
 * @template A - Action (resolve)
 * @template E - Error (reject)
 */
export class Task<A, E> {
  constructor(
    private computation: (
      resolve: (value: A) => void,
      reject: (error: E) => void,
    ) => void,
  ) {}

  /**
   * Creates a Task that resolves with the given value.
   *
   * @template A - The type of the value.
   * @template E - The type of the error.
   * @param value - The value to resolve the Task with.
   * @returns A Task that resolves with the given value.
   */
  static of<A, E>(value: A): Task<A, E> {
    return new Task(resolve => resolve(value))
  }

  static reject<A, E>(error: E): Task<A, E> {
    return new Task((_, reject) => reject(error))
  }

  /**
   * Creates a Task from a Promise.
   *
   * @template A - The type of the resolved value.
   * @template E - The type of the error.
   * @param fn - A function that returns a Promise of type `A`.
   * @returns A Task that resolves with the value of the Promise
   * or rejects with the error of the Promise.
   */
  static fromPromise<A, E>(
    promiseOrFn: Promise<A> | (() => Promise<A>),
  ): Task<A, E> {
    const promise =
      typeof promiseOrFn === 'function' ? promiseOrFn() : promiseOrFn
    return new Task((resolve, reject) => {
      promise.then(resolve).catch(reject)
    })
  }

  /**
   * Transforms the value of the Task using the given function.
   *
   * @template B - The type of the transformed value.
   * @param fn - A function that takes a value of type `A` and returns a value of type `B`.
   * @returns A new Task that resolves with the transformed value.
   */
  map<B>(fn: (value: A) => B): Task<B, E> {
    return new Task((resolve, reject) => {
      this.computation(value => resolve(fn(value)), reject)
    })
  }

  mapError<E2 = E>(fn: (error: E) => E2): Task<A, E2> {
    return new Task((resolve, reject) => {
      this.computation(resolve, error => reject(fn(error)))
    })
  }

  /**
   * Transforms the value of the Task using the given function, which returns a new Task.
   *
   * @template B - The type of the value of the new Task.
   * @param fn - A function that takes a value of type `A` and returns a Task of type `B`.
   * @returns A new Task that resolves with the value of the Task returned by the function.
   */
  chain<B>(fn: (value: A) => Task<B, E>): Task<B, E> {
    return new Task((resolve, reject) => {
      this.computation(
        value => fn(value).fork(resolve, reject),
        error => reject(error),
      )
    })
  }

  /**
   * Creates a new Task that resolves with the value of this Task if it resolves,
   * otherwise resolves with the given default value.
   *
   * @param defaultValue - The value to resolve the new Task with if this Task rejects.
   * @returns A new Task that resolves with the value of this Task or the given default value.
   */
  or(defaultValue: A): Task<A, E> {
    return new Task(resolve => {
      this.computation(resolve, () => resolve(defaultValue))
    })
  }

  /**
   * Creates a new Task that resolves with the value of this Task if it resolves,
   * otherwise calls the given function with the error and resolves with its result.
   *
   * @param fn - A function that takes an error of type `E` and returns a value of type `A`.
   * @returns A new Task that resolves with the value of this Task or the result of the given function.
   */
  orElse(fn: (error: E) => A): Task<A, E> {
    return new Task(resolve => {
      this.computation(resolve, error => resolve(fn(error)))
    })
  }

  /**
   * Creates a new Task that resolves with the value of this Task if it resolves,
   * otherwise calls the given function with the error and resolves with the value of the Task it returns.
   *
   * @param fn - A function that takes an error of type `E` and returns a Task of type `A`.
   * @returns A new Task that resolves with the value of this Task or the value of the Task returned by the given function.
   */
  orChain(fn: (error: E) => Task<A, E>): Task<A, E> {
    return new Task((resolve, reject) => {
      this.computation(
        value => resolve(value),
        error => fn(error).fork(resolve, reject),
      )
    })
  }

  /**
   * Executes the computation of the Task.
   *
   * @param resolve - A function to call with the value of type `A` if the Task resolves.
   * @param reject - A function to call with the error of type `E` if the Task rejects.
   */
  fork(resolve: (value: A) => void, reject: (error: E) => void): void {
    this.computation(resolve, reject)
  }

  match<R1, R2 = R1>(matchers: Matchers<A, E, R1, R2>): Task<R1 | R2, E> {
    return new Task<R1 | R2, E>(resolve => {
      this.computation(
        value => resolve(matchers.ok(value)),
        error => resolve(matchers.fail(error)),
      )
    })
  }

  /**
   * Converts the Task to a Promise.
   *
   * @returns A Promise that resolves with the value of type `A` or rejects with the error of type `E`.
   */
  toPromise() {
    return new Promise<A>(this.computation)
  }

  /**
   * Creates a Task that runs all the nested task, and return an array with each resolved value from each
   * task.
   *
   * @param tasks - An array of Task to be executed.
   * @returns a Task with the array of all resolved task values.
   */
  static all<
    E,
    A,
    B,
    T extends [Task<A, E>, Task<B, E>] = [Task<A, E>, Task<B, E>],
  >(tasks: T): Task<[A, B], E>
  static all<
    E,
    A,
    B,
    C,
    T extends [Task<A, E>, Task<B, E>, Task<C, E>] = [
      Task<A, E>,
      Task<B, E>,
      Task<C, E>,
    ],
  >(tasks: T): Task<[A, B, C], E>
  static all<
    E,
    A,
    B,
    C,
    D,
    T extends [Task<A, E>, Task<B, E>, Task<C, E>, Task<D, E>] = [
      Task<A, E>,
      Task<B, E>,
      Task<C, E>,
      Task<D, E>,
    ],
  >(tasks: T): Task<[A, B, C, D], E>
  static all<
    E,
    A,
    B,
    C,
    D,
    F,
    T extends [Task<A, E>, Task<B, E>, Task<C, E>, Task<D, E>, Task<F, E>] = [
      Task<A, E>,
      Task<B, E>,
      Task<C, E>,
      Task<D, E>,
      Task<F, E>,
    ],
  >(tasks: T): Task<[A, B, C, D, F], E>
  static all<
    E,
    A,
    B,
    C,
    D,
    F,
    G,
    T extends [
      Task<A, E>,
      Task<B, E>,
      Task<C, E>,
      Task<D, E>,
      Task<F, E>,
      Task<G, E>,
    ] = [
      Task<A, E>,
      Task<B, E>,
      Task<C, E>,
      Task<D, E>,
      Task<F, E>,
      Task<G, E>,
    ],
  >(tasks: T): Task<[A, B, C, D, F, G], E>
  static all<E, A, T extends Task<A, E>[] = Task<A, E>[]>(
    tasks: T,
  ): Task<A[], E>
  static all<E, T extends Task<any, E>[] = Task<any, E>[]>(
    tasks: T,
  ): Task<any[], E> {
    return new Task(function (resolve, reject) {
      // Note: I use `Promise.allSettled` to be sure all promises settle before
      // continuing
      Promise.allSettled(tasks.map(t => t.toPromise()))
        .then(promises =>
          promises.some(isRejected)
            ? reject(promises.find(isRejected)!.reason)
            : resolve(promises.filter(isFullfilled).map(p => p.value)),
        )
        .catch(reject)
    })
  }

  /**
   * Creates a Task that runs all the nested task, and return the first to complete.
   *
   * @param tasks - An array of Task to be executed.
   * @returns a Task with the array of all resolved task values.
   */
  static race<
    E,
    A,
    B,
    T extends [Task<A, E>, Task<B, E>] = [Task<A, E>, Task<B, E>],
  >(tasks: T): Task<A | B, E>
  static race<
    E,
    A,
    B,
    C,
    T extends [Task<A, E>, Task<B, E>, Task<C, E>] = [
      Task<A, E>,
      Task<B, E>,
      Task<C, E>,
    ],
  >(tasks: T): Task<A | B | C, E>
  static race<E, A, T extends Task<A, E>[] = Task<A, E>[]>(tasks: T): Task<A, E>
  static race<E, T extends Task<any, E>[] = Task<any, E>[]>(
    tasks: T,
  ): Task<any, E> {
    return new Task(function (resolve, reject) {
      Promise.race(tasks.map(t => t.toPromise()))
        .then(resolve)
        .catch(reject)
    })
  }

  static timeout<T>(seconds: number): Task<T, AppError> {
    if (seconds < 0) {
      return Task.reject(validationError('seconds should be greater then 0'))
    }
    return new Task((_resolve, reject) => {
      setTimeout(function () {
        reject(timeoutError())
      }, seconds * 1000)
    })
  }

  tap(fn: (value: A) => void): Task<A, E> {
    return new Task((resolve, reject) => {
      this.fork(value => {
        fn(value)
        resolve(value)
      }, reject)
    })
  }

  tapError(fn: (error: E) => void): Task<A, E> {
    return new Task((resolve, reject) => {
      this.fork(resolve, (error: E) => {
        fn(error)
        reject(error)
      })
    })
  }
}

function isFullfilled<T>(
  p: PromiseSettledResult<T>,
): p is PromiseFulfilledResult<T> {
  return p.status === 'fulfilled'
}

function isRejected<T>(p: PromiseSettledResult<T>): p is PromiseRejectedResult {
  return p.status === 'rejected'
}

interface Matchers<T, E, R1, R2 = R1> {
  ok(value: T): R1
  fail(error: E): R2
}

/**
 * It executes the task, without throwing an error in case of failure.
 *
 * @param task Task to execute
 * @returns A promise that resolves with the result of the task
 */
export function executeTask<A extends object | string | number | void, E>(
  task: Task<A, E>,
): Promise<
  | { success: true; value: A; error: undefined }
  | { success: false; value: undefined; error: E }
> {
  return new Promise(resolve => {
    task.fork(
      v => resolve({ success: true, value: v, error: undefined }),
      e => resolve({ success: false, value: undefined, error: e }),
    )
  })
}
