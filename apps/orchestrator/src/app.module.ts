import { Module } from '@nestjs/common'
import { InfraModule as AppDeploymentInfraModule } from './app-deployment/infra/infra.module.js'
import { ConfigModule } from '@nestjs/config'
import awsConfig from './config/aws.config.js'
import { InfraModule } from '#infra/infra.module.js'
import dbConfig from '#config/db.config.js'
import { BackModule } from '#back/back.module.js'
import { Web3Module } from '#web3/web3.module.js'

// Note: I need to override the default behavior of ConfigModule in the tests,
// and, as we use a dynamic module, we need to store the current instance to
// override it in the tests.
export const configModule = ConfigModule.forRoot({
  isGlobal: true,
  load: [awsConfig, dbConfig],
})

@Module({
  imports: [
    configModule,
    InfraModule,
    AppDeploymentInfraModule,
    BackModule,
    Web3Module,
  ],
})
export class AppModule {}
