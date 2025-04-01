import { Logger, UseFilters } from '@nestjs/common'
import { Parent, ResolveField, Resolver } from '@nestjs/graphql'
import * as uc from '#dapps/use-cases/index.js'

import { AppErrorFilter } from '#auth/infra/filters/app-error.filter.js'
import { CumulativeDappStatsType, DappStatsType } from './types/stat.type.js'

@UseFilters(AppErrorFilter)
@Resolver(() => DappStatsType)
export class StatsResolver {
  private readonly logger = new Logger(StatsResolver.name)
  constructor(
    private readonly getDappRawStatsUC: uc.GetDappRawStatsUseCase,
    private readonly getDappCumulativeStatsUC: uc.GetDappCumulativeStatsUseCase,
  ) {}

  // TODO: add query & subscription resolvers

  @ResolveField(() => CumulativeDappStatsType, {
    name: 'cumulative',
  })
  async cumulative(
    @Parent() dappStat: DappStatsType,
  ): Promise<CumulativeDappStatsType> {
    this.logger.log(`Resolving cumulative stats for ${dappStat.id}`)
    return this.getDappCumulativeStatsUC
      .execute({ dappId: dappStat.id })
      .toPromise()
  }

  @ResolveField(() => DappStatsType, {
    name: 'id',
  })
  async id(@Parent() dappStat: DappStatsType): Promise<string> {
    return Promise.resolve(dappStat.id)
  }
}
