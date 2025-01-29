import { Logger } from '@nestjs/common'
import {
  AppDeploymentCommand,
  discoverSC,
  scDiscovered,
  scDiscoveryFailed,
} from 'messages'
import { ContractService } from '#domain/services/contract.service.js'
import { MessageProducer } from '#domain/services/message.producer.js'
import { AppError, Task, UseCase } from 'utils'
import { Web3Address } from '#domain/entities/value-objects.js'

type Input = Extract<
  AppDeploymentCommand,
  { type: 'app-deployment.discover-sc' }
>

// TODO: we should move this parameter to the cain level
const MAX_RETRY = 5
const RETRY_DELAY_RATIO = 2

export class DiscoverContract implements UseCase<Input, void> {
  logger = new Logger(DiscoverContract.name)

  constructor(
    private readonly service: ContractService,
    private readonly producer: MessageProducer,
  ) {}

  execute({
    payload: { chainId, address, ...payload },
    $meta,
  }: Input): Task<void, AppError> {
    return Web3Address.fromString(address)
      .asyncChain(address => this.service.getContractCreation(chainId, address))
      .chain(data =>
        this.producer.produce(
          scDiscovered(
            {
              ...payload,
              contractAddress: data.contractAddress.value,
              creatorAddress: data.creatorAddress.value,
            },
            $meta,
          ),
        ),
      )
      .match({
        ok: message => {
          this.logger.debug(message)
        },
        fail: err => {
          this.logger.warn(
            `Failed to verify ${address} on chain ${chainId}: ${JSON.stringify(err)}`,
          )

          const retry = Number($meta?.retry ?? 0)
          const delay = Number($meta?.retry ?? 60)

          this.producer.produce(
            retry >= MAX_RETRY
              ? discoverSC(
                  { ...payload, address, chainId },
                  {
                    ...$meta,
                    retry: retry + 1,
                    delay: delay * RETRY_DELAY_RATIO,
                  },
                )
              : scDiscoveryFailed(payload, $meta),
          )
        },
      })
  }
}
