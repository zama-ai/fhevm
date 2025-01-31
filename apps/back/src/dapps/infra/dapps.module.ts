import { Module } from '@nestjs/common'
import { DappsResolver } from './dapps.resolver.js'
import { DatabaseModule } from '#infra/database/database.module.js'
import { CreateDapp } from '#dapps/use-cases/create-dapp.use-case.js'
import { UpdateDapp } from '#dapps/use-cases/update-dapp.use-case.js'
import { GetTeamById } from '#users/use-cases/get-team-by-id.use-case.js'
import { GetDappById } from '../use-cases/get-dapp-by-id.use-case.js'
import { APP_DEPLOYMENT_PRODUCER } from '../domain/services/app-deployment.producer.js'
import { SNSAppDeploymentProducer } from './adapter/sns-app-deployment.producer.js'
import { DeployDApp } from '../use-cases/deploy-dapp.use-case.js'
import { AppDeploymentEnded } from '../use-cases/app-deployment-ended.use-case.js'
import { AppDeploymentRequested } from '#dapps/use-cases/app-deployment-requested.use-case.js'
import { GetDappStatsUseCase } from '#dapps/use-cases/get-dapp-stats.use-case.js'
import { SharedModule } from '#shared/shared.module.js'

@Module({
  imports: [DatabaseModule, SharedModule],
  providers: [
    {
      provide: APP_DEPLOYMENT_PRODUCER,
      useClass: SNSAppDeploymentProducer,
    },
    DappsResolver,
    CreateDapp,
    UpdateDapp,
    GetDappById,
    GetTeamById,
    DeployDApp,
    GetDappStatsUseCase,
    AppDeploymentRequested,
    AppDeploymentEnded,
  ],
  exports: [
    AppDeploymentRequested,
    AppDeploymentEnded,
    UpdateDapp,
    GetDappById,
  ],
})
export class DappsModule {}
