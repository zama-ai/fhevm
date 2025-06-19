import type { Message } from '@aws-sdk/client-sqs'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { back } from 'messages'
import { SqsMessageHandler } from '@ssut/nestjs-sqs'
import { PUBSUB } from '#constants.js'
import { isAppError, PubSub } from 'utils'

@Injectable()
export class SQSConsumer {
  private logger = new Logger(SQSConsumer.name)

  constructor(
    @Inject(PUBSUB) private readonly pubsub: PubSub<back.BackEvent>,
  ) {}

  @SqsMessageHandler('back', false)
  public async handleMessage(message: Message) {
    this.logger.verbose(`message ${message.MessageId} received`)

    if (message.Body) {
      try {
        const data: unknown = JSON.parse(message.Body)
        if (back.isBackEvent(data)) {
          this.logger.debug(`🚀 Publishing ${data.type} to internal queue`)
          await this.pubsub.publish(data).toPromise()
        } else {
          this.logger.log(`❌ unhandled message: ${(data as any).type}`)
        }
      } catch (err) {
        this.logger.warn(
          `❌ failed to handle message: ${isAppError(err) ? err.message : err}`,
        )
        this.logger.verbose(
          `pushing { itemIdentifier: ${message.MessageId} } into batchItemFailures`,
        )
        throw err
      }
    }

    this.logger.verbose(`message ${message.MessageId} processed`)
    // Note: By returning the message, we aknowledge that the message has been processed
    // and the `sqs-consumer` library will delete it.
    return message
  }
}
