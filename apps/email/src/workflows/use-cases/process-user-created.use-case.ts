import { Inject, Injectable, Logger } from '@nestjs/common'
import { type IPubSub, type ISubscriber, Task, type UseCase } from 'utils'
import {
  isUserCreated,
  USER_CREATED_PRODUCER,
  UserCreated,
  UserCreatedProducer,
} from './gateways/user-created.producer.js'
import { PUBSUB } from '#constants.js'
import { email } from 'messages'

@Injectable()
export class ProcessUserCreated implements UseCase<UserCreated, void> {
  private readonly logger = new Logger(ProcessUserCreated.name)

  constructor(
    @Inject(PUBSUB) private readonly pubsub: IPubSub<email.EmailEvent>,
    @Inject(USER_CREATED_PRODUCER)
    private readonly producer: UserCreatedProducer,
  ) {
    this.logger.debug('subscribing to email:user:created')
    this.pubsub.subscribe('email:user:created', this.execute)
  }

  execute: ISubscriber<email.EmailEvent> = event => {
    if (isUserCreated(event)) {
      this.logger.log(`event ${event.type} received for ${event.payload.email}`)
      return this.producer.produce(event)
    }
    return Task.of(void 0)
  }
}
