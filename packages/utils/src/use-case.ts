import type { AppError } from './app-error.js'
import type { Task } from './task.js'

export interface UseCase<Input, Output> {
  execute(
    input: Input,
    context?: Record<string, unknown>,
  ): Task<Output, AppError>
}
