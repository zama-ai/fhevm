import { type AppError, Task } from 'utils'
import { AppDeployment } from '../entities/app-deployment.js'

export interface AppDeploymentRepository {
  findByChainIdAndAddress(
    chainId: string,
    address: string,
  ): Task<AppDeployment, AppError>
  upsert(deployment: AppDeployment): Task<void, AppError>
  delete(deployment: AppDeployment): Task<void, AppError>
}
