import { PRODUCER } from '#constants.js'
import { Address } from '#dapps/domain/entities/value-objects.js'
import { ChainId } from '#shared/entities/value-objects/chain-id.js'
import { SYNC_SERVICE, SyncService } from '#shared/services/sync.service.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { randomUUID } from 'crypto'
import { back, generateRequestId } from 'messages'
import { AppError, every, Task, unknownError, UseCase } from 'utils'
import { SyncInstances } from '../../shared/use-cases/sync-instances.use-case.js'
import { IProducer } from '#shared/services/producer.js'

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
  private readonly logger = new Logger(ValidateAddress.name)

  constructor(
    @Inject(PRODUCER) private readonly producer: IProducer,
    @Inject(SYNC_SERVICE) private readonly syncService: SyncService,

    syncInstances: SyncInstances,
  ) {
    // Note: I need to instruct the SyncInstances to listen to this event
    syncInstances.listenToEvent('back:address:validation:confirmed')
    syncInstances.listenToEvent('back:address:validation:failed')
  }

  execute = (
    input: ValidateAddressInput,
  ): Task<ValidateAddressOutput, AppError> => {
    return every([
      ChainId.parse(input.chainId),
      Address.fromString(input.address),
    ])
      .asyncChain(([chainId, address]) => {
        const requestId = generateRequestId()
        this.logger.verbose(
          `publishing address validation for requestId=${requestId}`,
        )
        return this.producer
          .publish(
            back.addressValidationRequested(
              { chainId: chainId.value, address: address.value, requestId },
              { correlationId: randomUUID() },
            ),
          )
          .chain(() => Task.of(requestId))
      })
      .chain(requestId => {
        this.logger.verbose(
          `waiting for response sync for requestId=${requestId}`,
        )
        return this.syncService.waitForResponse<ValidateAddressOutput>(
          requestId,
          data => {
            if (back.isBackEvent(data) && isAddressValidationResponse(data)) {
              return data.type === 'back:address:validation:confirmed'
                ? Task.of({ check: true })
                : Task.of({
                    check: false,
                    message: data.payload.reason,
                  })
            }
            this.logger.warn(`invalid event received: ${JSON.stringify(data)}`)
            return Task.reject(unknownError('Invalid event received'))
          },
        )
      })
      .tap(value => {
        this.logger.debug(`value=${JSON.stringify(value)}`)
      })
      .tapError(error => {
        this.logger.warn(
          `failed to validate address: ${error._tag}/${error.message}`,
        )
      })
  }
}

const EVENT_TYPES = [
  'back:address:validation:confirmed',
  'back:address:validation:failed',
] as const

type AddressValidationResponse = Extract<
  back.BackEvent,
  { type: (typeof EVENT_TYPES)[number] }
>

function isAddressValidationResponse(
  event: back.BackEvent,
): event is AddressValidationResponse {
  return (EVENT_TYPES as readonly string[]).includes(event.type)
}
