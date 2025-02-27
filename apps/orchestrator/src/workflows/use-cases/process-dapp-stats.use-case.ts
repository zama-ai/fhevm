import {
  DAppStats,
  DAppStatsEvents,
  isDAppStatsEvent,
} from '#workflows/entities/dapp-stats.js'
import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { Logger } from '@nestjs/common'
import { back, web3 } from 'messages'
import { AppError, IPubSub, ISubscriber, Task, UseCase } from 'utils'

export class ProcessDAppStats implements UseCase<DAppStatsEvents, void> {
  private readonly logger = new Logger(ProcessDAppStats.name)

  constructor(
    private readonly pupsub: IPubSub<back.BackEvent | web3.Web3Event>,
    private readonly producer: EventProducer,
  ) {
    this.pupsub.subscribe('*', this.handleEvent)
  }

  private handleEvent: ISubscriber<back.BackEvent | web3.Web3Event> = event => {
    this.logger.verbose(`event ${event.type} received`)
    return isDAppStatsEvent(event)
      ? this.execute(event)
          .tap(() => {
            this.logger.verbose(`event ${event.type} processed`)
          })
          .mapError(err => {
            this.logger.warn(
              `failed to process event ${event.type}: ${err._tag}/${err.message}`,
            )
            return err
          })
      : Task.of<void, AppError>(void 0).tap(() => {
          this.logger.verbose(`event ${event.type} ignored`)
        })
  }

  execute = (event: DAppStatsEvents): Task<void, AppError> => {
    return Task.of<DAppStats, AppError>(new DAppStats()).chain(dAppStats =>
      Task.all<AppError, void>(
        dAppStats.send(event).map(this.producer.publish),
      ).map(() => void 0),
    )
  }
}
