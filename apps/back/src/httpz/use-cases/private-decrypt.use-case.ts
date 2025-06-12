import { ChainId } from '#chains/domain/entities/value-objects.js'
import { PRODUCER } from '#constants.js'
import {
  API_KEY_ALLOWS_REQUEST,
  IApiKeyAllowsRequest,
} from '#dapps/use-cases/api-key-allows-request.use-case.js'
import { Web3Address } from '#shared/entities/value-objects/web3-address.js'
import { IProducer } from '#shared/services/producer.js'
import { SYNC_SERVICE, SyncService } from '#shared/services/sync.service.js'
import { SyncInstances } from '#shared/use-cases/sync-instances.use-case.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { randomUUID } from 'crypto'
import { back, generateRequestId } from 'messages'
import { any, AppError, every, Task, unknownError, UseCase } from 'utils'

type RequestValidity = {
  startTimestamp: string
  durationDays: string
}

type HandleContractPair = {
  handle: string
  contractAddress: string
}

type Input = {
  contractsChainId: string | number
  handleContractPairs: HandleContractPair[]
  requestValidity: RequestValidity
  contractAddresses: string[]
  userAddress: string
  signature: string
  publicKey: string
}

type UserDecryptResponse = {
  payload: string,
  signature: string,
}

type Output = UserDecryptResponse[]

export type IPrivateDecrypt = UseCase<Input, Output>

export const PRIVATE_DECRYPT = 'PRIVATE_DECRYPT'

// TODO: add private decrypt to event

@Injectable()
export class PrivateDecrypt implements UseCase<Input, Output> {
  private readonly logger = new Logger(PrivateDecrypt.name)
  constructor(
    @Inject(PRODUCER)
    private readonly producer: IProducer,
    @Inject(SYNC_SERVICE)
    private readonly syncService: SyncService,
    @Inject(API_KEY_ALLOWS_REQUEST)
    private readonly apiKeyAllowsRequest: IApiKeyAllowsRequest,
    syncInstances: SyncInstances,
  ) {
    // NOTE: I need to instruct the SyncInstances to listen to this event
    syncInstances.listenToEvent('back:httpz:private-decrypt:completed')
  }

  execute = (
    input: Input,
    context: Record<string, any>,
  ): Task<Output, AppError> => {
    this.logger.debug(`input=${JSON.stringify(input)}`)

    return every([
      typeof input.contractsChainId === 'string'
        ? any([
          ChainId.fromString(input.contractsChainId),
          ChainId.fromHex(input.contractsChainId),
        ])
        : ChainId.from(input.contractsChainId),
      Web3Address.from(input.contractAddresses[0]),
    ])
      .asyncChain(([chainId, address]) =>
        this.apiKeyAllowsRequest
          .execute(
            {
              // FIXME: change this to consider all couples chain-id, contract-address
              // TODO: add input verification that both attributes have the same length
              chainId,
              address,
            },
            context,
          )
          .map(() => chainId),
      )
      .tap(() => {
        this.logger.debug(`apiKey=${context.apiKey}}`)
      })
      .chain(chainId => {
        const requestId = generateRequestId()
        this.logger.verbose(`executing for ${requestId}`)

        return Task.race([
          this.producer
            .publish(
              back.httpzPrivateDecryptRequested(
                {
                  requestId,
                  ...input,
                  contractsChainId: chainId.value,
                },
                {
                  correlationId: randomUUID(),
                },
              ),
            )
            .chain(() =>
              this.syncService.waitForResponse<Output>(requestId, data => {
                if (back.isBackEvent(data) && isPrivateDecryptResult(data)) {
                  return Task.of<Output, AppError>(
                    data.payload.response,
                  )
                }
                return Task.reject(unknownError('Invalid event received'))
              }),
            ),
          Task.timeout(parseInt(process.env.DEFAULT_TIMEOUT ?? '30', 10)),
        ])
        // }),
        // )
      })
  }
}

type PrivateDecryptResult = Extract<
  back.BackEvent,
  { type: 'back:httpz:private-decrypt:completed' }
>

function isPrivateDecryptResult(
  event: back.BackEvent,
): event is PrivateDecryptResult {
  return event.type === 'back:httpz:private-decrypt:completed'
}
