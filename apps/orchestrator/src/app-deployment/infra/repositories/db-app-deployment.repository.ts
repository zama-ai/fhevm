import { AppDeployment } from 'src/app-deployment/entities/app-deployment';
import { AppDeploymentRepository } from 'src/app-deployment/interfaces/app-deployment.repository';
import { DatabaseService } from 'src/database/database.service';

export class DbAppDeploymentRepository implements AppDeploymentRepository {
  constructor(private readonly db: DatabaseService) {}

  async findByApplicationId(
    applicationId: string,
  ): Promise<AppDeployment | null> {
    const snapshot = await this.db.snapshot.findFirst({
      where: { id: applicationId },
    });
    return snapshot ? AppDeployment.fromSnapshot(snapshot.content) : null;
  }

  async upsert(deployment: AppDeployment): Promise<void> {
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
  }
}
