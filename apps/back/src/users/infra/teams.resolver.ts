import { Resolver, ResolveField, Parent } from '@nestjs/graphql'
import { TeamId } from '../domain/entities/value-objects'
import { DappType } from '@/dapps/infra/types/dapp.type'
import { TeamType } from './types/team.type'
import { GetDappsByTeamId } from '@/dapps/use-cases/get-dapps-by-team-id.use-case'

@Resolver(() => TeamType)
export class TeamsResolver {
  constructor(private readonly getDappsByTeamUC: GetDappsByTeamId) {}

  @ResolveField(() => [DappType], { name: 'dapps' })
  async dapps(@Parent() team: TeamType) {
    return this.getDappsByTeamUC.execute(new TeamId(team.id)).toPromise()
  }
}
