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
   * Transforms the value of the Task using the given function.
   *
   * @template B - The type of the transformed value.
   * @param fn - A function that takes a value of type `A` and returns a value of type `B`.
   * @returns A new Task that resolves with the transformed value.
   */
  map<B>(fn: (value: A) => B): Task<B, E> {
    return new Task((resolve, reject) => {
      this.computation(
        value => resolve(fn(value)),
        error => reject(error),
      )
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
   * Executes the computation of the Task.
   *
   * @param resolve - A function to call with the value of type `A` if the Task resolves.
   * @param reject - A function to call with the error of type `E` if the Task rejects.
   */
  fork(resolve: (value: A) => void, reject: (error: E) => void): void {
    this.computation(resolve, reject)
  }

  /**
   * Converts the Task to a Promise.
   *
   * @returns A Promise that resolves with the value of type `A` or rejects with the error of type `E`.
   */
  toPromise() {
    return new Promise<A>((resolve, reject) => {
      this.fork(resolve, reject)
    })
  }
}
