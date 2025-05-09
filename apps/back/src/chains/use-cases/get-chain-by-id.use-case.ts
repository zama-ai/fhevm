import { Chain } from '#chains/domain/entities/chain.js'
import { ChainId } from '#chains/domain/entities/value-objects.js'
import {
  CHAINS_REPOSITORY,
  ChainsRepository,
} from '#chains/domain/repositories/chains.repository.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { AppError, Task, UseCase } from 'utils'

type Input = {
  id: string | number
}

@Injectable()
export class GetChainById implements UseCase<Input, Chain> {
  private readonly logger = new Logger(GetChainById.name)
  constructor(
    @Inject(CHAINS_REPOSITORY) private readonly repo: ChainsRepository,
  ) {}

  execute = (input: Input): Task<Chain, AppError> => {
    this.logger.debug(`searching for chain ID ${input.id}`)
    return ChainId.from(input.id).asyncChain(this.repo.getChainById)
  }
}
