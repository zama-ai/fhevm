import { Module } from '@nestjs/common'
import { DappsResolver } from './dapps.resolver.js'
import { DatabaseModule } from '#infra/database/database.module.js'
import { CreateDapp } from '#dapps/use-cases/create-dapp.use-case.js'
import { UpdateDapp } from '#dapps/use-cases/update-dapp.use-case.js'
import { GetTeamById } from '#users/use-cases/get-team-by-id.use-case.js'
import { GetDappById } from '../use-cases/get-dapp-by-id.use-case.js'
import { DeployDApp } from '../use-cases/deploy-dapp.use-case.js'
import { AppDeployment } from '../use-cases/app-deployment.use-case.js'
import { GetDappStatsUseCase } from '#dapps/use-cases/get-dapp-stats.use-case.js'
import { SharedModule } from '#shared/shared.module.js'
import { SubscriptionsModule } from '#subscriptions/infra/subscriptions.module.js'
import { AppUpdatesSubscription } from '#dapps/use-cases/app-updates-subscription.use-case.js'
import { StoreDAppStats } from '#dapps/use-cases/store-dapp-stats.use-case.js'

@Module({
  imports: [DatabaseModule, SharedModule, SubscriptionsModule],
  providers: [
    DappsResolver,
    CreateDapp,
    UpdateDapp,
    GetDappById,
    GetTeamById,
    DeployDApp,
    GetDappStatsUseCase,
    AppDeployment,
    AppUpdatesSubscription,
    StoreDAppStats,
  ],
  exports: [UpdateDapp, GetDappById],
})
export class DappsModule {}
