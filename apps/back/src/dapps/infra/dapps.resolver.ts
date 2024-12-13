import { UseGuards } from '@nestjs/common'
import { Args, Mutation, Parent, ResolveField, Resolver } from '@nestjs/graphql'
import { CreateDappInput } from '@/dapps/infra/dto/inputs/create-dapp.input'
import { UpdateDappInput } from './dto/inputs/update-dapp.input'
import { CreateDapp } from '@/dapps/use-cases/create-dapp.use-case'
import { GetTeamById } from '@/users/use-cases/get-team-by-id.use-case'
import { UpdateDapp } from '../use-cases/update-dapp.use-case'
import { DappType } from './types/dapp.type'
import { CurrentUser } from '@/auth/infra/decorators/current-user'
import { JwtAuthGuard } from '@/auth/infra/guards/jwt-auth-guard'
import { User } from '@/users/domain/entities/user'
import { TeamId } from '@/users/domain/entities/value-objects'
import { DeployDApp } from '../use-cases/deploy-dapp.use-case'

@Resolver(() => DappType)
export class DappsResolver {
  constructor(
    private readonly createDappUC: CreateDapp,
    private readonly updateDappUC: UpdateDapp,
    private readonly getTeamByIdUC: GetTeamById,
    private readonly deployDappUC: DeployDApp,
  ) {}

  @Mutation(() => DappType, { name: 'createDapp' })
  @UseGuards(JwtAuthGuard)
  createDapp(@Args('input') input: CreateDappInput, @CurrentUser() user: User) {
    return this.createDappUC.execute({ dapp: input, user }).toPromise()
  }

  @Mutation(() => DappType, { name: 'updateDapp' })
  @UseGuards(JwtAuthGuard)
  updateDapp(@Args('input') input: UpdateDappInput, @CurrentUser() user: User) {
    return this.updateDappUC.execute({ dapp: input, user }).toPromise()
  }

  @Mutation(() => DappType, { name: 'deployDapp' })
  @UseGuards(JwtAuthGuard)
  deployDapp(
    @Args('applicationId') applicationId: string,
    @CurrentUser() user: User,
  ) {
    return this.deployDappUC.execute({ applicationId, user }).toPromise()
  }

  @ResolveField()
  async team(@Parent() dapp: DappType) {
    const { teamId } = dapp
    return this.getTeamByIdUC.execute(new TeamId(teamId)).toPromise()
  }
}
