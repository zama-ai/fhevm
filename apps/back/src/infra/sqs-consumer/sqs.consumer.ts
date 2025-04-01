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
    const batchItemFailures: { itemIdentifier: string | undefined }[] = []

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
        batchItemFailures.push({ itemIdentifier: message.MessageId })
      }
    }

    return { batchItemFailures }
  }
}
