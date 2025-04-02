import { DAppId } from '#dapps/domain/entities/value-objects.js'
import { DAppRepository } from '#dapps/domain/repositories/dapp.repository.js'
import { Injectable, Logger } from '@nestjs/common'
import { AppError, Task, UseCase, validationError } from 'utils'

type Input = {
  dappId: string
}

type Output = {
  id: string
  day: string
  total: number
  symbolic: number
  fhe: number
}[]

@Injectable()
export class GetDappDailyStatsUseCase implements UseCase<Input, Output> {
  private readonly logger = new Logger(GetDappDailyStatsUseCase.name)

  constructor(private readonly repo: DAppRepository) {}

  execute = (input: Input): Task<Output, AppError> => {
    const dappId = DAppId.fromString(input.dappId)
    if (!dappId.isOk()) {
      this.logger.error(`Invalid dapp id: ${input.dappId}`)
      return Task.reject(validationError('Invalid dapp id'))
    }

    return this.repo.findDailyStats(dappId.unwrap())
  }
}
