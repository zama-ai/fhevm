import { FheEvent } from '#domain/entities/fhe-event.js'
import { FheEventService } from '#domain/services/fhe-event.service.js'
import { ChainId } from '#domain/entities/value-objects.js'
import { FheEventRepository } from '#domain/services/fhe-event.repository.js'
import { AppError, PubSub, Subscriber, Task, UseCase } from 'utils'
import { web3 } from 'messages'
import { Logger } from '@nestjs/common'

export class FetchFHEEvents implements UseCase<ChainId, FheEvent[]> {
  private readonly logger = new Logger(FetchFHEEvents.name)
  constructor(
    private readonly pubsub: PubSub<web3.Web3Event>,
    private readonly service: FheEventService,
    private readonly repo: FheEventRepository,
  ) {
    this.logger.debug(`subscribing to web3:fhe-event:requested`)
    this.pubsub.subscribe('web3:fhe-event:requested', this.handleFheEvent)
  }

  handleFheEvent: Subscriber<web3.Web3Event, 'web3:fhe-event:requested'> = (
    event,
    payload,
  ): Task<void, AppError> => {
    this.logger.log(`received ${event}: ${JSON.stringify(payload)}`)
    return ChainId.fromString(payload.chainId)
      .asyncChain(this.execute)
      .map<void>(() => void 0)
  }

  execute = (chainId: ChainId): Task<FheEvent[], AppError> => {
    // Note: use a Unit of Work to handle transactions
    return this.repo
      .getLastBlockNumber(chainId)
      .map(blockNumebr => ({ chainId, blockNumebr }))
      .chain(({ chainId, blockNumebr }) =>
        this.service.fetchEvents(chainId, blockNumebr),
      )
      .chain(events =>
        Task.all<AppError, FheEvent>(events.map(this.repo.create)),
      )
      .tap(events => {
        events.forEach(event => {
          const toPublish = web3.fheDetected({
            address: event.callerAddress,
            chainId: event.chainId,
            name: event.name,
            timestamp: event.timestamp,
          })
          this.logger.log(
            `publishing ${toPublish.type}: ${JSON.stringify(toPublish.payload)}`,
          )
          this.pubsub.publish(toPublish)
        })
      })
  }
}
