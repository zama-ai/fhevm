import { Task } from './task'

export interface UnitOfWork {
  exec<A, E>(task: Task<A, E>): Task<A, E>
}
