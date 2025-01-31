import { DAppStat } from '#dapps/domain/entities/dapp-stat.js'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import { DAppRepository } from '#dapps/domain/repositories/dapp.repository.js'
import { Injectable, Logger } from '@nestjs/common'
import { AppError, Task, UseCase } from 'utils'

type Input = {
  dappId: DAppId
}

type Output = {
  stats: DAppStat[]
}

@Injectable()
export class GetDappStatsUseCase implements UseCase<Input, Output> {
  private readonly logger = new Logger(GetDappStatsUseCase.name)
  constructor(private readonly repo: DAppRepository) {}

  execute(input: Input): Task<Output, AppError> {
    this.logger.debug(`requested stats for dappId=${input.dappId}`)
    return this.repo
      .findAllStats(input.dappId)
      .tap(stats => {
        this.logger.verbose(`stats: ${JSON.stringify(stats)}`)
      })
      .map(stats => ({ stats }))
  }
}
