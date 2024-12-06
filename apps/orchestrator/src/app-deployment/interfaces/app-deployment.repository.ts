import { AppDeployment } from '../entities/app-deployment'

export interface AppDeploymentRepository {
  findByApplicationId(
    applicationId: string,
    deploymentId: string,
  ): Promise<AppDeployment>
  upsert(deployment: AppDeployment): Promise<void>
}

export const APP_DEPLOYMENT_REPO = Symbol('AppDeploymentRepo')
