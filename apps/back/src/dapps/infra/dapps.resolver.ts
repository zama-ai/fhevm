import { Inject, UseGuards } from '@nestjs/common'
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
import { CreateDapp } from '#dapps/use-cases/create-dapp.use-case.js'
import { GetTeamById } from '#users/use-cases/get-team-by-id.use-case.js'
import { UpdateDapp } from '#dapps/use-cases/update-dapp.use-case.js'
import { DappType } from '#dapps/infra/types/dapp.type.js'
import { DummyType } from '#dapps/infra/types/dummy.type.js'
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
import {
  SUBSCRIPTION_SERVICE,
  SubscriptionService,
} from '#subscriptions/domain/services/subscription.service.js'
import { DApp } from '#dapps/domain/entities/dapp.js'
import { DeployedDAppInput } from './dto/inputs/deployed-dapp.input.js'
import { AppUpdatesSubscription } from '#dapps/use-cases/app-updates-subscription.use-case.js'
import { SubscriptionDummyPayload } from '#subscriptions/domain/entities/subscription.js'

@Resolver(() => DappType)
export class DappsResolver {
  constructor(
    private readonly createDappUC: CreateDapp,
    private readonly updateDappUC: UpdateDapp,
    private readonly getDappByIdUC: GetDappById,
    private readonly getTeamByIdUC: GetTeamById,
    private readonly deployDappUC: DeployDApp,
    private readonly appUpdatesSubscriptionUC: AppUpdatesSubscription,
    @Inject(SUBSCRIPTION_SERVICE)
    private readonly subscriptions: SubscriptionService,
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

  @Mutation(() => DummyType, { name: 'testDummySubscription' })
  testDummySubscription() {
    this.subscriptions.publish('dummy', {
      dummy: {
        id: 'dummy_' + Math.floor(Math.random() * 100),
        name: 'test',
      },
    })
    return { id: 'dummy_1', name: 'test' + Math.floor(Math.random() * 100) }
  }

  @Mutation(() => DappType, { name: 'testDappSubscription' })
  testDappSubscription() {
    const dapp = DApp.parse({
      id: 'dapp_cRcSlh0_the9',
      teamId: 'team_lSOuxerGl4',
      createdAt: new Date(),
      status: 'DEPLOYING',
      name: 'test' + Math.floor(Math.random() * 100),
      address: '0x004f6ab8b0c9977fb5464354ac152d3d1b5605f9',
    })
      .unwrap()
      .toJSON()

    this.subscriptions.publish('dappUpdated', { dappUpdated: dapp })
    return dapp
  }

  @Subscription(() => DummyType)
  @UseGuards(JwtAuthGuard)
  dummy(@Args('input') input: DeployedDAppInput, @CurrentUser() user: User) {
    console.log('input', input, user.email)
    return this.subscriptions.asyncIterableIterator<SubscriptionDummyPayload>(
      'dummy',
    )
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
    console.log('input', input, user.email)
    return this.appUpdatesSubscriptionUC
      .execute({
        dappId: input.id,
        user,
      })
      .toPromise()
  }

  @ResolveField(() => TeamType, { name: 'team' })
  async team(@Parent() dapp: DappType) {
    const { teamId } = dapp
    return this.getTeamByIdUC.execute(new TeamId(teamId)).toPromise()
  }
}
