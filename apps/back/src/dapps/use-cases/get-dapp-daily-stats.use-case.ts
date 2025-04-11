import { DAppId } from '#dapps/domain/entities/value-objects.js'
import { DAppRepository } from '#dapps/domain/repositories/dapp.repository.js'
import { Injectable, Logger } from '@nestjs/common'
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

  constructor(private readonly repo: DAppRepository) {}

  execute = (input: Input): Task<Output, AppError> => {
    return DAppId.fromString(input.dappId)
      .asyncChain(this.repo.findDailyStats)
      .tapError(error => {
        this.logger.warn(`failed to fetch daily stats: ${error.message}`)
      })
  }
}
