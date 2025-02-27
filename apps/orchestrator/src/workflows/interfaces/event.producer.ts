import { back, web3 } from 'messages'
import { type AppError, Task } from 'utils'

export interface EventProducer {
  publish(event: back.BackEvent | web3.Web3Event): Task<void, AppError>
}
