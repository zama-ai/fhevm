import { Logger } from '@nestjs/common';
import { AppDeployment } from 'src/app-deployment/entities/app-deployment';
import { AppDeploymentRepository } from 'src/app-deployment/interfaces/app-deployment.repository';
import { DatabaseService } from 'src/database/database.service';

export class DbAppDeploymentRepository implements AppDeploymentRepository {
  logger = new Logger(DbAppDeploymentRepository.name);

  constructor(private readonly db: DatabaseService) {}

  async findByApplicationId(
    applicationId: string,
    deploymentId: string,
  ): Promise<AppDeployment> {
    this.logger.debug(`requested applicationId=${applicationId}`);
    try {
      const snapshot = await this.db.snapshot.findFirst({
        where: { id: `${applicationId}/${deploymentId}` },
      });
      this.logger.debug(`snapshot: ${JSON.stringify(snapshot)}`);
      return new AppDeployment(
        { applicationId, deploymentId },
        snapshot?.content,
      );
    } catch (error) {
      this.logger.error(`Failed: ${error}`);
      throw error;
    }
  }

  async upsert(deployment: AppDeployment): Promise<void> {
    this.logger.debug(`upserting applicationId=${deployment.applicationId}`);
    try {
      await this.db.snapshot.upsert({
        create: {
          id: deployment.applicationId,
          content: deployment.snapshot,
        },
        update: {
          content: deployment.snapshot,
        },
        where: { id: deployment.applicationId },
      });
    } catch (error) {
      this.logger.error(`Failed: ${error}`);
      throw error;
    }
  }
}
