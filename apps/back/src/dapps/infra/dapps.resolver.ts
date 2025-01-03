import { UseGuards } from '@nestjs/common'
import {
  Args,
  Mutation,
  Parent,
  Query,
  ResolveField,
  Resolver,
} from '@nestjs/graphql'
import { CreateDappInput } from '@/dapps/infra/dto/inputs/create-dapp.input'
import { UpdateDappInput } from '@/dapps/infra/dto/inputs/update-dapp.input'
import { CreateDapp } from '@/dapps/use-cases/create-dapp.use-case'
import { GetTeamById } from '@/users/use-cases/get-team-by-id.use-case'
import { UpdateDapp } from '@/dapps/use-cases/update-dapp.use-case'
import { DappType } from '@/dapps/infra/types/dapp.type'
import { CurrentUser } from '@/auth/infra/decorators/current-user'
import { JwtAuthGuard } from '@/auth/infra/guards/jwt-auth-guard'
import { User } from '@/users/domain/entities/user'
import { TeamId } from '@/users/domain/entities/value-objects'
import { DeployDApp } from '../use-cases/deploy-dapp.use-case'
import { DeployDAppInput } from './dto/inputs/deploy-dapp.input'
import { GetDappById } from '../use-cases/get-dapp-by-id.use-case'
import { DAppId } from '../domain/entities/value-objects'
import { TeamType } from '@/users/infra/types/team.type'
import { QueryDappInput } from './dto/inputs/query-dapp.input'

@Resolver(() => DappType)
export class DappsResolver {
  constructor(
    private readonly createDappUC: CreateDapp,
    private readonly updateDappUC: UpdateDapp,
    private readonly getDappByIdUC: GetDappById,
    private readonly getTeamByIdUC: GetTeamById,
    private readonly deployDappUC: DeployDApp,
  ) {}

  @Query(() => DappType, { name: 'dapp' })
  @UseGuards(JwtAuthGuard)
  dapp(@Args('input') input: QueryDappInput, @CurrentUser() user: User) {
    return this.getDappByIdUC
      .execute({ dappId: new DAppId(input.id), userId: user.id })
      .toPromise()
  }

  @Mutation(() => DappType, { name: 'createDapp' })
  @UseGuards(JwtAuthGuard)
  createDapp(@Args('input') input: CreateDappInput, @CurrentUser() user: User) {
    return this.createDappUC.execute({ dapp: input, user }).toPromise()
  }

  @Mutation(() => DappType, { name: 'updateDapp' })
  @UseGuards(JwtAuthGuard)
  updateDapp(@Args('input') input: UpdateDappInput, @CurrentUser() user: User) {
    const { id, ...props } = input
    return this.updateDappUC
      .execute({ dapp: { id: new DAppId(id), ...props }, user })
      .toPromise()
  }

  @Mutation(() => DappType, { name: 'deployDapp' })
  @UseGuards(JwtAuthGuard)
  deployDapp(@Args('input') input: DeployDAppInput, @CurrentUser() user: User) {
    return this.deployDappUC
      .execute({ dappId: new DAppId(input.dappId), user })
      .toPromise()
  }

  @ResolveField(() => TeamType, { name: 'team' })
  async team(@Parent() dapp: DappType) {
    const { teamId } = dapp
    return this.getTeamByIdUC.execute(new TeamId(teamId)).toPromise()
  }
}
