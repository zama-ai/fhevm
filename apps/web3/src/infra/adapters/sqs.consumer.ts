import { Message } from '@aws-sdk/client-sqs';
import { Injectable, Logger } from '@nestjs/common';
import { SqsMessageHandler } from 'sqs';

@Injectable()
export class SQSConsumer {
  private readonly logger = new Logger(SQSConsumer.name);

  @SqsMessageHandler('web3', false)
  public handleMessage(message: Message) {
    this.logger.debug(`received message: ${message.Body}`);
  }
}
