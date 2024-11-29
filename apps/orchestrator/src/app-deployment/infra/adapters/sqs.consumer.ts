import { Injectable, Logger } from '@nestjs/common';
import { SqsMessageHandler } from '@ssut/nestjs-sqs';
import type { Message } from '@aws-sdk/client-sqs';
import { isAppDeploymentMessage } from 'src/app-deployment/entities/app-deployment.messages';
import { ProcessEventUseCase } from 'src/app-deployment/use-cases/process-event.use-case';

@Injectable()
export class SQSConsumer {
  private readonly logger = new Logger(SQSConsumer.name);
  constructor(private readonly processEvent: ProcessEventUseCase) {}

  @SqsMessageHandler('orchestrator', false)
  public async handleMessage(message: Message) {
    this.logger.debug(`received message: ${message.Body}`);
    if (message.Body) {
      const body = JSON.parse(message.Body);
      const data: unknown = JSON.parse(body.Message);
      if (isAppDeploymentMessage(data)) {
        await this.processEvent.execute(data);
      } else {
        this.logger.warn(`cannot handle message: ${message.Body}`);
      }
    }
  }
}
