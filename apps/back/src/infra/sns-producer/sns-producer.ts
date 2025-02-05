import { MS_NAME, PUBSUB } from '#constants.js'
import { PublishCommand, SNSClient } from '@aws-sdk/client-sns'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { back } from 'messages'
import { AppError, PubSub, type ISubscriber, Task, unknownError } from 'utils'

@Injectable()
export class SnsProducer {
  private readonly sns: SNSClient
  private readonly topicArn: string
  private readonly logger = new Logger(SnsProducer.name)

  constructor(
    @Inject(PUBSUB) private readonly pubsub: PubSub<back.BackEvent>,
    config: ConfigService,
  ) {
    this.sns = new SNSClient({
      endpoint: config.get('aws.endpoint'),
      region: config.get('aws.region'),
    })
    this.topicArn = config.getOrThrow('aws.topicArn')

    this.pubsub.subscribe('back:*', this.handleBackEvent)
  }

  publish = (event: any): Task<void, AppError> => {
    this.logger.debug(`publishing: ${JSON.stringify(event)}`)

    return new Task((resolve, reject) => {
      this.sns
        .send(
          new PublishCommand({
            TopicArn: this.topicArn,
            Message: JSON.stringify(event),
            MessageAttributes: {
              Sender: { DataType: 'string', StringValue: MS_NAME },
            },
          }),
        )
        .then(result => {
          this.logger.debug(`status code: ${result.$metadata?.httpStatusCode}`)
          resolve(void 0)
        })
        .catch(error => {
          this.logger.warn(`failed to publish on ${this.topicArn}: ${error}`)
          reject(unknownError(String(error)))
        })
    })
  }

  handleBackEvent: ISubscriber<back.BackEvent> = event => {
    this.logger.debug(`handling event: ${event.type}`)
    switch (event.type as unknown as back.BackEvent['type']) {
      case 'back:dapp:stats-requested':
        return this.publish(event)

      default:
        return Task.of(void 0)
    }
  }
}
