import { type AppError, Option, Task } from 'utils'
import { AppDeployment } from '../entities/app-deployment.js'

export interface AppDeploymentRepository {
  findByDAppId(dAppId: string): Task<Option<AppDeployment>, AppError>
  findByChainIdAndAddress(
    chainId: string,
    address: string,
  ): Task<Option<AppDeployment>, AppError>
  upsert(deployment: AppDeployment): Task<void, AppError>
  delete(deployment: AppDeployment): Task<void, AppError>
}
