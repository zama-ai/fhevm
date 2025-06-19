import { EVENT_PRODUCER, PUBSUB } from '#constants.js'
import {
  AddressValidation,
  AddressValidationEvents,
  isAddressValidationEvent,
} from '#workflows/entities/address-validation.js'
import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { back, web3 } from 'messages'
import {
  type AppError,
  type ISubscriber,
  Task,
  type IPubSub,
  type UseCase,
} from 'utils'

@Injectable()
export class ProcessAddressValidation
  implements UseCase<back.BackEvent | web3.Web3Event, void>
{
  private readonly logger = new Logger(ProcessAddressValidation.name)

  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: IPubSub<back.BackEvent | web3.Web3Event>,
    @Inject(EVENT_PRODUCER) private readonly producer: EventProducer,
  ) {
    this.pubsub.subscribe('back:address:validation:requested', this.handleEvent)
    this.pubsub.subscribe('web3:contract:validation:*', this.handleEvent)
  }

  private handleEvent: ISubscriber<back.BackEvent | web3.Web3Event> = event => {
    this.logger.verbose(`event ${event.type} received`)
    return isAddressValidationEvent(event)
      ? this.execute(event)
      : Task.of<void, AppError>(void 0)
  }

  execute = (event: AddressValidationEvents): Task<void, AppError> => {
    this.logger.debug(`processing ${event.type}`)
    return Task.of<AddressValidation, AppError>(new AddressValidation())
      .map(addressValidation => addressValidation.send(event))
      .chain(messages => {
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
