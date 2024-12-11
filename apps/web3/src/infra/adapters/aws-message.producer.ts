import { PublishCommand, SNSClient } from '@aws-sdk/client-sns'
import { SendMessageCommand, SQSClient } from '@aws-sdk/client-sqs'
import { Injectable } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { AppDeploymentMessage } from 'messages'
import { MessageProducer } from 'src/domain/services/message.producer'

@Injectable()
export class AwsMessageProducer implements MessageProducer {
  #sns: SNSClient
  #sqs: SQSClient
  #topicArn: string
  #queueUrl: string

  constructor(config: ConfigService) {
    this.#sns = new SNSClient({
      endpoint: config.get('aws.endpoint'),
      region: config.get('aws.region'),
    })
    this.#topicArn = config.getOrThrow('aws.topicArn')

    this.#sqs = new SQSClient({})
    this.#queueUrl = config.getOrThrow('aws.queueUrl')
  }

  /**
   * Publish a message on the SQS queue with a delay.
   * It's used in case of error to retry with an exponential delay.
   * @param message - The message to publish
   */
  private sendMessage = (message: AppDeploymentMessage) => {
    this.#sqs.send(
      new SendMessageCommand({
        QueueUrl: this.#queueUrl,
        DelaySeconds: message.$meta?.dalay as number,
        MessageBody: JSON.stringify(message),
      }),
    )
  }

  /**
   * Publish a message on the SNS topic.
   * @param message - The message to publish
   */
  private publishCommand = (message: AppDeploymentMessage): void => {
    this.#sns.send(
      new PublishCommand({
        TopicArn: this.#topicArn,
        Message: JSON.stringify(message),
        MessageGroupId: 'app-deployment',
      }),
    )
  }

  private handlers: Partial<
    Record<
      AppDeploymentMessage['type'],
      (message: AppDeploymentMessage) => void
    >
  > = {
    'app-deployment.discover-sc': this.sendMessage,
    'app-deployment.sc-discovered': this.publishCommand,
    'app-deployment.sc-discovery-failed': this.publishCommand,
  }

  produce(message: AppDeploymentMessage) {
    this.handlers[message.type]?.(message)
  }
}
