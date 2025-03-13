import { back } from 'messages'
import { AppError, Task } from 'utils'

export interface IProducer {
  publish(event: back.BackEvent): Task<void, AppError>
}
