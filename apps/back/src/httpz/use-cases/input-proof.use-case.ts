import { PRODUCER } from '#constants.js'
import {
  API_KEY_ALLOWS_REQUEST,
  type IApiKeyAllowsRequest,
} from '#dapps/use-cases/api-key-allows-request.use-case.js'
import { ChainId } from '#shared/entities/value-objects/chain-id.js'
import { Web3Address } from '#shared/entities/value-objects/web3-address.js'
import { IProducer } from '#shared/services/producer.js'
import { SYNC_SERVICE, SyncService } from '#shared/services/sync.service.js'
import { SyncInstances } from '#shared/use-cases/sync-instances.use-case.js'
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

type Input = {
  contractChainId: string | number
  contractAddress: string
  userAddress: string
  ciphertextWithInputVerification: string
}

type Output = {
  handles: string[]
  signatures: string[]
}

export type IInputProof = UseCase<Input, Output>

export const INPUT_PROOF = 'INPUT_PROOF'

@Injectable()
export class InputProof implements IInputProof {
  private readonly logger = new Logger(InputProof.name)
  constructor(
    @Inject(PRODUCER)
    private readonly producer: IProducer,
  ) { }

  execute = (
    input: Input,
    context?: Record<string, any>,
  ): Task<Output, AppError> => {
    this.logger.debug(`input=${JSON.stringify(input)}`)

    return every<string, ChainId, Web3Address, Web3Address, AppError>([
      fromOption(
        fromNullable<string>(context?.requestId).orElse<string>(() =>
          generateRequestId(),
        ),
        () => validationError('missing requestId'),
      ),
      ChainId.parse(input.contractChainId),
      Web3Address.parse(input.contractAddress),
      Web3Address.parse(input.userAddress),
    ]).asyncChain(
      ([requestId, contractChainId, contractAddress, userAddress]) => {
        this.logger.verbose(`executing for ${context?.requestId}`)

        return this.producer
          .publish(
            back.httpzInputProofRequested(
              {
                requestId,
                contractChainId: contractChainId.value,
                contractAddress: contractAddress.value,
                userAddress: userAddress.value,
                ciphertextWithInputVerification: input.ciphertextWithInputVerification,
              },
              {
                correlationId: randomUUID(),
              },
            ),
          )
          .map(() => ({ handles: [], signatures: [] }))
      },
    )
  }
}

/**
 * It decorates the InputProof use case with the synchronization logic.
 */
@Injectable()
export class InputProofWithSync implements IInputProof {
  constructor(
    private readonly inputProof: InputProof,
    @Inject(SYNC_SERVICE) private readonly syncService: SyncService,
    syncInstances: SyncInstances,
  ) {
    // Note: I need to instruct the SyncInstances to listen to this event
    syncInstances.listenToEvent('back:httpz:input-proof:completed')
  }

  execute(input: Input, context?: Record<string, any>): Task<Output, AppError> {
    return fromOption<string, AppError>(
      fromNullable<string>(context?.requestId).orElse(() =>
        generateRequestId(),
      ),
      () => validationError('missing requestId'),
    ).asyncChain(requestId =>
      Task.race([
        this.inputProof
          .execute(input, {
            ...context,
            requestId,
          })
          .chain(() =>
            this.syncService.waitForResponse<Output>(requestId, data => {
              if (back.isBackEvent(data) && isInputProofResult(data)) {
                return Task.of<Output, AppError>({
                  handles: data.payload.handles,
                  signatures: data.payload.signatures,
                })
              }
              return Task.reject(unknownError('Invalid event received'))
            }),
          ),
        Task.timeout(parseInt(process.env.DEFAULT_TIMEOUT ?? '30', 10)),
      ]),
    )
  }
}

type InputProofResult = Extract<
  back.BackEvent,
  { type: 'back:httpz:input-proof:completed' }
>

function isInputProofResult(event: back.BackEvent): event is InputProofResult {
  return event.type === 'back:httpz:input-proof:completed'
}

/**
 * It decorates the InputProof use case with the authorization logic.
 */
@Injectable()
export class InputProofWithApiKey implements IInputProof {
  private readonly logger = new Logger(InputProofWithApiKey.name)
  constructor(
    private readonly inputProof: InputProofWithSync,
    @Inject(API_KEY_ALLOWS_REQUEST)
    private readonly apiKeyAllowsRequest: IApiKeyAllowsRequest,
  ) { }

  execute(input: Input, context?: Record<string, any>): Task<Output, AppError> {
    return this.apiKeyAllowsRequest
      .execute({
        apiKey: context?.apiKey,
        chainId: input.contractChainId,
        address: input.contractAddress,
      })
      .tap(() => {
        this.logger.debug(`apiKey=${context?.apiKey}}`)
      })
      .chain(() => this.inputProof.execute(input, context))
  }
}
