import { Module } from '@nestjs/common'
import { InfraModule as AppDeploymentInfraModule } from './app-deployment/infra/infra.module.js'
import { ConfigModule } from '@nestjs/config'
import awsConfig from './config/aws.config.js'
@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      load: [awsConfig],
    }),
    AppDeploymentInfraModule,
  ],
  providers: [],
})
export class AppModule {}
