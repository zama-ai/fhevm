import { Module } from '@nestjs/common'
import { DappsResolver } from './dapps.resolver'
import { DatabaseModule } from '@/infra/database/database.module'
import { CreateDapp } from '@/dapps/use-cases/create-dapp.use-case'
import { UpdateDapp } from '@/dapps/use-cases/update-dapp.use-case'
import { GetTeamById } from '@/users/use-cases/get-team-by-id.use-case'
import { APP_DEPLOYMENT_PRODUCER } from '../domain/services/app-deployment.producer'
import { SNSAppDeploymentProducer } from './adapter/sns-app-deployment.producer'
import { DeployDApp } from '../use-cases/deploy-dapp.use-case'

@Module({
  imports: [DatabaseModule],
  providers: [
    {
      provide: APP_DEPLOYMENT_PRODUCER,
      useClass: SNSAppDeploymentProducer,
    },
    DappsResolver,
    CreateDapp,
    UpdateDapp,
    GetTeamById,
    DeployDApp,
  ],
  exports: [UpdateDapp],
})
export class DappsModule {}
