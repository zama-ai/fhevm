import { Inject, Logger, UseFilters, UseGuards } from '@nestjs/common'
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
  RawStatsType,
  ValidateAddress,
} from '#dapps/infra/types/dapp.type.js'
import { CurrentUser } from '#auth/infra/decorators/current-user.js'
import { JwtAuthGuard } from '#auth/infra/guards/jwt-auth-guard.js'
import { User } from '#users/domain/entities/user.js'
import { TeamId } from '#users/domain/entities/value-objects.js'
import { DeployDAppInput } from './dto/inputs/deploy-dapp.input.js'
import { DAppId } from '../domain/entities/value-objects.js'
import { TeamType } from '#users/infra/types/team.type.js'
import { QueryDappInput } from './dto/inputs/query-dapp.input.js'
import { DAppStatProps } from '#dapps/domain/entities/dapp-stat.js'
import { TeamProps } from '#users/domain/entities/team.js'
import { DeployedDAppInput } from './dto/inputs/deployed-dapp.input.js'
import { ValidateAddressInput } from './dto/inputs/validate-address.input.js'
import { AppErrorFilter } from '#auth/infra/filters/app-error.filter.js'
import { ApiKeyType } from './types/api-key.type.js'
import { DappStatsType } from './types/stat.type.js'
import { ChainType } from '#chains/infra/graphql/types/chain.type.js'
import { ChainProps } from '#chains/domain/entities/chain.js'
import { GetChainById } from '#chains/use-cases/get-chain-by-id.use-case.js'

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
    private readonly getDappRawStatsUC: uc.GetDappRawStatsUseCase,
    private readonly appUpdatesSubscriptionUC: uc.AppUpdatesSubscription,
  ) {}
  @Inject(uc.GetAllApiKeys)
  private readonly getAllApiKeysUC: uc.GetAllApiKeys

  @Inject(GetChainById)
  private readonly getChainByIdUC: GetChainById

  @Inject(uc.VALIDATE_ADDRESS)
  private readonly validateAddressUC: uc.IValidateAddress

  @Query(() => DappType, { name: 'dapp' })
  @UseGuards(JwtAuthGuard)
  dapp(@Args('input') input: QueryDappInput, @CurrentUser() user: User) {
    this.logger.verbose(`resolving dapp ${input.id}`)
    return DAppId.from(input.id)
      .asyncChain(dappId =>
        this.getDappByIdUC.execute({ dappId, userId: user.id }),
      )
      .toPromise()
  }

  @Mutation(() => DappType, { name: 'createDapp' })
  @UseGuards(JwtAuthGuard)
  createDapp(@Args('input') input: CreateDappInput, @CurrentUser() user: User) {
    this.logger.verbose(`creating dapp ${JSON.stringify(input)}`)
    return this.createDappUC.execute({ dapp: input, user }).toPromise()
  }

  @Mutation(() => DappType, { name: 'updateDapp' })
  @UseGuards(JwtAuthGuard)
  updateDapp(@Args('input') input: UpdateDappInput, @CurrentUser() user: User) {
    this.logger.verbose(
      `updating dapp ${input.id} with ${JSON.stringify(input)}`,
    )
    const { id, ...props } = input
    return DAppId.from(id)
      .asyncChain(dappId =>
        this.updateDappUC.execute({ dapp: { id: dappId, ...props }, user }),
      )
      .toPromise()
  }

  @Mutation(() => DappType, { name: 'deployDapp' })
  @UseGuards(JwtAuthGuard)
  deployDapp(@Args('input') input: DeployDAppInput, @CurrentUser() user: User) {
    this.logger.verbose(`deploying dapp ${input.dappId}`)
    return DAppId.from(input.dappId)
      .asyncChain(dappId => this.deployDappUC.execute({ dappId, user }))
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
    this.logger.verbose(`subscribing to dapp updates for dappId=${input.id}`)
    this.appUpdatesSubscriptionUC
      .execute({
        dappId: input.id,
        user,
      })
      .toPromise()
  }

  @ResolveField(() => ChainType, { name: 'chain', nullable: true })
  async getDappChain(@Parent() dapp: DappType): Promise<ChainProps | null> {
    return dapp.chainId
      ? this.getChainByIdUC
          .execute({ id: dapp.chainId })
          .map(chain => chain.toJSON())
          .toPromise()
      : null
  }

  @ResolveField(() => TeamType, { name: 'team' })
  async team(@Parent() dapp: DappType): Promise<TeamProps> {
    const { teamId } = dapp
    this.logger.verbose(`resolving team field for dappId=${dapp.id}`)
    return TeamId.from(teamId)
      .asyncChain(this.getTeamByIdUC.execute)
      .toPromise()
  }

  @ResolveField(() => [RawStatsType], { name: 'rawStats' })
  async rawStats(@Parent() dapp: DappType): Promise<DAppStatProps[]> {
    this.logger.verbose(`resolving raw stats field for dappId=${dapp.id}`)
    const result = await this.getDappRawStatsUC
      .execute({ dappId: dapp.id })
      .toPromise()
    return result.stats
  }

  @ResolveField(() => DappStatsType, { name: 'stats' })
  async stats(@Parent() dapp: DappType): Promise<DappStatsType> {
    return Promise.resolve({
      id: dapp.id,
    })
  }

  @Query(() => ValidateAddress, { name: 'validateAddress', complexity: 2 })
  @UseGuards(JwtAuthGuard)
  async validateAddress(
    @Args('input') input: ValidateAddressInput,
  ): Promise<ValidateAddress> {
    this.logger.verbose(`validating address ${input.chainId}/${input.address}`)
    const result = await this.validateAddressUC.execute(input).toPromise()
    return result
  }

  @ResolveField(() => [ApiKeyType], { name: 'apiKeys' })
  async apiKeys(
    @CurrentUser() user: User,
    @Parent() dapp: DappType,
  ): Promise<ApiKeyType[]> {
    this.logger.verbose(`resolving apiKeys field for ${dapp.id}`)
    const apiKeys = await this.getAllApiKeysUC
      .execute({ dappId: dapp.id }, { user })
      .toPromise()
    this.logger.verbose(`apiKeys: ${JSON.stringify(apiKeys)}`)
    return apiKeys.map(apiKey => ({
      id: apiKey.id,
      name: apiKey.name,
      description: apiKey.description,
      createdAt: Number(apiKey.createdAt),
      dappId: apiKey.dappId,
    }))
  }
}
