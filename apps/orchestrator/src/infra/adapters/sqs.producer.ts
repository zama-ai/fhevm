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
    this.client = new SQSClient({
      endpoint: config.get('aws.endpoint'),
      region: config.get('aws.region'),
      useQueueUrlAsEndpoint: true,
    })
    this.queueMap.set('back', config.getOrThrow('aws.back.queueUrl'))
    this.queueMap.set('web3', config.getOrThrow('aws.web3.queueUrl'))
    this.queueMap.set('relayer', config.getOrThrow('aws.relayer.queueUrl'))
  }

  private getQueueFromEvent = (
    event: back.BackEvent | web3.Web3Event,
  ): Result<string, AppError> => {
    const prefix = event.type.split(':')[0] as MSPrefix
    console.log(`queueMap has ${prefix}? ${this.queueMap.has(prefix)}`)
    return this.queueMap.has(prefix)
      ? ok(this.queueMap.get(prefix)!)
      : fail(validationError('invalid event prefix'))
  }

  readonly publish = (
    message: back.BackEvent | web3.Web3Event,
  ): Task<void, AppError> => {
    console.log(`🚀 publishing: ${message.type}`)
    this.logger.debug(`🚀 publishing: ${message.type}`)

    return this.getQueueFromEvent(message).asyncChain(queueUrl => {
      this.logger.verbose(`queueUrl: ${queueUrl}`)
      return new Task((resolve, reject) => {
        this.client
          .send(
            new SendMessageCommand({
              QueueUrl: queueUrl,
              MessageBody: JSON.stringify(message),
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
              `❌ failed to publish message to queue ${queueUrl}: ${JSON.stringify(err)}`,
            )
            return reject(unknownError(String(err)))
          })
      })
    })
  }
}
