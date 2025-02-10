import { DAppStat } from '#dapps/domain/entities/dapp-stat.js'
import { DAppStatId } from '#dapps/domain/entities/value-objects.js'
import { DAppRepository } from '#dapps/domain/repositories/dapp.repository.js'
import { Logger } from '@nestjs/common'
import { back } from 'messages'
import {
  AppError,
  PubSub,
  type ISubscriber,
  Task,
  UseCase,
  isNotFoundError,
} from 'utils'

type Input = {
  chainId: string
  address: string
  name: string
  timestamp: string
  externalRef: string
}
type Output = DAppStat

export class StoreDAppStats implements UseCase<Input, Output> {
  logger = new Logger(StoreDAppStats.name)
  constructor(
    private readonly pubsub: PubSub<back.BackEvent>,
    private readonly repo: DAppRepository,
  ) {
    this.pubsub.subscribe(
      'back:dapp:stats-available',
      this.handleStatsAvailableEvent,
    )
  }

  private handleStatsAvailableEvent: ISubscriber<back.BackEvent> = event => {
    // Note: I should receive only `back:dapp:stats-available` events but I
    // need to restrict the event type
    return event.type === 'back:dapp:stats-available'
      ? this.execute(event.payload)
          .tap(stat => {
            this.logger.debug(`stat created ${JSON.stringify(stat.toJSON())}`)
          })
          .map<void>(() => void 0)
          .orChain(err =>
            isNotFoundError(err)
              ? Task.of<void, AppError>(void 0)
              : Task.reject<void, AppError>(err),
          )
      : Task.of(void 0)
  }

  execute = (input: Input): Task<DAppStat, AppError> => {
    return this.repo.findByAddress(input.chainId, input.address).chain(dapp =>
      this.repo.createStat(dapp.id, {
        id: DAppStatId.random().value,
        name: input.name,
        timestamp: new Date(input.timestamp),
        dappId: dapp.id.value,
        externalRef: input.externalRef,
      }),
    )
  }
}
