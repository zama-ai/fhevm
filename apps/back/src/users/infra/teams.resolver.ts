import { Resolver, ResolveField, Parent } from '@nestjs/graphql'
import { TeamId } from '../domain/entities/value-objects.js'
import { DappType } from '#dapps/infra/types/dapp.type.js'
import { TeamType } from './types/team.type.js'
import { GetDappsByTeamId } from '#dapps/use-cases/get-dapps-by-team-id.use-case.js'

@Resolver(() => TeamType)
export class TeamsResolver {
  constructor(private readonly getDappsByTeamUC: GetDappsByTeamId) {}

  @ResolveField(() => [DappType], { name: 'dapps' })
  async dapps(@Parent() team: TeamType) {
    return this.getDappsByTeamUC.execute(new TeamId(team.id)).toPromise()
  }
}
