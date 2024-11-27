import type { AppError } from './app-error'
import type { Task } from './task'

export interface UseCase<Input, Output> {
  execute(input: Input): Task<Output, AppError>
}
