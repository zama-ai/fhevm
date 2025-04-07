import { Module } from '@nestjs/common'
import { DappsResolver } from './dapps.resolver.js'
import { DatabaseModule } from '#infra/database/database.module.js'
import * as uc from '#dapps/use-cases/index.js'
import { UpdateDapp } from '#dapps/use-cases/update-dapp.use-case.js'
import { GetTeamById } from '#users/use-cases/get-team-by-id.use-case.js'
import { GetDappById } from '../use-cases/get-dapp-by-id.use-case.js'
import { SharedModule } from '#shared/shared.module.js'
import { SubscriptionsModule } from '#subscriptions/infra/subscriptions.module.js'
import { StatsResolver } from './stats.resolver.js'

@Module({
  imports: [DatabaseModule, SharedModule, SubscriptionsModule],
  providers: [
    DappsResolver,
    StatsResolver,
    uc.CreateDapp,
    uc.UpdateDapp,
    uc.GetDappById,
    uc.DeployDApp,
    uc.GetDappRawStatsUseCase,
    uc.GetDappCumulativeStatsUseCase,
    uc.GetDappDailyStatsUseCase,
    uc.AppDeployment,
    uc.AppUpdatesSubscription,
    uc.StoreDAppStats,
    uc.ValidateAddress,
    GetTeamById,
  ],
  exports: [UpdateDapp, GetDappById],
})
export class DappsModule {}
