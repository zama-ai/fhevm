import { Module } from '@nestjs/common'
import { DappsResolver } from './dapps.resolver.js'
import { DatabaseModule } from '#infra/database/database.module.js'
import * as uc from '#dapps/use-cases/index.js'
import { UpdateDapp } from '#dapps/use-cases/update-dapp.use-case.js'
import { GetTeamById } from '#users/use-cases/get-team-by-id.use-case.js'
import { GetDappById } from '../use-cases/get-dapp-by-id.use-case.js'
import { SharedModule } from '#shared/shared.module.js'
import { SubscriptionsModule } from '#subscriptions/infra/subscriptions.module.js'
import { ApiKeyResolver } from './api-key.resolver.js'
import { StatsResolver } from './stats.resolver.js'
import {
  FEATURE_FLAGS_SERVICE,
  FeatureFlagHandler,
} from '#feature-flag/services/feature-flags.service.js'
import { SYNC_SERVICE, SyncService } from '#shared/services/sync.service.js'
import { SyncInstances } from '#shared/use-cases/sync-instances.use-case.js'

@Module({
  imports: [DatabaseModule, SharedModule, SubscriptionsModule],
  providers: [
    DappsResolver,
    ApiKeyResolver,
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
    {
      provide: uc.VALIDATE_ADDRESS,
      inject: [uc.ValidateAddress, SYNC_SERVICE, SyncInstances],
      useFactory: (
        validateAddress: uc.ValidateAddress,
        syncService: SyncService,
        syncInstances: SyncInstances,
      ) =>
        new uc.ValidateAddressWithSync(
          validateAddress,
          syncService,
          syncInstances,
        ),
    },
    uc.CreateApiKey,
    uc.GetAllApiKeys,
    uc.GetApiKey,
    uc.GetApiKeyByToken,
    uc.UpdateApiKey,
    uc.DeleteApiKey,
    uc.ApiKeyAllowsRequest,
    {
      provide: uc.API_KEY_ALLOWS_REQUEST,
      inject: [uc.ApiKeyAllowsRequest, FEATURE_FLAGS_SERVICE],
      useFactory: (
        apiKeyAllowsRequest: uc.ApiKeyAllowsRequest,
        ffService: FeatureFlagHandler,
      ) =>
        new uc.ApiKeyAllowRequestWithFeatureFlag(
          apiKeyAllowsRequest,
          ffService,
        ),
    },
    uc.ApiKeyAllowsRequest,
    GetTeamById,
  ],
  exports: [
    UpdateDapp,
    GetDappById,
    uc.API_KEY_ALLOWS_REQUEST,
    uc.GetApiKeyByToken,
  ],
})
export class DappsModule {}
