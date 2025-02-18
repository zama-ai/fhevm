import { PublishCommand, SNSClient } from '@aws-sdk/client-sns'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { web3 } from 'messages'
import type { IPubSub, ISubscriber } from 'utils'
import { Task, unknownError } from 'utils'
import { MS_NAME, PUBSUB } from '#constants.js'

@Injectable()
export class SnsProducer {
  logger = new Logger(SnsProducer.name)

  #sns: SNSClient
  #topicArn: string

  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: IPubSub<web3.Web3Event>,
    config: ConfigService,
  ) {
    this.logger.debug(`endpoint: ${config.get('aws.endpoint')}`)
    this.#sns = new SNSClient({
      endpoint: config.get('aws.endpoint'),
      region: config.get('aws.region'),
    })
    this.#topicArn = config.getOrThrow('aws.topicArn')

    this.pubsub.subscribe('web3:*', this.sendMessage)
  }

  /**
   * Publish a message on the SQS queue with a delay.
   * It's used in case of error to retry with an exponential delay.
   * @param message - The message to publish
   */
  sendMessage: ISubscriber<web3.Web3Event> = event => {
    this.logger.debug(`sendMessage: ${JSON.stringify(event)}`)
    return new Task((resolve, reject) =>
      // Note: think a better way to resend failed messages
      this.#sns
        .send(
          new PublishCommand({
            TopicArn: this.#topicArn,
            Message: JSON.stringify(event),
            MessageAttributes: {
              Sender: { DataType: 'String', StringValue: MS_NAME },
            },
          }),
        )
        .then(res => {
          this.logger.debug(
            `message ${event.type} sent to topic ${this.#topicArn} [${res.$metadata?.httpStatusCode}]`,
          )
          resolve(void 0)
        })
        .catch((err: unknown) => {
          this.logger.warn(`failed to send message: ${err}`)
          reject(unknownError(String(err)))
        }),
    )
  }
}
