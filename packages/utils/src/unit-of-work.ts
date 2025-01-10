import { Task } from './task.js'

export interface UnitOfWork {
  exec<A, E>(task: Task<A, E>): Task<A, E>
}
