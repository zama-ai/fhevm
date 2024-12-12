import type { AppError } from './app-error'
import type { Task } from './task'

export interface UseCase<Input, Output> {
  execute(input: Input, context?: Record<string, any>): Task<Output, AppError>
}
