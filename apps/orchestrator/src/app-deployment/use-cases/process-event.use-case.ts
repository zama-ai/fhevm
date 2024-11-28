import { AppDeployment } from '../entities/app-deployment';
import { isAppDeploymentEvent } from '../entities/app-deployment.events';
import { AppDeploymentMessage } from '../entities/app-deployment.messages';
import { AppDeploymentMessagesProducer } from '../interfaces/app-deployment-messages.producer';
import { AppDeploymentRepository } from '../interfaces/app-deployment.repository';
import type { UseCase } from './use-case';

export class ProcessEventUseCase
  implements UseCase<AppDeploymentMessage, void>
{
  constructor(
    private readonly repo: AppDeploymentRepository,
    private readonly producer: AppDeploymentMessagesProducer,
  ) {}

  async execute(message: AppDeploymentMessage): Promise<void> {
    let deployment = await this.repo.findByApplicationId(
      message.payload.applicationId,
    );
    if (!deployment) {
      deployment = AppDeployment.init(message.payload.applicationId);
    }

    if (isAppDeploymentEvent(message)) {
      const messages = deployment.notify(message);
      await Promise.all(messages.map(this.producer.publish));
      await this.repo.upsert(deployment);
    }
  }
}
