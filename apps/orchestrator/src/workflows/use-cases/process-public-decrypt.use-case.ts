import { EVENT_PRODUCER, PUBSUB } from '#constants.js'
import {
  isPublicDecrypt,
  PublicDecrypt,
} from '#workflows/entities/public-decrypt.js'
import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { back, relayer } from 'messages'
import { AppError, IPubSub, Task, UseCase } from 'utils'

@Injectable()
export class ProcessPublicDecrypt
  implements UseCase<back.BackEvent | relayer.RelayerEvent, void>
{
  private readonly logger = new Logger(ProcessPublicDecrypt.name)

  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: IPubSub<back.BackEvent | relayer.RelayerEvent>,
    @Inject(EVENT_PRODUCER) private readonly producer: EventProducer,
  ) {
    this.pubsub.subscribe('back:httpz:public-decrypt:requested', this.execute)
    this.pubsub.subscribe(
      'relayer:public-decryption:operation-response',
      this.execute,
    )
  }

  execute = (
    event: back.BackEvent | relayer.RelayerEvent,
  ): Task<void, AppError> => {
    if (isPublicDecrypt(event)) {
      this.logger.debug(
        `processing ${event.type}: ${JSON.stringify(event.payload)}`,
      )
      return Task.of<PublicDecrypt, AppError>(new PublicDecrypt())
        .map(privateDecrypt => privateDecrypt.send(event))
        .chain(messages =>
          Task.all(messages.map(message => this.producer.publish(message))),
        )
        .map(() => void 0)
    }
    return Task.of(void 0)
  }
}
