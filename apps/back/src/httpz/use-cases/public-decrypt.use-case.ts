import { PRODUCER } from '#constants.js'
import {
  API_KEY_ALLOWS_REQUEST,
  IApiKeyAllowsRequest,
} from '#dapps/use-cases/api-key-allows-request.use-case.js'
import { IProducer } from '#shared/services/producer.js'
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
  unauthorizedError,
  unknownError,
  UseCase,
  validationError,
} from 'utils'
import ms from 'ms'
import { SyncInstances } from '#shared/use-cases/sync-instances.use-case.js'

type Input = {
  ciphertextHandles: string[]
}

const type = 'back:httpz:public-decrypt:completed' as const

type PublicDecryptResult = Extract<back.BackEvent, { type: typeof type }>
function isPublicDecryptResult(
  event: back.BackEvent,
): event is PublicDecryptResult {
  return event.type === type
}

type Output = PublicDecryptResult['payload']['response']

export const PUBLIC_DECRYPT = 'PUBLIC_DECRYPT'

export type IPublicDecrypt = UseCase<Input, Output>

@Injectable()
export class PublicDecrypt implements UseCase<Input, void> {
  private readonly logger = new Logger(PublicDecrypt.name)
  constructor(@Inject(PRODUCER) private readonly producer: IProducer) {}

  execute = (
    input: Input,
    context?: Record<string, unknown>,
  ): Task<void, AppError> => {
    this.logger.debug(`input=${JSON.stringify(input)}`)

    const requestId: string =
      (context?.requestId as string) ?? generateRequestId()

    return this.producer.publish(
      back.httpzPublicDecryptRequested(
        {
          requestId,
          ciphertextHandles: input.ciphertextHandles,
        },
        {
          correlationId: requestId,
        },
      ),
    )
  }
}

@Injectable()
export class PublicDecryptWithSync implements IPublicDecrypt {
  constructor(
    private readonly publicDecrypt: PublicDecrypt,
    @Inject(SYNC_SERVICE) private readonly syncService: SyncService,
    syncInstances: SyncInstances,
  ) {
    // Note: I need to instruct the SyncInstances to listen to this event
    syncInstances.listenToEvent('back:httpz:public-decrypt:completed')
  }

  execute = (
    input: Input,
    context?: Record<string, unknown>,
  ): Task<Output, AppError> => {
    const requestId: string =
      (context?.requestId as string) ?? generateRequestId()

    return Task.race([
      this.publicDecrypt.execute(input, { ...context, requestId }).chain(() =>
        this.syncService.waitForResponse<Output>(requestId, data => {
          if (back.isBackEvent(data) && isPublicDecryptResult(data)) {
            return Task.of<Output, AppError>(data.payload.response)
          }
          return Task.reject(unknownError('Invalid event received'))
        }),
      ),
      Task.timeout(ms('30s')),
    ])
  }
}

@Injectable()
export class PublicDecryptWithApiKey implements IPublicDecrypt {
  constructor(
    private readonly publicDecrypt: PublicDecryptWithSync,
    @Inject(API_KEY_ALLOWS_REQUEST)
    private readonly apiKeyAllowsRequest: IApiKeyAllowsRequest,
  ) {}

  execute = (
    input: Input,
    context?: Record<string, unknown>,
  ): Task<Output, AppError> => {
    return this.apiKeyAllowsRequest
      .execute('*', context)
      .chain(() => this.publicDecrypt.execute(input, context))
  }
}
