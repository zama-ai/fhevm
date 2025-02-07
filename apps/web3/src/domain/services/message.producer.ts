import type { AppDeploymentMessage } from 'messages'
import type { AppError, Task } from 'utils'

export interface MessageProducer {
  produce(message: AppDeploymentMessage): Task<string, AppError>
}
