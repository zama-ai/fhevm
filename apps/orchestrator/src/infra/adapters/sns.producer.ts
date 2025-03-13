import { MS_NAME, PUBSUB } from '#constants.js'
import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { PublishCommand, SNSClient } from '@aws-sdk/client-sns'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { back, web3 } from 'messages'
import type { IPubSub, ISubscriber } from 'utils'
import { AppError, Task, unknownError } from 'utils'

@Injectable()
export class SNSProducer implements EventProducer {
  private readonly logger = new Logger(SNSProducer.name)
  private readonly client: SNSClient
  private readonly topicArn: string
  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: IPubSub<back.BackEvent | web3.Web3Event>,
    config: ConfigService,
  ) {
    this.client = new SNSClient({
      endpoint: config.get('aws.endpoint'),
      region: config.get('aws.region'),
    })
    this.topicArn = config.getOrThrow('aws.topicArn')

    this.pubsub.subscribe('*', this.handleEvent)
  }

  readonly publish = (
    message: back.BackEvent | web3.Web3Event,
  ): Task<void, AppError> => {
    console.log(`🚀 publishing: ${message.type}`)
    this.logger.debug(`🚀 publishing: ${message.type}`)
    return new Task((resolve, reject) => {
      this.client
        .send(
          new PublishCommand({
            TopicArn: this.topicArn,
            Message: JSON.stringify(message),
            MessageAttributes: {
              Sender: { DataType: 'String', StringValue: MS_NAME },
            },
          }),
        )
        .then(result => {
          this.logger.verbose(
            `✅ PublishCommand status code: ${result.$metadata?.httpStatusCode}`,
          )
          resolve()
        })
        .catch(err => {
          this.logger.warn(
            `❌ failed to publish message: ${JSON.stringify(err)}`,
          )
          return reject(unknownError(String(err)))
        })
    })
  }

  handleEvent: ISubscriber<back.BackEvent | web3.Web3Event> = (
    event: back.BackEvent | web3.Web3Event,
  ): Task<void, AppError> => {
    if (event.meta[`${MS_NAME}-dir`] === 'in') {
      console.log(`stopping incoming event ${event.type}`)
      this.logger.verbose(`stopping incoming event ${event.type}`)
      return Task.of(void 0)
    }

    console.log(`publishing ${event.type}`)
    this.logger.debug(`publishing ${event.type}`)
    return this.publish(event)
  }
}
