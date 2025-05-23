import { EVENT_PRODUCER, PUBSUB } from '#constants.js'
import {
  isPasswordResetEvent,
  PasswordReset,
  PasswordResetEvents,
} from '#workflows/entities/password-reset.js'
import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { back, email } from 'messages'
import { AppError, IPubSub, ISubscriber, Task, UseCase } from 'utils'

@Injectable()
export class ProcessPasswordReset
  implements UseCase<PasswordResetEvents, void>
{
  private readonly logger = new Logger(ProcessPasswordReset.name)

  constructor(
    @Inject(PUBSUB) private readonly pubsub: IPubSub<PasswordResetEvents>,
    @Inject(EVENT_PRODUCER) private readonly producer: EventProducer,
  ) {
    this.pubsub.subscribe('back:user:password-reset:*', this.handleEvent)
  }

  private handleEvent: ISubscriber<PasswordResetEvents> = event => {
    this.logger.verbose(`event ${event.type} received`)
    return isPasswordResetEvent(event) ? this.execute(event) : Task.of(void 0)
  }

  execute = (event: PasswordResetEvents): Task<void, AppError> => {
    this.logger.debug(`processing ${event.type}`)
    return Task.of<(back.BackEvent | email.EmailEvent)[], AppError>(
      new PasswordReset().send(event),
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
