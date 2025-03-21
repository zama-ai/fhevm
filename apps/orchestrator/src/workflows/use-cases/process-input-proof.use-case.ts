import { EVENT_PRODUCER, PUBSUB } from '#constants.js'
import {
  InputProof,
  isInputProofEvent,
} from '#workflows/entities/input-proof.js'
import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { back, relayer } from 'messages'
import { AppError, IPubSub, Task, UseCase } from 'utils'

@Injectable()
export class ProcessInputProof
  implements UseCase<back.BackEvent | relayer.RelayerEvent, void>
{
  private readonly logger = new Logger(ProcessInputProof.name)

  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: IPubSub<back.BackEvent | relayer.RelayerEvent>,
    @Inject(EVENT_PRODUCER) private readonly producer: EventProducer,
  ) {
    this.pubsub.subscribe('back:httpz:input-proof:requested', this.execute)
    this.pubsub.subscribe(
      'relayer:input-registration:input-registration-response',
      this.execute,
    )
  }

  execute = (
    event: back.BackEvent | relayer.RelayerEvent,
  ): Task<void, AppError> => {
    console.log(
      `ProcessInputProof> ${event.type} [${isInputProofEvent(event)}]`,
    )
    if (isInputProofEvent(event)) {
      return Task.of<InputProof, AppError>(new InputProof())
        .map(inputProof => inputProof.send(event))
        .chain(messages =>
          Task.all(messages.map(message => this.producer.publish(message))),
        )
        .map(() => void 0)
    }
    return Task.of(void 0)
  }
}
