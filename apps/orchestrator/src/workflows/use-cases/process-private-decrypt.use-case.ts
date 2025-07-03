import { PUBSUB } from '#constants.js'
import {
  isPrivateDecrypt,
  PrivateDecrypt,
} from '#workflows/entities/private-decrypt.js'
import {
  EVENT_PRODUCER,
  type EventProducer,
} from '#workflows/interfaces/event.producer.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { back, relayer } from 'messages'
import { AppError, IPubSub, shortString, Task, UseCase } from 'utils'

@Injectable()
export class ProcessPrivateDecrypt
  implements UseCase<back.BackEvent | relayer.RelayerEvent, void>
{
  private readonly logger = new Logger(ProcessPrivateDecrypt.name)

  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: IPubSub<back.BackEvent | relayer.RelayerEvent>,
    @Inject(EVENT_PRODUCER) private readonly producer: EventProducer,
  ) {
    this.pubsub.subscribe('back:httpz:private-decrypt:requested', this.execute)
    this.pubsub.subscribe(
      'relayer:private-decryption:operation-response',
      this.execute,
    )
  }

  execute = (
    event: back.BackEvent | relayer.RelayerEvent,
  ): Task<void, AppError> => {
    this.logger.verbose(`event ${event.type} received`)
    if (isPrivateDecrypt(event)) {
      this.logger.debug(
        `processing ${event.type}: ${JSON.stringify(event.payload, (_, v) =>
          typeof v === 'string' ? shortString(v) : v,
        )}`,
      )
      return Task.of<PrivateDecrypt, AppError>(new PrivateDecrypt())
        .map(privateDecrypt => privateDecrypt.send(event))
        .chain(messages =>
          Task.all(messages.map(message => this.producer.publish(message))),
        )
        .map(() => void 0)
    }
    return Task.of(void 0)
  }
}
