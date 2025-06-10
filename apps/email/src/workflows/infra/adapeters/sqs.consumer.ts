import { PUBSUB } from '#constants.js'
import { Message } from '@aws-sdk/client-sqs'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { SqsMessageHandler } from '@ssut/nestjs-sqs'
import { email } from 'messages'
import { IPubSub, isAppError } from 'utils'

@Injectable()
export class SQSConsumer {
  private readonly logger = new Logger(SQSConsumer.name)

  constructor(
    @Inject(PUBSUB) private readonly pubsub: IPubSub<email.EmailEvent>,
  ) {}

  @SqsMessageHandler('email', false)
  public async handleMessage(message: Message) {
    const batchItemFailures: { itemIdentifier: string | undefined }[] = []

    if (message.Body) {
      try {
        const event: unknown = JSON.parse(message.Body)
        if (email.isEmailEvent(event)) {
          this.logger.debug(
            `📬 [${event.payload.requestId}] publishing event ${event.type} on the internal queue`,
          )
          await this.pubsub.publish(event).toPromise()
        } else {
          this.logger.debug(
            `📬 [${(event as any).requestId}] received unknown event ${
              (event as any).type
            }`,
          )
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
