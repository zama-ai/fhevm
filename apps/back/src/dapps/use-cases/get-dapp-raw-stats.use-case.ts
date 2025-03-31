import { PRODUCER } from '#constants.js'
import { DAppStat, DAppStatProps } from '#dapps/domain/entities/dapp-stat.js'
import { DApp } from '#dapps/domain/entities/dapp.js'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '#dapps/domain/repositories/dapp.repository.js'
import { IProducer } from '#shared/services/producer.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { randomUUID } from 'crypto'
import { back, generateRequestId } from 'messages'
import {
  AppError,
  LOCAL_FHEVM_CHAIN_ID,
  Task,
  UseCase,
  validationError,
} from 'utils'

type Input = {
  dappId: string
}

type Output = {
  stats: DAppStatProps[]
}

@Injectable()
export class GetDappRawStatsUseCase implements UseCase<Input, Output> {
  private readonly logger = new Logger(GetDappRawStatsUseCase.name)
  constructor(
    @Inject(PRODUCER) private readonly producer: IProducer,
    @Inject(DAPP_REPOSITORY) private readonly repo: DAppRepository,
  ) {}

  execute(input: Input): Task<Output, AppError> {
    this.logger.debug(`requested stats for dappId=${input.dappId}`)
    return DAppId.fromString(input.dappId)
      .asyncChain(dappId =>
        Task.all<AppError, DAppStat[], void>([
          this.repo.findAllStats(dappId).tap(stats => {
            this.logger.verbose(`stats: ${JSON.stringify(stats)}`)
          }),
          this.repo
            .findById(dappId)
            .chain<DApp>(dapp =>
              dapp.address
                ? Task.of(dapp)
                : Task.reject(validationError('missing dApp address')),
            )
            .chain<void>(dapp => {
              this.logger.debug(
                `publishing dappStatsRequested for dappId=${dappId}`,
              )
              this.logger.verbose(
                `publishing dappStatsRequested for dappId=${dappId} on chain ${LOCAL_FHEVM_CHAIN_ID}`,
              )
              return this.producer
                .publish(
                  back.dappStatsRequested(
                    {
                      // TODO: retrieve the `requestId` from the adapter
                      requestId: generateRequestId(),
                      dAppId: dappId.value,
                      // Note: We should store the chainId in the DApp entity
                      chainId: LOCAL_FHEVM_CHAIN_ID,
                      address: dapp.address!,
                    },
                    {
                      correlationId: randomUUID(),
                    },
                  ),
                )
                .orElse(() => {
                  this.logger.warn(
                    `failed to publish dappStatsRequested for dappId=${dappId}`,
                  )
                })
            }),
        ]),
      )
      .map(([stats]) => ({ stats: stats.map(stat => stat.toJSON()) }))
  }
}
