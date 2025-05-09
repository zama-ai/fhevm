import { Chain } from '#chains/domain/entities/chain.js'
import {
  CHAINS_REPOSITORY,
  ChainsRepository,
} from '#chains/domain/repositories/chains.repository.js'
import { Inject, Injectable } from '@nestjs/common'
import { AppError, Task, UseCase } from 'utils'

@Injectable()
export class GetAllChains implements UseCase<void, Chain[]> {
  constructor(
    @Inject(CHAINS_REPOSITORY)
    private readonly chainRepository: ChainsRepository,
  ) {}

  execute = (
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _: void,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<Chain[], AppError> => {
    return this.chainRepository.getChains()
  }
}
