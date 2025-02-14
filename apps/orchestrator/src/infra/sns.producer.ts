import { MS_NAME, PUBSUB } from '#constants.js'
import { PublishCommand, SNSClient } from '@aws-sdk/client-sns'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { back, web3 } from 'messages'
import { AppError, PubSub, Task, unknownError, type ISubscriber } from 'utils'

@Injectable()
export class SnsProducer {
  private readonly logger = new Logger(SnsProducer.name)
  private readonly client: SNSClient
  private readonly topicArn: string
  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: PubSub<back.BackEvent | web3.Web3Event>,
    config: ConfigService,
  ) {
    this.client = new SNSClient({
      endpoint: config.get('aws.endpoint'),
      region: config.get('aws.region'),
    })
    this.topicArn = config.getOrThrow('aws.topicArn')

    this.pubsub.subscribe('*', this.handleEvent)
  }

  readonly sendMessage = (
    message: back.BackEvent | web3.Web3Event,
  ): Task<void, AppError> => {
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
    switch (event.type) {
      case 'web3:fhe-event:requested':
      case 'back:dapp:stats-available':
        this.logger.debug(`publishing ${event.type}`)
        return this.sendMessage(event)

      default:
        this.logger.debug(`⛔️ no handler for ${event.type}`)
        return Task.of<void, AppError>(void 0).tap(() => {})
    }
  }
}
