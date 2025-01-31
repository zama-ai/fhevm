import { PUBSUB } from '#constants.js'
import { DAppStat, DAppStatProps } from '#dapps/domain/entities/dapp-stat.js'
import { DApp } from '#dapps/domain/entities/dapp.js'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import { DAppRepository } from '#dapps/domain/repositories/dapp.repository.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { randomUUID } from 'crypto'
import { back } from 'messages'
import { AppError, PubSub, Task, UseCase, validationError } from 'utils'

type Input = {
  dappId: string
}

type Output = {
  stats: DAppStatProps[]
}

@Injectable()
export class GetDappStatsUseCase implements UseCase<Input, Output> {
  private readonly logger = new Logger(GetDappStatsUseCase.name)
  constructor(
    @Inject(PUBSUB) private readonly pubsub: PubSub<back.BackEvent>,
    private readonly repo: DAppRepository,
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
              console.log(`publishing dappStatsRequested for dappId=${dappId}`)
              this.logger.debug(
                `publishing dappStatsRequested for dappId=${dappId}`,
              )
              return this.pubsub
                .publish(
                  back.dappStatsRequested(
                    {
                      // Note: We should store the chainId in the DApp entity
                      chainId: '12345',
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
