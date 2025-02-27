import { type AppError, Option, Task } from 'utils'
import { AppDeployment } from '../entities/app-deployment.js'

export interface AppDeploymentRepository {
  findByRequestId(requestId: string): Task<Option<AppDeployment>, AppError>
  upsert(requestId: string, status: string): Task<void, AppError>
  delete(requestId: string): Task<void, AppError>
}
