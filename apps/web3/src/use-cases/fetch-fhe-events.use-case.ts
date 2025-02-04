import { FheEvent } from '#domain/entities/fhe-event.js'
import { FheEventService } from '#domain/services/fhe-event.service.js'
import { ChainId } from '#domain/entities/value-objects.js'
import { FheEventRepository } from '#domain/services/fhe-event.repository.js'
import { AppError, PubSub, Subscriber, Task, UseCase } from 'utils'
import { web3 } from 'messages'
import { Logger } from '@nestjs/common'
import { randomUUID } from 'crypto'

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

  handleFheEvent: Subscriber<web3.Web3Event> = (
    event,
  ): Task<void, AppError> => {
    this.logger.log(`received ${event.type}: ${JSON.stringify(event.payload)}`)
    return ChainId.fromString(event.payload.chainId)
      .asyncChain(this.execute)
      .map<void>(() => void 0)
  }

  execute = (chainId: ChainId): Task<FheEvent[], AppError> => {
    this.logger.log(`fetching fhe events for chain ${chainId.value}`)
    // Note: use a Unit of Work to handle transactions
    return this.repo
      .getLastBlockNumber(chainId)
      .tap(block => {
        this.logger.debug(`last block for ${chainId.value} is ${block}`)
      })
      .map(blockNumber => ({ chainId, blockNumber }))
      .chain(({ chainId, blockNumber }) =>
        this.service.fetchEvents(chainId, blockNumber),
      )
      .tap(events => {
        this.logger.debug(
          `fetched #${events.length} events from chain ${chainId.value}`,
        )
      })
      .chain(events =>
        Task.all<AppError, FheEvent>(events.map(this.repo.create)),
      )
      .tap(events => {
        events.forEach(event => {
          const toPublish = web3.fheDetected(
            {
              address: event.callerAddress.value,
              chainId: event.chainId.value,
              name: event.name,
              timestamp: event.timestamp.toISOString(),
            },
            {
              correlationId: randomUUID(),
            },
          )
          this.logger.log(
            `publishing ${toPublish.type}: ${JSON.stringify(toPublish.payload)}`,
          )
          this.pubsub.publish(toPublish)
        })
      })
  }
}
