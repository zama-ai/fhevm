import { back, email, relayer, web3 } from 'messages'
import { type AppError, Task } from 'utils'

export const EVENT_PRODUCER = Symbol('EventProducer')

export interface EventProducer {
  publish(
    event:
      | back.BackEvent
      | email.EmailEvent
      | relayer.RelayerEvent
      | web3.Web3Event,
  ): Task<void, AppError>
}
