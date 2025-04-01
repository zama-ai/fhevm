import { back, relayer, web3 } from 'messages'
import { type AppError, Task } from 'utils'

export interface EventProducer {
  publish(
    event: back.BackEvent | relayer.RelayerEvent | web3.Web3Event,
  ): Task<void, AppError>
}
