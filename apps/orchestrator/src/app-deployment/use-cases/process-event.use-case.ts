import { Logger } from '@nestjs/common';
import { AppDeployment } from '../entities/app-deployment';
import { isAppDeploymentEvent } from '../entities/app-deployment.events';
import { AppDeploymentMessage } from '../entities/app-deployment.messages';
import { AppDeploymentMessagesProducer } from '../interfaces/app-deployment-messages.producer';
import { AppDeploymentRepository } from '../interfaces/app-deployment.repository';
import type { UseCase } from './use-case';

export class ProcessEventUseCase
  implements UseCase<AppDeploymentMessage, void>
{
  logger = new Logger(ProcessEventUseCase.name);

  constructor(
    private readonly repo: AppDeploymentRepository,
    private readonly producer: AppDeploymentMessagesProducer,
  ) {}

  async execute(message: AppDeploymentMessage): Promise<void> {
    let deployment = await this.repo.findByApplicationId(
      message.payload.applicationId,
    );
    if (!deployment) {
      this.logger.debug(
        `init new AppDeployment for ${message.payload.applicationId}`,
      );
      deployment = AppDeployment.init(message.payload.applicationId);
    }

    if (isAppDeploymentEvent(message)) {
      const messages = deployment.notify(message);
      this.logger.debug(`messages: ${JSON.stringify(messages)}`);
      try {
        await Promise.all(messages.map(this.producer.publish));
      } catch (error) {
        this.logger.error(`Failed to publish messages: ${error}`);
        throw error;
      }
      try {
        await this.repo.upsert(deployment);
      } catch (error) {
        this.logger.error(`Failed to upsert: ${error}`);
      }
    }
  }
}
