import { AppDeploymentMessage } from 'messages'
import { AppError, Task } from 'utils'

export interface MessageProducer {
  produce(message: AppDeploymentMessage): Task<string, AppError>
}
