import { Module } from '@nestjs/common';
import { InfraModule as AppDeploymentInfraModule } from './app-deployment/infra/infra.module';
@Module({
  imports: [AppDeploymentInfraModule],
  providers: [],
})
export class AppModule {}
