import { MS_NAME } from '#constants.js'
import { back, web3 } from 'messages'
import { AppError, ISubscriber, PubSub, Task, UseCase } from 'utils'

type Input = Extract<web3.Web3Event, { type: 'web3:fhe-event:detected' }>

export class FheEventDetected implements UseCase<Input, void> {
  constructor(
    private readonly pubsub: PubSub<back.BackEvent | web3.Web3Event>,
  ) {
    this.pubsub.subscribe(
      'web3:fhe-event:detected',
      this.handleFheEventDetected,
    )
  }

  private handleFheEventDetected: ISubscriber<back.BackEvent | web3.Web3Event> =
    event =>
      event.type === 'web3:fhe-event:detected'
        ? this.execute(event)
        : Task.of(void 0)

  execute = ({ payload, meta }: Input): Task<void, AppError> => {
    return this.pubsub.publish(
      back.dappStatsAvailable(
        {
          requestId: payload.requestId,
          chainId: payload.chainId,
          address: payload.address,
          name: payload.name,
          timestamp: payload.timestamp,
          externalRef: payload.id,
        },
        {
          ...meta,
          // Note: I need to override the message otherwise is stopped
          [`${MS_NAME}-dir`]: 'out',
        },
      ),
    )
  }
}
