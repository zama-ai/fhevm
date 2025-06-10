import { PUBSUB } from '#constants.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { Task, type IPubSub, type ISubscriber, type UseCase } from 'utils'
import { email } from 'messages'
import {
  PASSWORD_RESET_REQUESTED_PRODUCER,
  PasswordResetRequested,
  PasswordResetRequestedProducer,
} from './gateways/password-reset-requested.producer.js'

@Injectable()
export class ProcessPasswordReset
  implements UseCase<PasswordResetRequested, void>
{
  private readonly logger = new Logger(ProcessPasswordReset.name)

  constructor(
    @Inject(PUBSUB) private readonly pubsub: IPubSub<email.EmailEvent>,
    @Inject(PASSWORD_RESET_REQUESTED_PRODUCER)
    private readonly producer: PasswordResetRequestedProducer,
  ) {
    this.logger.debug('subscribing to email:password-reset:requested')
    this.pubsub.subscribe('email:password-reset:requested', this.execute)
  }

  execute: ISubscriber<email.EmailEvent> = event => {
    if (isPasswordResetRequested(event)) {
      this.logger.log(`event ${event.type} received for ${event.payload.email}`)
      return this.producer.produce(event)
    }
    return Task.of(void 0)
  }
}

function isPasswordResetRequested(
  event: email.EmailEvent,
): event is PasswordResetRequested {
  return event.type === 'email:password-reset:requested'
}
