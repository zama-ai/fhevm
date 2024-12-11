import { Logger } from '@nestjs/common'
import {
  AppDeploymentCommand,
  discoverSC,
  scDiscovered,
  scDiscoveryFailed,
} from 'messages'
import { Address } from 'src/domain/entities/address'
import { ContractService } from 'src/domain/services/contract.service'
import { MessageProducer } from 'src/domain/services/message.producer'
import { AppError, Task, UseCase } from 'utils'

type Input = Extract<
  AppDeploymentCommand,
  { type: 'app-deployment.discover-sc' }
>

// TODO: we should move this parameter to the cain level
const MAX_RETRY = 5
const RETRY_DELAY_RATIO = 2

export class VerifyContract implements UseCase<Input, void> {
  logger = new Logger(VerifyContract.name)

  constructor(
    private readonly service: ContractService,
    private readonly producer: MessageProducer,
  ) {}

  execute({
    payload: { chainId, address, ...payload },
    $meta,
  }: Input): Task<void, AppError> {
    return Address.fromString(address)
      .asyncChain(address => this.service.getAbi(chainId, address))
      .match({
        ok: () => {
          this.producer.produce(scDiscovered(payload, $meta))
        },
        fail: err => {
          this.logger.warn(
            `Failed to verify ${address} on chain ${chainId}: ${err}`,
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
