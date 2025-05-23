import { Resolver, ResolveField, Parent } from '@nestjs/graphql'
import { TeamId } from '#teams/domain/entities/value-objects.js'
import { DappType } from '#dapps/infra/types/dapp.type.js'
import { TeamType } from './types/team.type.js'
import { GetDappsByTeamId } from '#dapps/use-cases/get-dapps-by-team-id.use-case.js'
import { UseFilters } from '@nestjs/common'
import { AppErrorFilter } from '#auth/infra/filters/app-error.filter.js'

@UseFilters(AppErrorFilter)
@Resolver(() => TeamType)
export class TeamsResolver {
  constructor(private readonly getDappsByTeamUC: GetDappsByTeamId) {}

  @ResolveField(() => [DappType], { name: 'dapps' })
  async dapps(@Parent() team: TeamType) {
    return TeamId.from(team.id)
      .asyncChain(this.getDappsByTeamUC.execute)
      .toPromise()
  }
}
