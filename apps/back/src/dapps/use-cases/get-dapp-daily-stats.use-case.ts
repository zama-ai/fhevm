import { Inject, Injectable, Logger } from '@nestjs/common'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '#dapps/domain/repositories/dapp.repository.js'
import { AppError, Task, UseCase } from 'utils'

type Input = {
  dappId: string
}

type Output = {
  id: string
  day: string
  total: number
  computation: number
  encryption: number
}[]

@Injectable()
export class GetDappDailyStatsUseCase implements UseCase<Input, Output> {
  private readonly logger = new Logger(GetDappDailyStatsUseCase.name)

  constructor(@Inject(DAPP_REPOSITORY) private readonly repo: DAppRepository) {}

  execute = (input: Input): Task<Output, AppError> => {
    return DAppId.from(input.dappId)
      .asyncChain(this.repo.findDailyStats)
      .tapError(error => {
        this.logger.warn(`failed to fetch daily stats: ${error.message}`)
      })
  }
}
