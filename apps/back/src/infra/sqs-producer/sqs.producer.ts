import { IProducer } from '#shared/services/producer.js'
import { SendMessageCommand, SQSClient } from '@aws-sdk/client-sqs'
import { Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { back } from 'messages'
import { AppError, Task, unknownError } from 'utils'

@Injectable()
export class SqsProducer implements IProducer {
  private readonly sqs: SQSClient
  private readonly queueUrl: string
  private readonly logger = new Logger(SqsProducer.name)

  constructor(config: ConfigService) {
    this.sqs = new SQSClient({
      endpoint: config.get('aws.endpoint'),
      region: config.get('aws.region'),
      useQueueUrlAsEndpoint: true,
    })
    this.queueUrl = config.getOrThrow('aws.orchestrator.queueUrl')
  }

  publish = (event: back.BackEvent): Task<void, AppError> => {
    this.logger.debug(`publishing: ${JSON.stringify(event)} to ${this.queueUrl}`)

    return new Task((resolve, reject) => {
      this.sqs
        .send(
          new SendMessageCommand({
            QueueUrl: this.queueUrl,
            MessageBody: JSON.stringify(event),
          }),
        )
        .then(result => {
          this.logger.debug(`status code: ${result.$metadata?.httpStatusCode}`)
          resolve(void 0)
        })
        .catch(error => {
          this.logger.warn(`failed to publish on ${this.queueUrl}: ${error}`)
          reject(unknownError(String(error)))
        })
    })
  }
}
