import { PUBSUB } from '#constants.js'
import { Address } from '#dapps/domain/entities/value-objects.js'
import { Inject, Injectable } from '@nestjs/common'
import { randomUUID } from 'crypto'
import { back, generateRequestId } from 'messages'
import { AppError, IPubSub, ISubscriber, Task, UseCase } from 'utils'

export type ValidateAddressInput = {
  chainId: string
  address: string
}

export type ValidateAddressOutput =
  | { check: true; message?: never }
  | { check: false; message: string }

@Injectable()
export class ValidateAddress
  implements UseCase<ValidateAddressInput, ValidateAddressOutput>
{
  constructor(
    @Inject(PUBSUB) private readonly pubsub: IPubSub<back.BackEvent>,
  ) {}

  execute = (
    input: ValidateAddressInput,
  ): Task<ValidateAddressOutput, AppError> => {
    return Address.fromString(input.address).asyncChain(address =>
      Task.race<AppError, ValidateAddressOutput>([
        new Task<ValidateAddressOutput, AppError>(resolve => {
          const handler: ISubscriber<back.BackEvent> = event => {
            switch (event.type) {
              case 'back:address:validation:confirmed':
                if (
                  event.payload.chainId === input.chainId &&
                  event.payload.address === address.value
                ) {
                  this.pubsub.unsubscribe('back:address:validation:*', handler)
                  resolve({ check: true })
                }
                break

              case 'back:address:validation:failed':
                if (
                  event.payload.chainId === input.chainId &&
                  event.payload.address === address.value
                ) {
                  this.pubsub.unsubscribe('back:address:validation:*', handler)
                  resolve({ check: false, message: event.payload.reason })
                }
                break
            }
            return Task.of(void 0)
          }
          this.pubsub.subscribe('back:address:validation:*', handler)
          // Note: retrieve the correlationId & requestId from the request
          this.pubsub.publish(
            back.addressValidationRequested(
              { ...input, requestId: generateRequestId() },
              { correlationId: randomUUID() },
            ),
          )
        }),
        Task.timeout<ValidateAddressOutput>(1),
      ]),
    )
  }
}
