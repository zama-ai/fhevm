import { EVENT_PRODUCER, PUBSUB } from '#constants.js'
import {
  InputProof,
  isInputProofEvent,
} from '#workflows/entities/input-proof.js'
import {
  isPrivateDecrypt,
  PrivateDecrypt,
} from '#workflows/entities/private-decrypt.js'
import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { privateDecrypt } from 'crypto'
import { back, relayer } from 'messages'
import { AppError, IPubSub, Task, UseCase } from 'utils'

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
    if (isPrivateDecrypt(event)) {
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
