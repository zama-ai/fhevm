import { Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { web3 } from 'messages'
import { Task, unknownError } from 'utils'
import { SendMessageCommand, SQSClient } from '@aws-sdk/client-sqs'

@Injectable()
export class SqsProducer {
  logger = new Logger(SqsProducer.name)

  #sns: SQSClient
  #queueUrl: string

  constructor(config: ConfigService) {
    this.logger.debug(`endpoint: ${config.get('aws.endpoint')}`)
    this.#sns = new SQSClient({
      endpoint: config.get('aws.endpoint'),
      region: config.get('aws.region'),
      useQueueUrlAsEndpoint: true,
    })
    this.#queueUrl = config.getOrThrow('aws.orchestrator.queueUrl')
  }

  /**
   * Publish a message on the SQS queue with a delay.
   * It's used in case of error to retry with an exponential delay.
   * @param message - The message to publish
   */
  sendMessage = (event: web3.Web3Event) => {
    this.logger.debug(`sendMessage: ${JSON.stringify(event)}`)

    return new Task((resolve, reject) =>
      this.#sns
        .send(
          new SendMessageCommand({
            QueueUrl: this.#queueUrl,
            MessageBody: JSON.stringify(event),
          }),
        )
        .then(res => {
          this.logger.debug(
            `message ${event.type} sent to queue ${this.#queueUrl} [${res.$metadata?.httpStatusCode}]`,
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
