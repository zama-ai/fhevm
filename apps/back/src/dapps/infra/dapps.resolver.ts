import { Args, Mutation, Parent, ResolveField, Resolver } from '@nestjs/graphql'
import { CreateDappInput } from '@/dapps/infra/dto/inputs/create-dapp.input'
import { CreateDapp } from '@/dapps/use-cases/create-dapp.use-case'
import { GetTeamById } from '@/users/use-cases/get-team-by-id.use-case'
import { DappType } from './types/dapp.type'
import { CurrentUser } from '@/auth/infra/decorators/current-user'
import { UseGuards } from '@nestjs/common'
import { JwtAuthGuard } from '@/auth/infra/guards/jwt-auth-guard'
import { User } from '@/users/domain/entities/user'
import { TeamId } from '@/users/domain/entities/value-objects'

@Resolver(() => DappType)
export class DappsResolver {
  constructor(
    private readonly createDappUC: CreateDapp,
    private readonly getTeamByIdUC: GetTeamById,
  ) {}

  @Mutation(() => DappType, { name: 'createDapp' })
  @UseGuards(JwtAuthGuard)
  createInvitation(
    @Args('input') input: CreateDappInput,
    @CurrentUser() user: User,
  ) {
    return this.createDappUC.execute(input, { user }).toPromise()
  }

  @ResolveField()
  async team(@Parent() dapp: DappType) {
    const { teamId } = dapp
    return this.getTeamByIdUC.execute(new TeamId(teamId)).toPromise()
  }
}
