import { AppDeployment } from '../entities/app-deployment';

export interface AppDeploymentRepository {
  findByApplicationId(applicationId: string): Promise<AppDeployment | null>;
  upsert(deployment: AppDeployment): Promise<void>;
}
