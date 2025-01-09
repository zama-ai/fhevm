import { Logger } from '@nestjs/common'
import { AppDeploymentMessagesProducer } from '../interfaces/app-deployment-messages.producer.js'
import { AppDeploymentRepository } from '../interfaces/app-deployment.repository.js'
import type { UseCase } from './use-case.js'
import { type AppDeploymentMessage, isAppDeploymentEvent } from 'messages'

export class ProcessEventUseCase
  implements UseCase<AppDeploymentMessage, void>
{
  logger = new Logger(ProcessEventUseCase.name)

  constructor(
    private readonly repo: AppDeploymentRepository,
    private readonly producer: AppDeploymentMessagesProducer,
  ) {}

  async execute(message: AppDeploymentMessage): Promise<void> {
    if (isAppDeploymentEvent(message)) {
      const deployment = await this.repo.findByApplicationId(
        message.payload.applicationId,
        message.payload.deploymentId,
      )
      const messages = deployment.send(message)
      this.logger.debug(`messages: ${JSON.stringify(messages)}`)
      try {
        await Promise.all(messages.map(this.producer.publish))
      } catch (error) {
        this.logger.error(`Failed to publish messages: ${error}`)
        throw error
      }
      try {
        await this.repo.upsert(deployment)
      } catch (error) {
        this.logger.error(`Failed to upsert: ${error}`)
      }
    }
  }
}
