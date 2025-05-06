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

type RequestValidity = {
  startTimestamp: string
  durationDays: string
}

type CtHandleContractPair = {
  ctHandle: string
  contractAddress: string
}

type Input = {
  contractsChainId: string | number
  ctHandleContractPairs: CtHandleContractPair[]
  requestValidity: RequestValidity
  contractsAddresses: string[]
  userAddress: string,
  signature: string,
  publicKey: string,
}

type Output = {
  gatewayRequestId: number
  decryptedValue: string
  signatures: string[]
}

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
    private readonly apiKeyAllowsRequest: ApiKeyAllowsRequest,
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

    return this.apiKeyAllowsRequest
      .execute({
        apiKey: context.apiKey,
        // FIXME: change this to consider all couples chain-id, contract-address
        // TODO: add input verification that both attributes have the same length
        chainId: input.contractsChainId,
        address: input.contractsAddresses[0],
      })
      .tap(() => {
        this.logger.debug(`apiKey=${context.apiKey}}`)
      })
      .chain(() => {
        // every<ChainId, Web3Address, Web3Address, AppError>([
        //   ChainId.parse(input.contractChainId),
        //   Web3Address.parse(input.contractAddress),
        //   Web3Address.parse(input.userAddress),
        // ]).asyncChain(([contractChainId, contractAddress, userAddress]) => {
        const requestId = generateRequestId()
        this.logger.verbose(`executing for ${requestId}`)

        return Task.race([
          this.producer
            .publish(
              back.httpzPrivateDecryptRequested(
                {
                  requestId,
                  ...input
                },
                {
                  correlationId: randomUUID(),
                },
              ),
            )
            .chain(() =>
              this.syncService.waitForResponse<Output>(requestId, data => {
                if (back.isBackEvent(data) && isPrivateDecryptResult(data)) {
                  return Task.of<Output, AppError>({
                    ...data.payload
                  })
                }
                return Task.reject(unknownError('Invalid evnet received'))
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

function isPrivateDecryptResult(event: back.BackEvent): event is PrivateDecryptResult {
  return event.type === 'back:httpz:private-decrypt:completed'
}
