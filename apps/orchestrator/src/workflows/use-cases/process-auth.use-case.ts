import { PUBSUB } from '#constants.js'
import { isAuthEvent, Auth, AuthEvents } from '#workflows/entities/auth.js'
import {
  EVENT_PRODUCER,
  type EventProducer,
} from '#workflows/interfaces/event.producer.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { back, email } from 'messages'
import {
  AppError,
  type IPubSub,
  type ISubscriber,
  Task,
  type UseCase,
} from 'utils'

@Injectable()
export class ProcessAuth implements UseCase<AuthEvents, void> {
  private readonly logger = new Logger(ProcessAuth.name)

  constructor(
    @Inject(PUBSUB) private readonly pubsub: IPubSub<AuthEvents>,
    @Inject(EVENT_PRODUCER) private readonly producer: EventProducer,
  ) {
    this.logger.verbose(`subscribing to 'back:user:created' events`)
    this.pubsub.subscribe('back:user:created', this.handleEvent)
    this.logger.verbose(`subscribing to 'back:password-reset:*' events`)
    this.pubsub.subscribe('back:password-reset:*', this.handleEvent)
  }

  private handleEvent: ISubscriber<AuthEvents> = event => {
    this.logger.verbose(`event ${event.type} received`)
    return isAuthEvent(event) ? this.execute(event) : Task.of(void 0)
  }

  execute = (event: AuthEvents): Task<void, AppError> => {
    this.logger.debug(`processing ${event.type}`)
    return Task.of<(back.BackEvent | email.EmailEvent)[], AppError>(
      new Auth().send(event),
    )
      .chain(messages => {
        this.logger.verbose(`publishing #${messages.length} messages`)
        return Task.all(
          messages.map(message => {
            this.logger.verbose(
              `[${message.payload.requestId}] calling producer.publish(${message.type})`,
            )
            return this.producer.publish(message)
          }),
        )
      })
      .map(() => void 0)
  }
}
