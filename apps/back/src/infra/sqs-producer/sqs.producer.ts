import { MS_NAME, PUBSUB } from '#constants.js'
import { SendMessageCommand, SQSClient } from '@aws-sdk/client-sqs'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { back } from 'messages'
import { AppError, PubSub, type ISubscriber, Task, unknownError } from 'utils'

@Injectable()
export class SqsProducer {
  private readonly sqs: SQSClient
  private readonly queueUrl: string
  private readonly logger = new Logger(SqsProducer.name)

  constructor(
    @Inject(PUBSUB) private readonly pubsub: PubSub<back.BackEvent>,
    config: ConfigService,
  ) {
    this.sqs = new SQSClient({
      endpoint: config.get('aws.endpoint'),
      region: config.get('aws.region'),
      useQueueUrlAsEndpoint: true,
    })
    this.queueUrl = config.getOrThrow('aws.orchestrator.queueUrl')

    this.pubsub.subscribe('back:*', this.handleBackEvent)
  }

  publish = (event: back.BackEvent): Task<void, AppError> => {
    this.logger.debug(`publishing: ${JSON.stringify(event)}`)

    return new Task((resolve, reject) => {
      this.sqs
        .send(
          new SendMessageCommand({
            QueueUrl: this.queueUrl,
            MessageBody: JSON.stringify(event),
            MessageAttributes: {
              Sender: { DataType: 'String', StringValue: MS_NAME },
            },
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

  private handleBackEvent: ISubscriber<back.BackEvent> = event => {
    if (event.meta[`${MS_NAME}-dir`] === 'in') {
      this.logger.verbose(`stopping incoming event ${event.type}`)
      return Task.of(void 0)
    }

    this.logger.debug(`handling event: ${event.type}`)

    return this.publish(event)
  }
}
