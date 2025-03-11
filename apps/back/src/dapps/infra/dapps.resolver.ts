import { Logger, UseFilters, UseGuards } from '@nestjs/common'
import {
  Args,
  Mutation,
  Parent,
  Query,
  ResolveField,
  Resolver,
  Subscription,
} from '@nestjs/graphql'
import { CreateDappInput } from '#dapps/infra/dto/inputs/create-dapp.input.js'
import { UpdateDappInput } from '#dapps/infra/dto/inputs/update-dapp.input.js'
import * as uc from '#dapps/use-cases/index.js'
import { GetTeamById } from '#users/use-cases/get-team-by-id.use-case.js'
import {
  DappType,
  StatsType,
  ValidateAddress,
} from '#dapps/infra/types/dapp.type.js'
import { CurrentUser } from '#auth/infra/decorators/current-user.js'
import { JwtAuthGuard } from '#auth/infra/guards/jwt-auth-guard.js'
import { User, type UserProps } from '#users/domain/entities/user.js'
import { TeamId, UserId } from '#users/domain/entities/value-objects.js'
import { DeployDAppInput } from './dto/inputs/deploy-dapp.input.js'
import { DAppId } from '../domain/entities/value-objects.js'
import { TeamType } from '#users/infra/types/team.type.js'
import { QueryDappInput } from './dto/inputs/query-dapp.input.js'
import { DAppStatProps } from '#dapps/domain/entities/dapp-stat.js'
import { TeamProps } from '#users/domain/entities/team.js'
import { DeployedDAppInput } from './dto/inputs/deployed-dapp.input.js'
import { ValidateAddressInput } from './dto/inputs/validate-address.input.js'
import { AppErrorFilter } from '#auth/infra/filters/app-error.filter.js'

@UseFilters(AppErrorFilter)
@Resolver(() => DappType)
export class DappsResolver {
  private readonly logger = new Logger(DappsResolver.name)
  constructor(
    private readonly createDappUC: uc.CreateDapp,
    private readonly updateDappUC: uc.UpdateDapp,
    private readonly getDappByIdUC: uc.GetDappById,
    private readonly getTeamByIdUC: GetTeamById,
    private readonly deployDappUC: uc.DeployDApp,
    private readonly getDappStatsUC: uc.GetDappStatsUseCase,
    private readonly appUpdatesSubscriptionUC: uc.AppUpdatesSubscription,
    private readonly validateAddressUC: uc.ValidateAddress,
  ) {}

  @Query(() => DappType, { name: 'dapp' })
  @UseGuards(JwtAuthGuard)
  dapp(@Args('input') input: QueryDappInput, @CurrentUser() user: UserProps) {
    return this.getDappByIdUC
      .execute({ dappId: DAppId.from(input.id), userId: UserId.from(user.id) })
      .toPromise()
  }

  @Mutation(() => DappType, { name: 'createDapp' })
  @UseGuards(JwtAuthGuard)
  createDapp(
    @Args('input') input: CreateDappInput,
    @CurrentUser() user: UserProps,
  ) {
    return this.createDappUC.execute({ dapp: input, user }).toPromise()
  }

  @Mutation(() => DappType, { name: 'updateDapp' })
  @UseGuards(JwtAuthGuard)
  updateDapp(
    @Args('input') input: UpdateDappInput,
    @CurrentUser() user: UserProps,
  ) {
    const { id, ...props } = input
    return this.updateDappUC
      .execute({ dapp: { id: DAppId.from(id), ...props }, user })
      .toPromise()
  }

  @Mutation(() => DappType, { name: 'deployDapp' })
  @UseGuards(JwtAuthGuard)
  deployDapp(
    @Args('input') input: DeployDAppInput,
    @CurrentUser() user: UserProps,
  ) {
    return this.deployDappUC
      .execute({ dappId: DAppId.from(input.dappId), user })
      .toPromise()
  }

  @Subscription(() => DappType, {
    filter: (
      payload: { dappUpdated: DappType },
      variables: { input: DeployedDAppInput },
    ) => {
      return payload.dappUpdated.id === variables.input.id
    },
  })
  @UseGuards(JwtAuthGuard)
  dappUpdated(
    @Args('input') input: DeployedDAppInput,
    @CurrentUser() user: User,
  ) {
    return this.appUpdatesSubscriptionUC
      .execute({
        dappId: input.id,
        user,
      })
      .toPromise()
  }

  @ResolveField(() => TeamType, { name: 'team' })
  async team(@Parent() dapp: DappType): Promise<TeamProps> {
    const { teamId } = dapp
    return this.getTeamByIdUC.execute(TeamId.from(teamId)).toPromise()
  }

  @ResolveField(() => [StatsType], { name: 'stats' })
  async stats(@Parent() dapp: DappType): Promise<DAppStatProps[]> {
    this.logger.debug(`getting stats for dappId=${dapp.id}`)
    const result = await this.getDappStatsUC
      .execute({ dappId: dapp.id })
      .toPromise()
    return result.stats
  }

  @Query(() => ValidateAddress, { name: 'validateAddress', complexity: 2 })
  @UseGuards(JwtAuthGuard)
  async validateAddress(
    @Args('input') input: ValidateAddressInput,
  ): Promise<ValidateAddress> {
    this.logger.debug(`validating address ${input.chainId}/${input.address}`)
    const result = await this.validateAddressUC.execute(input).toPromise()
    return result
  }
}
