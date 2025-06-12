import { IProducer } from '#shared/services/producer.js'
import { SendMessageCommand, SQSClient } from '@aws-sdk/client-sqs'
import { Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { back } from 'messages'
import { AppError, Task, unknownError } from 'utils'

@Injectable()
export class SqsProducer implements IProducer {
  private _sqs: SQSClient | undefined
  private _queueUrl: string | undefined
  private readonly logger = new Logger(SqsProducer.name)

  constructor(private readonly config: ConfigService) {}

  private get sqs(): SQSClient {
    if (!this._sqs) {
      this._sqs = new SQSClient(this.config.get<boolean>('aws.useConfigCredentials', false)
        ? {
            endpoint: this.config.get<string>('aws.endpoint'),
            region: this.config.get<string>('aws.region'),
            credentials: {
              accessKeyId: this.config.getOrThrow<string>('aws.accessKeyId'),
              secretAccessKey: this.config.getOrThrow<string>('aws.secretAccessKey'),
            },
          }
        : {})
      this._sqs.config.useQueueUrlAsEndpoint = true
    }
    return this._sqs
  }

  private get queueUrl(): string {
    if (!this._queueUrl) {
      this._queueUrl = this.config.getOrThrow('aws.orchestrator.queueUrl')
    }
    return this._queueUrl
  }

  publish = (event: back.BackEvent): Task<void, AppError> => {
    this.logger.debug(
      `publishing: ${JSON.stringify(event)} to ${this.queueUrl}`,
    )

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
