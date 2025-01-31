import { PublishCommand, SNSClient } from '@aws-sdk/client-sns'
import { SendMessageCommand, SQSClient } from '@aws-sdk/client-sqs'
import { Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { AppDeploymentMessage } from 'messages'
import { MessageProducer } from '#domain/services/message.producer.js'
import { AppError, Task, unknownError } from 'utils'

@Injectable()
export class AwsMessageProducer implements MessageProducer {
  logger = new Logger(AwsMessageProducer.name)

  #sns: SNSClient
  #sqs: SQSClient
  #topicArn: string
  #queueUrl: string

  constructor(config: ConfigService) {
    this.logger.debug(`endpoint: ${config.get('aws.endpoint')}`)
    this.#sns = new SNSClient({
      endpoint: config.get('aws.endpoint'),
      region: config.get('aws.region'),
    })
    this.#topicArn = config.getOrThrow('aws.topicArn')

    this.#sqs = new SQSClient({
      endpoint: config.get('aws.endpoint'),
      region: config.get('aws.region'),
    })
    this.#queueUrl = config.getOrThrow('aws.queueUrl')
  }

  /**
   * Publish a message on the SQS queue with a delay.
   * It's used in case of error to retry with an exponential delay.
   * @param message - The message to publish
   */
  private sendMessage = (
    message: AppDeploymentMessage,
  ): Task<string, AppError> => {
    this.logger.debug(`sendMessage: ${JSON.stringify(message)}`)
    return new Task((resolve, reject) =>
      this.#sqs
        .send(
          new SendMessageCommand({
            QueueUrl: this.#queueUrl,
            DelaySeconds: message.meta?.delay as number | undefined,
            MessageBody: JSON.stringify(message),
          }),
        )
        .then(res => resolve(`status code: ${res.$metadata.httpStatusCode}`))
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
