import { PublishCommand, SNSClient } from '@aws-sdk/client-sns'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { AppDeploymentMessage, web3 } from 'messages'
import { MessageProducer } from '#domain/services/message.producer.js'
import { AppError, PubSub, type ISubscriber, Task, unknownError } from 'utils'
import { MS_NAME, PUBSUB } from '#constants.js'

@Injectable()
export class SnsProducer implements MessageProducer {
  logger = new Logger(SnsProducer.name)

  #sns: SNSClient
  #topicArn: string

  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: PubSub<web3.Web3Event>,
    config: ConfigService,
  ) {
    this.logger.debug(`endpoint: ${config.get('aws.endpoint')}`)
    this.#sns = new SNSClient({
      endpoint: config.get('aws.endpoint'),
      region: config.get('aws.region'),
    })
    this.#topicArn = config.getOrThrow('aws.topicArn')

    this.pubsub.subscribe('web3:*', this.handleWeb3Events)
  }

  private handleWeb3Events: ISubscriber<web3.Web3Event> = (
    event,
  ): Task<void, AppError> => {
    switch (event.type) {
      // Note: I need to improve the PubSub typing.
      // event should be an expanded of `web:*` and
      // payload should be narrowed to the right type
      case 'web3:fhe-event:detected':
        this.logger.debug(
          `🚀 publishing ${event.type} => ${event.payload.chainId}/${event.payload.address}`,
        )
        return this.sendMessage<web3.Web3Event>(event).map<void>(() => void 0)

      default:
        // Note: after improving PubSub typing,
        // the default behavior should be publishing the event
        // and I should define a case statement for all the events
        // I want to ignore
        return Task.of(void 0)
    }
  }

  /**
   * Publish a message on the SQS queue with a delay.
   * It's used in case of error to retry with an exponential delay.
   * @param message - The message to publish
   */
  private sendMessage = <
    T extends {
      type: string
      payload: any
      meta?: Record<string, string | number>
    },
  >(
    message: T,
  ): Task<string, AppError> => {
    this.logger.debug(`sendMessage: ${JSON.stringify(message)}`)
    return new Task((resolve, reject) =>
      // Note: think a better way to resend failed messages
      this.#sns
        .send(
          new PublishCommand({
            TopicArn: this.#topicArn,
            Message: JSON.stringify(message),
            MessageAttributes: {
              Sender: { DataType: 'String', StringValue: MS_NAME },
            },
          }),
        )
        .then(res => {
          this.logger.debug(
            `message ${message.type} sent to topic ${this.#topicArn}`,
          )
          resolve(`status code: ${res.$metadata.httpStatusCode}`)
        })
        .catch((err: unknown) => {
          this.logger.warn(`failed to send message: ${err}`)
          reject(unknownError(String(err)))
        }),
    )
  }

  /**
   * Publish a message on the SNS topic.
   * @param message - The message to publish
   */
  private publishCommand = (
    message: AppDeploymentMessage,
  ): Task<string, AppError> => {
    this.logger.debug(
      `publishCommand: [${this.#topicArn}] ${JSON.stringify(message)}`,
    )
    return new Task((resolve, reject) =>
      this.#sns
        .send(
          new PublishCommand({
            TopicArn: this.#topicArn,
            Message: JSON.stringify(message),
          }),
        )
        .then(result =>
          resolve(`status code: ${result.$metadata.httpStatusCode}`),
        )
        .catch((err: unknown) => {
          this.logger.warn(`failed to publish command: ${err}`)
          reject(unknownError(String(err)))
        }),
    )
  }

  private handlers: Partial<
    Record<
      AppDeploymentMessage['type'],
      (message: AppDeploymentMessage) => Task<string, AppError>
    >
  > = {
    'app-deployment.discover-sc': this.sendMessage,
    'app-deployment.sc-discovered': this.publishCommand,
    'app-deployment.sc-discovery-failed': this.publishCommand,
  }

  produce(message: AppDeploymentMessage) {
    this.logger.debug(`produce: ${message._tag}/${message.type}`)
    return (
      this.handlers[message.type]?.(message) ??
      Task.reject(unknownError('missing handler'))
    )
  }
}
