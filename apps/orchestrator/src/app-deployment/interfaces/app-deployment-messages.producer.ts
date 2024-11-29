import { AppDeploymentMessage } from '../entities/app-deployment.messages';

export interface AppDeploymentMessagesProducer {
  publish(message: AppDeploymentMessage): Promise<void>;
}

export const APP_DEPLOYMENT_PRODUCER = Symbol('AppDeploymentMessagesProducer');
