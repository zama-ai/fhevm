import { PRODUCER } from '#constants.js'
import { ApiKeyAllowsRequest } from '#dapps/use-cases/api-key-allows-request.use-case.js'
import { ChainId } from '#shared/entities/value-objects/chain-id.js'
import { Web3Address } from '#shared/entities/value-objects/web3-address.js'
import { IProducer } from '#shared/services/producer.js'
import { SYNC_SERVICE, SyncService } from '#shared/services/sync.service.js'
import { SyncInstances } from '#shared/use-cases/sync-instances.use-case.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { randomUUID } from 'crypto'
import { back, generateRequestId } from 'messages'
import { AppError, every, Task, unknownError, UseCase } from 'utils'

type Input = {
  contractChainId: string | number
  contractAddress: string
  userAddress: string
  ciphertextWithZkpok: string
}

type Output = {
  handles: string[]
  signatures: string[]
}

@Injectable()
export class InputProof implements UseCase<Input, Output> {
  private readonly logger = new Logger(InputProof.name)
  constructor(
    @Inject(PRODUCER)
    private readonly producer: IProducer,
    @Inject(SYNC_SERVICE)
    private readonly syncService: SyncService,
    private readonly apiKeyAllowsRequest: ApiKeyAllowsRequest,
    syncInstances: SyncInstances,
  ) {
    // Note: I need to instruct the SyncInstances to listen to this event
    syncInstances.listenToEvent('back:httpz:input-proof:completed')
  }

  execute = (
    input: Input,
    context: Record<string, any>,
  ): Task<Output, AppError> => {
    this.logger.debug(`input=${JSON.stringify(input)}`)

    return this.apiKeyAllowsRequest
      .execute({
        apiKey: context.apiKey,
        chainId: input.contractChainId,
        address: input.contractAddress,
      })
      .tap(() => {
        this.logger.debug(`apiKey=${context.apiKey}}`)
      })
      .chain(() =>
        every<ChainId, Web3Address, Web3Address, AppError>([
          ChainId.parse(input.contractChainId),
          Web3Address.parse(input.contractAddress),
          Web3Address.parse(input.userAddress),
        ]).asyncChain(([contractChainId, contractAddress, userAddress]) => {
          const requestId = generateRequestId()
          this.logger.verbose(`executing for ${requestId}`)

          return Task.race([
            this.producer
              .publish(
                back.httpzInputProofRequested(
                  {
                    requestId,
                    contractChainId: contractChainId.value,
                    contractAddress: contractAddress.value,
                    userAddress: userAddress.value,
                    ciphertextWithZkpok: input.ciphertextWithZkpok,
                  },
                  {
                    correlationId: randomUUID(),
                  },
                ),
              )
              .chain(() =>
                this.syncService.waitForResponse<Output>(requestId, data => {
                  if (back.isBackEvent(data) && isInputProofResult(data)) {
                    return Task.of<Output, AppError>({
                      handles: data.payload.handles,
                      signatures: data.payload.signatures,
                    })
                  }
                  return Task.reject(unknownError('Invalid evnet received'))
                }),
              ),
            Task.timeout(parseInt(process.env.DEFAULT_TIMEOUT ?? '30', 10)),
          ])
        }),
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
