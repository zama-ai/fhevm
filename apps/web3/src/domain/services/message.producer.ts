import { AppDeploymentMessage } from 'messages'

export interface MessageProducer {
  produce(message: AppDeploymentMessage): void
}
