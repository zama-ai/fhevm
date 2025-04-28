import { PUBSUB } from '#constants.js'
import { SYNC_SERVICE, SyncService } from '#shared/services/sync.service.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { back } from 'messages'
import { AppError, IPubSub, Task, UseCase } from 'utils'

@Injectable()
export class SyncInstances implements UseCase<back.BackEvent, void> {
  private readonly logger = new Logger(SyncInstances.name)
  private readonly eventsNames: back.BackEvent['type'][] = []

  constructor(
    @Inject(PUBSUB) private readonly pubsub: IPubSub<back.BackEvent>,
    @Inject(SYNC_SERVICE) private readonly syncService: SyncService,
  ) {}

  listenToEvent = (eventName: back.BackEvent['type']): void => {
    if (!this.eventsNames.includes(eventName)) {
      this.logger.debug(`listening to event ${eventName}`)
      this.eventsNames.push(eventName)
      this.pubsub.subscribe(eventName, this.execute)
    }
  }

  execute = (
    event: back.BackEvent,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, any>,
  ): Task<void, AppError> => {
    this.logger.verbose(`event ${event.type} received`)

    if (this.eventsNames.includes(event.type)) {
      this.logger.debug(
        `Syncing instance for requestId=${event.payload.requestId}`,
      )
      return this.syncService.publishResponse(event.payload.requestId, event)
    }
    this.logger.verbose(`ignoring event ${event.type}`)
    return Task.of<void, AppError>(void 0)
  }
}
