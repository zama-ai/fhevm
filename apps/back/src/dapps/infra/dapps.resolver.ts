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
import { GetTeamById } from '#teams/use-cases/get-team-by-id.use-case.js'
import {
  DappType,
  RawStatsType,
  ValidateAddress,
} from '#dapps/infra/types/dapp.type.js'
import { CurrentUser } from '#auth/infra/decorators/current-user.js'
import { JwtAuthGuard } from '#auth/infra/guards/jwt-auth-guard.js'
import { User } from '#users/domain/entities/user.js'
import { DAppId } from '../domain/entities/value-objects.js'
import { TeamType } from '#teams/infra/grapqhl/types/team.type.js'
import { QueryDappInput } from './dto/inputs/query-dapp.input.js'
import { DAppStatProps } from '#dapps/domain/entities/dapp-stat.js'
import { TeamProps } from '#teams/domain/entities/team.js'
import { DeployedDAppInput } from './dto/inputs/deployed-dapp.input.js'
import { ValidateAddressInput } from './dto/inputs/validate-address.input.js'
import { AppErrorFilter } from '#auth/infra/filters/app-error.filter.js'
import { ApiKeyType } from './types/api-key.type.js'
import { DappStatsType } from './types/stat.type.js'
import { ChainType } from '#chains/infra/graphql/types/chain.type.js'
import { ChainProps } from '#chains/domain/entities/chain.js'
import { GetChainById } from '#chains/use-cases/get-chain-by-id.use-case.js'
import { shortString } from 'utils'

@UseFilters(AppErrorFilter)
@Resolver(() => DappType)
export class DappsResolver {
  private readonly logger = new Logger(DappsResolver.name)
  constructor(
    private readonly createDappUC: uc.CreateDapp,
    private readonly updateDappUC: uc.UpdateDapp,
    private readonly getDappByIdUC: uc.GetDappById,
    private readonly getTeamByIdUC: GetTeamById,
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
  async dapp(@Args('input') input: QueryDappInput, @CurrentUser() user: User) {
    this.logger.verbose(`resolving dapp ${input.id}`)
    try {
      const dapp = await DAppId.from(input.id)
        .asyncChain(dappId =>
          this.getDappByIdUC.execute({ dappId, userId: user.id }),
        )
        .toPromise()
      this.logger.debug(
        `dapp: ${JSON.stringify(dapp, (_, v) => (typeof v === 'string' ? shortString(v) : v))}`,
      )
      return dapp
    } catch (err) {
      this.logger.warn(`failed to resolve dapp: ${(err as any).message ?? err}`)
      throw err
    }
  }

  @Mutation(() => DappType, { name: 'createDapp' })
  @UseGuards(JwtAuthGuard)
  async createDapp(
    @Args('input') input: CreateDappInput,
    @CurrentUser() user: User,
  ) {
    this.logger.verbose(
      `creating dapp ${JSON.stringify(input, (_, v) => (typeof v === 'string' ? shortString(v) : v))}`,
    )
    try {
      const dapp = await this.createDappUC
        .execute({ dapp: input, user })
        .toPromise()
      this.logger.log(`dapp ${dapp.id} created`)
      return dapp
    } catch (err) {
      this.logger.warn(`failed to create dapp: ${(err as any).message ?? err}`)
      throw err
    }
  }

  @Mutation(() => DappType, { name: 'updateDapp' })
  @UseGuards(JwtAuthGuard)
  async updateDapp(
    @Args('input') input: UpdateDappInput,
    @CurrentUser() user: User,
  ) {
    this.logger.verbose(
      `updating dapp ${input.id} with ${JSON.stringify(input, (_, v) => (typeof v === 'string' ? shortString(v) : v))}`,
    )
    try {
      const { id, ...props } = input
      const dapp = await DAppId.from(id)
        .asyncChain(dappId =>
          this.updateDappUC.execute({ dapp: { id: dappId, ...props }, user }),
        )
        .toPromise()
      this.logger.log(`dapp ${dapp.id} updated`)
      return dapp
    } catch (err) {
      this.logger.warn(`failed to update dapp: ${(err as any).message ?? err}`)
      throw err
    }
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
    try {
      const chain = (await dapp.chainId)
        ? this.getChainByIdUC
            .execute({ id: dapp.chainId })
            .map(chain => chain.toJSON())
            .toPromise()
        : null
      this.logger.debug(
        `chain: ${JSON.stringify(chain, (_, v) => (typeof v === 'string' ? shortString(v) : v))}`,
      )
      return chain
    } catch (err) {
      this.logger.warn(
        `failed to resolve chain: ${(err as any).message ?? err}`,
      )
      throw err
    }
  }

  @ResolveField(() => TeamType, { name: 'team' })
  async team(@Parent() dapp: DappType): Promise<TeamProps> {
    const { teamId } = dapp
    this.logger.verbose(`resolving team field for dappId=${dapp.id}`)
    try {
      const team = await this.getTeamByIdUC
        .execute(teamId)
        .map(team => team.toJSON())
        .toPromise()
      this.logger.log(
        `team: ${JSON.stringify(team, (_, v) => (typeof v === 'string' ? shortString(v) : v))}`,
      )
      return team
    } catch (err) {
      this.logger.warn(`failed to resolve team: ${(err as any).message ?? err}`)
      throw err
    }
  }

  @ResolveField(() => [RawStatsType], { name: 'rawStats' })
  async rawStats(@Parent() dapp: DappType): Promise<DAppStatProps[]> {
    this.logger.verbose(`resolving raw stats field for dappId=${dapp.id}`)
    try {
      const result = await this.getDappRawStatsUC
        .execute({ dappId: dapp.id })
        .toPromise()
      return result.stats
    } catch (err) {
      this.logger.warn(
        `failed to resolve raw stats: ${(err as any).message ?? err}`,
      )
      throw err
    }
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
    try {
      const result = await this.validateAddressUC.execute(input).toPromise()
      this.logger.verbose(
        `result: ${JSON.stringify(result, (_, v) => (typeof v === 'string' ? shortString(v) : v))}`,
      )
      return result
    } catch (err) {
      this.logger.warn(
        `failed to validate address: ${(err as any).message ?? err}`,
      )
      throw err
    }
  }

  @ResolveField(() => [ApiKeyType], { name: 'apiKeys' })
  async apiKeys(
    @CurrentUser() user: User,
    @Parent() dapp: DappType,
  ): Promise<ApiKeyType[]> {
    this.logger.verbose(`resolving apiKeys field for ${dapp.id}`)
    try {
      const apiKeys = await this.getAllApiKeysUC
        .execute({ dappId: dapp.id }, { user })
        .toPromise()
      this.logger.verbose(
        `apiKeys: ${JSON.stringify(apiKeys, (_, v) => (typeof v === 'string' ? shortString(v) : v))}`,
      )
      return apiKeys.map(apiKey => ({
        id: apiKey.id,
        name: apiKey.name,
        description: apiKey.description,
        createdAt: Number(apiKey.createdAt),
        dappId: apiKey.dappId,
      }))
    } catch (err) {
      this.logger.warn(
        `failed to resolve apiKeys: ${(err as any).message ?? err}`,
      )
      throw err
    }
  }
}
