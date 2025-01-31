import { Logger, UseGuards } from '@nestjs/common'
import {
  Args,
  Mutation,
  Parent,
  Query,
  ResolveField,
  Resolver,
} from '@nestjs/graphql'
import { CreateDappInput } from '#dapps/infra/dto/inputs/create-dapp.input.js'
import { UpdateDappInput } from '#dapps/infra/dto/inputs/update-dapp.input.js'
import { CreateDapp } from '#dapps/use-cases/create-dapp.use-case.js'
import { GetTeamById } from '#users/use-cases/get-team-by-id.use-case.js'
import { UpdateDapp } from '#dapps/use-cases/update-dapp.use-case.js'
import { DappType, StatsType } from '#dapps/infra/types/dapp.type.js'
import { CurrentUser } from '#auth/infra/decorators/current-user.js'
import { JwtAuthGuard } from '#auth/infra/guards/jwt-auth-guard.js'
import { User } from '#users/domain/entities/user.js'
import { TeamId } from '#users/domain/entities/value-objects.js'
import { DeployDApp } from '../use-cases/deploy-dapp.use-case.js'
import { DeployDAppInput } from './dto/inputs/deploy-dapp.input.js'
import { GetDappById } from '../use-cases/get-dapp-by-id.use-case.js'
import { DAppId } from '../domain/entities/value-objects.js'
import { TeamType } from '#users/infra/types/team.type.js'
import { QueryDappInput } from './dto/inputs/query-dapp.input.js'
import { GetDappStatsUseCase } from '#dapps/use-cases/get-dapp-stats.use-case.js'
import { DAppStatProps } from '#dapps/domain/entities/dapp-stat.js'
import { DAppProps } from '#dapps/domain/entities/dapp.js'
import { TeamProps } from '#users/domain/entities/team.js'

@Resolver(() => DappType)
export class DappsResolver {
  private readonly logger = new Logger(DappsResolver.name)
  constructor(
    private readonly createDappUC: CreateDapp,
    private readonly updateDappUC: UpdateDapp,
    private readonly getDappByIdUC: GetDappById,
    private readonly getTeamByIdUC: GetTeamById,
    private readonly deployDappUC: DeployDApp,
    private readonly getDappStatsUC: GetDappStatsUseCase,
  ) {}

  @Query(() => DappType, { name: 'dapp' })
  @UseGuards(JwtAuthGuard)
  dapp(
    @Args('input') input: QueryDappInput,
    @CurrentUser() user: User,
  ): Promise<DAppProps> {
    return this.getDappByIdUC
      .execute({ dappId: new DAppId(input.id), userId: user.id })
      .toPromise()
  }

  @Mutation(() => DappType, { name: 'createDapp' })
  @UseGuards(JwtAuthGuard)
  createDapp(
    @Args('input') input: CreateDappInput,
    @CurrentUser() user: User,
  ): Promise<DAppProps> {
    return this.createDappUC.execute({ dapp: input, user }).toPromise()
  }

  @Mutation(() => DappType, { name: 'updateDapp' })
  @UseGuards(JwtAuthGuard)
  updateDapp(
    @Args('input') input: UpdateDappInput,
    @CurrentUser() user: User,
  ): Promise<DAppProps> {
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
  async team(@Parent() dapp: DappType): Promise<TeamProps> {
    const { teamId } = dapp
    return this.getTeamByIdUC.execute(new TeamId(teamId)).toPromise()
  }

  @ResolveField(() => [StatsType], { name: 'stats' })
  async stats(@Parent() dapp: DappType): Promise<DAppStatProps[]> {
    this.logger.debug(`getting stats for dappId=${dapp.id}`)
    const result = await this.getDappStatsUC
      .execute({ dappId: dapp.id })
      .toPromise()
    return result.stats
  }
}
