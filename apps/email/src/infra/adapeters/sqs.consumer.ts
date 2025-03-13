import { Message } from '@aws-sdk/client-sqs'
import { Injectable, Logger } from '@nestjs/common'
import { SqsMessageHandler } from '@ssut/nestjs-sqs'

@Injectable()
export class SQSConsumer {
  private readonly logger = new Logger(SQSConsumer.name)

  @SqsMessageHandler('email', false)
  public handleMessage(message: Message) {
    this.logger.debug(`received message: ${message.Body}`)
  }
}
