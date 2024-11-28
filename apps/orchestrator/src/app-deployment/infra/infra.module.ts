import { Module } from '@nestjs/common';
import { DbAppDeploymentRepository } from './repositories/db-app-deployment.repository';
import { APP_DEPLOYMENT_REPO } from '../interfaces/app-deployment.repository';
import { DatabaseModule } from 'src/database/database.module';
import { AppDeploymentModule } from '../app-deployment.module';

@Module({
  imports: [DatabaseModule, AppDeploymentModule],
  providers: [
    {
      provide: APP_DEPLOYMENT_REPO,
      useClass: DbAppDeploymentRepository,
    },
  ],
})
export class InfraModule {}
