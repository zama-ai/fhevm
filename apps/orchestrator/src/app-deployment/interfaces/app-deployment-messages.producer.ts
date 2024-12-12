import { AppDeploymentMessage } from 'messages'

export interface AppDeploymentMessagesProducer {
  publish(message: AppDeploymentMessage): Promise<void>
}

export const APP_DEPLOYMENT_PRODUCER = Symbol('AppDeploymentMessagesProducer')
