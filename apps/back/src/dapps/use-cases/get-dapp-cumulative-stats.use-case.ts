import { Inject, Injectable, Logger } from '@nestjs/common'
import { AppError, Task, UseCase } from 'utils'
import { DAppId } from '../domain/entities/value-objects.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
  type CumulativeStats,
} from '../domain/repositories/dapp.repository.js'

type Input = {
  dappId: string
}

@Injectable()
export class GetDappCumulativeStatsUseCase
  implements UseCase<Input, CumulativeStats>
{
  private readonly logger = new Logger(GetDappCumulativeStatsUseCase.name)

  constructor(
    @Inject(DAPP_REPOSITORY) private readonly dappRepository: DAppRepository,
  ) {}

  execute = (input: Input): Task<CumulativeStats, AppError> => {
    this.logger.debug(`Calculating cumulative stats for dappId=${input.dappId}`)
    return DAppId.from(input.dappId).asyncChain(dappId =>
      this.dappRepository.findCumulativeStats(dappId),
    )
  }
}
