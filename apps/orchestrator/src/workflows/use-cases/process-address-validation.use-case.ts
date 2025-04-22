import {
  AddressValidation,
  AddressValidationEvents,
  isAddressValidationEvent,
} from '#workflows/entities/address-validation.js'
import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { Logger } from '@nestjs/common'
import { back, web3 } from 'messages'
import {
  type AppError,
  type ISubscriber,
  Task,
  type IPubSub,
  type UseCase,
} from 'utils'

export class ProcessAddressValidation
  implements UseCase<back.BackEvent | web3.Web3Event, void>
{
  private readonly logger = new Logger(ProcessAddressValidation.name)

  constructor(
    private readonly pupsub: IPubSub<back.BackEvent | web3.Web3Event>,
    private readonly producer: EventProducer,
  ) {
    this.pupsub.subscribe('*', this.handleEvent)
  }

  private handleEvent: ISubscriber<back.BackEvent | web3.Web3Event> = event => {
    return isAddressValidationEvent(event)
      ? this.execute(event)
      : Task.of<void, AppError>(void 0)
  }

  execute = (event: AddressValidationEvents): Task<void, AppError> => {
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
