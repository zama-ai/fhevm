import { AppDeploymentMessage } from '../entities/app-deployment.messages';

export interface AppDeploymentMessagesProducer {
  publish(message: AppDeploymentMessage): Promise<void>;
}
