import { AppDeploymentMessage } from 'messages'
import { AppError, Task } from 'utils'

export interface AppDeploymentProducer {
  publish(message: AppDeploymentMessage): Task<string, AppError>
}

export const APP_DEPLOYMENT_PRODUCER = Symbol('AppDeploymentProducer')
