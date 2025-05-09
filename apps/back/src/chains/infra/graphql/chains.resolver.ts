import { Inject, Logger, UseFilters } from '@nestjs/common'
import * as uc from '#chains/use-cases/index.js'
import { Args, Query, Resolver } from '@nestjs/graphql'
import { ChainType } from './types/chain.type.js'
import { QueryChainInput } from './inputs/query-chain.input.js'
// TODO: move AppErrorFilter to shared module
import { AppErrorFilter } from '#auth/infra/filters/app-error.filter.js'

@UseFilters(AppErrorFilter)
@Resolver(() => ChainType)
export class ChainsResolver {
  private readonly logger = new Logger(ChainsResolver.name)

  @Inject(uc.GetChainById)
  private readonly getChainByIdUC: uc.GetChainById

  @Inject(uc.GetAllChains)
  private readonly getAllChainsUC: uc.GetAllChains

  @Query(() => [ChainType], { name: 'chains', description: 'Get all chains' })
  async getAllChains() {
    this.logger.log('getAllChains')
    return this.getAllChainsUC.execute().toPromise()
  }

  @Query(() => ChainType, { name: 'chain', description: 'Get chain by ID' })
  async getChainById(@Args('input') input: QueryChainInput) {
    this.logger.log('getChainById')
    return this.getChainByIdUC.execute(input).toPromise()
  }
}
