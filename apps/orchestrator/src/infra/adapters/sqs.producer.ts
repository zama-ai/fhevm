import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { SendMessageCommand, SQSClient } from '@aws-sdk/client-sqs'
import { Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { back, MSPrefix, web3 } from 'messages'
import type { Result } from 'utils'
import { AppError, fail, ok, Task, unknownError, validationError } from 'utils'

@Injectable()
export class SQSProducer implements EventProducer {
  private readonly logger = new Logger(SQSProducer.name)
  private readonly client: SQSClient
  private readonly queueMap = new Map<MSPrefix, string>()

  constructor(config: ConfigService) {
    this.client = new SQSClient(
      config.get<boolean>('aws.useConfigCredentials', false)
      ? {
          endpoint: config.get<string>('aws.endpoint'),
          region: config.get<string>('aws.region'),
          credentials: {
            accessKeyId: config.getOrThrow<string>('aws.accessKeyId'),
            secretAccessKey: config.getOrThrow<string>('aws.secretAccessKey'),
          },
        }
      : {},
    )
    this.client.config.useQueueUrlAsEndpoint = true
    this.queueMap.set('back', config.getOrThrow('aws.back.queueUrl'))
    this.queueMap.set('email', config.getOrThrow('aws.email.queueUrl'))
    this.queueMap.set('web3', config.getOrThrow('aws.web3.queueUrl'))
    this.queueMap.set('relayer', config.getOrThrow('aws.relayer.queueUrl'))
  }

  private getQueueFromEvent = (
    event: back.BackEvent | web3.Web3Event,
  ): Result<string, AppError> => {
    const prefix = event.type.split(':')[0] as MSPrefix
    return this.queueMap.has(prefix)
      ? ok(this.queueMap.get(prefix)!)
      : fail(validationError('invalid event prefix'))
  }

  readonly publish = (
    message: back.BackEvent | web3.Web3Event,
  ): Task<void, AppError> => {
    this.logger.verbose(`handling ${message.type}`)
    const { requestId } = message.payload

    return this.getQueueFromEvent(message)
      .asyncChain(queueUrl => {
        this.logger.verbose(`queueUrl: ${queueUrl}`)

        return Task.fromPromise<void, AppError>(
          this.client
            .send(
              new SendMessageCommand({
                QueueUrl: queueUrl,
                MessageBody: JSON.stringify(message),
              }),
            )
            .then(result => {
              this.logger.verbose(
                `✅ [${requestId}] PublishCommand status code: ${result.$metadata?.httpStatusCode}`,
              )
              return void 0
            })
            .catch(err => {
              this.logger.warn(
                `❌ [${requestId}] failed to publish message to queue ${queueUrl}: ${JSON.stringify(err)}`,
              )
              throw unknownError(String(err))
            }),
        )
      })
      .tapError(err => {
        this.logger.warn(
          `❌ [${requestId}] failed to publish ${message.type}: ${err._tag}/${err.message}`,
        )
      })
  }
}
