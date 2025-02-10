import { back, web3 } from 'messages'
import { AppError, ISubscriber, PubSub, Task, UseCase } from 'utils'

type Input = Extract<back.BackEvent, { type: 'back:dapp:stats-requested' }>

/**
 * It listen for 'back:dapp:stats-requested' and maps them to a
 * web3:fhe-event:requested event.
 */
export class DAppStatsRequested implements UseCase<Input, void> {
  constructor(
    private readonly pubsub: PubSub<back.BackEvent | web3.Web3Event>,
  ) {
    this.pubsub.subscribe(
      'back:dapp:stats-requested',
      this.handleDAppStatsRequested,
    )
  }

  private handleDAppStatsRequested: ISubscriber<
    back.BackEvent | web3.Web3Event
  > = event =>
    event.type === 'back:dapp:stats-requested'
      ? this.execute(event)
      : Task.of(void 0)

  execute = ({ payload, meta }: Input): Task<void, AppError> => {
    return this.pubsub.publish(
      web3.fheRequested(
        {
          chainId: payload.chainId,
          address: payload.address,
        },
        meta,
      ),
    )
  }
}
