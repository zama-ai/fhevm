import { PRODUCER } from '#constants.js'
import { Address } from '#dapps/domain/entities/value-objects.js'
import { ChainId } from '#shared/entities/value-objects/chain-id.js'
import { SYNC_SERVICE, SyncService } from '#shared/services/sync.service.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { randomUUID } from 'crypto'
import { back, generateRequestId } from 'messages'
import {
  AppError,
  every,
  fromNullable,
  fromOption,
  Task,
  unknownError,
  UseCase,
  validationError,
} from 'utils'
import { SyncInstances } from '../../shared/use-cases/sync-instances.use-case.js'
import { IProducer } from '#shared/services/producer.js'

export type ValidateAddressInput = {
  chainId: string
  address: string
}

export type ValidateAddressOutput =
  | { check: true; message?: never }
  | { check: false; message: string }

export type IValidateAddress = UseCase<
  ValidateAddressInput,
  ValidateAddressOutput
>

export const VALIDATE_ADDRESS = 'VALIDATE_ADDRESS'

@Injectable()
export class ValidateAddress implements IValidateAddress {
  private readonly logger = new Logger(ValidateAddress.name)

  constructor(@Inject(PRODUCER) private readonly producer: IProducer) { }

  execute = (
    input: ValidateAddressInput,
    context?: Record<string, any>,
  ): Task<ValidateAddressOutput, AppError> => {
    return every([
      fromOption(
        fromNullable(context?.requestId).orElse(() => generateRequestId()),
        () => validationError('missing requestId'),
      ),
      ChainId.parse(input.chainId),
      Address.fromString(input.address),
    ])
      .asyncChain(([requestId, chainId, address]) => {
        this.logger.verbose(
          `publishing address validation for requestId=${requestId}`,
        )
        return this.producer.publish(
          back.addressValidationRequested(
            { chainId: chainId.value, address: address.value, requestId },
            { correlationId: randomUUID() },
          ),
        )
      })
      .map(() => ({ check: true }))
  }
}

@Injectable()
export class ValidateAddressWithSync implements IValidateAddress {
  private readonly logger = new Logger(ValidateAddressWithSync.name)
  constructor(
    private readonly validateAddress: ValidateAddress,
    @Inject(SYNC_SERVICE) private readonly syncService: SyncService,

    syncInstances: SyncInstances,
  ) {
    // Note: I need to instruct the SyncInstances to listen to this event
    syncInstances.listenToEvent('back:address:validation:confirmed')
    syncInstances.listenToEvent('back:address:validation:failed')
  }
  execute(
    input: ValidateAddressInput,
    context?: Record<string, any>,
  ): Task<ValidateAddressOutput, AppError> {
    return fromOption<string, AppError>(
      fromNullable<string>(context?.requestId).orElse(() =>
        generateRequestId(),
      ),
      () => validationError('missing requestId'),
    )
      .asyncChain<ValidateAddressOutput>(requestId =>
        Task.race([
          this.validateAddress
            .execute(input, { ...context, requestId })
            .chain(() => {
              this.logger.verbose(
                `waiting for response sync for requestId=${requestId}`,
              )
              return this.syncService.waitForResponse<ValidateAddressOutput>(
                requestId,
                data => {
                  this.logger.verbose(`received event: ${JSON.stringify(data)}`)
                  if (
                    back.isBackEvent(data) &&
                    isAddressValidationResponse(data)
                  ) {
                    return data.type === 'back:address:validation:confirmed'
                      ? Task.of({ check: true })
                      : Task.of({
                        check: false,
                        message: data.payload.reason,
                      })
                  }
                  this.logger.warn(
                    `invalid event received: ${JSON.stringify(data)}`,
                  )
                  return Task.reject(unknownError('Invalid event received'))
                },
              )
            }),
          Task.timeout(parseInt(process.env.DEFAULT_TIMEOUT ?? '30', 10)),
        ]),
      )
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
